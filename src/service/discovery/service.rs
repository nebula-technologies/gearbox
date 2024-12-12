use crate::net::socket_addr::SocketAddr;
use crate::rails::ext::blocking::TapResult;
use crate::service::discovery::advertisement::AdvertisementTransformer;
use crate::service::discovery::discovery_type::broadcaster::Broadcaster;
use crate::service::discovery::discovery_type::discoverer::Discoverer;
use crate::service::discovery::discovery_type::ServiceDiscoveryType;
use crate::{error, info, trace};
use bytes::Bytes;
use std::future::Future;
use std::net::SocketAddr as StdSocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::sleep;

pub struct Service<S, A>
where
    A: 'static + Send + Sync + Clone,
    S: 'static + Send + Sync,

    Broadcaster<A>: AdvertisementTransformer<A>,
{
    pub managed: bool,
    pub bind: SocketAddr,
    pub service_types: Vec<ServiceDiscoveryType<S, A>>,
    pub discovery_functions: Vec<
        Arc<dyn Fn(&Arc<Bytes>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>,
    >,
}

impl<S, A> Service<S, A>
where
    A: 'static + Send + Sync + Clone,
    S: 'static + Send + Sync,

    Broadcaster<A>: AdvertisementTransformer<A>,
{
    pub fn new<T: Into<SocketAddr>>(bind: T, managed: bool) -> Self {
        Service {
            managed,
            bind: bind.into(),
            service_types: Vec::new(),
            discovery_functions: Vec::new(),
        }
    }
    pub fn add_broadcaster(&mut self, broadcaster: Broadcaster<A>) -> &mut Self {
        trace!("Adding Broadcaster: {:?}", &broadcaster);
        self.service_types
            .push(ServiceDiscoveryType::Broadcaster(broadcaster));
        self
    }

    pub fn add_discoverer(&mut self, discoverer: Discoverer<S, A>) -> &mut Self {
        info!("Adding Discoverer: {:?}", &discoverer);
        self.service_types
            .push(ServiceDiscoveryType::Discoverer(discoverer));
        self
    }

    pub fn add_discovery_function<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&Arc<Bytes>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        self.discovery_functions
            .push(Arc::new(move |bytes| Box::pin(f(bytes))));
        self
    }

    pub fn get_bind(&mut self) -> SocketAddr {
        self.bind.clone()
    }

    pub async fn serve(mut self, state: Option<S>) -> Result<(), String> {
        let bind: StdSocketAddr = self.get_bind().into();

        info!("Binding to: {:?}", bind);
        let socket = UdpSocket::bind(bind)
            .await
            .and_then(|t| t.set_broadcast(true).map(|_| t))
            .map(Arc::new)
            .map_err(|e| format!("Failed to bind socket ({:?}): {}", bind, e))
            .tap_err(|e| error!("{}", e))?;

        // Clone the socket to use it for both sending and receiving
        let socket_send = socket.clone();
        let arc_self = Arc::new(self);

        // Start sending broadcasts in a separate task
        let send_task = tokio::spawn({
            info!("Starting broadcast task...");
            let thread_socket = socket_send.clone();
            let arc_self = Arc::clone(&arc_self);
            async move {
                let mut tick = 0u64;
                info!("Broadcasting loop starting...");
                loop {
                    for service_type in arc_self.service_types.iter() {
                        if let ServiceDiscoveryType::Broadcaster(broadcaster) = service_type {
                            if tick % broadcaster.interval.unwrap_or(120) == 0 {
                                if let (data, Some(broadcast)) =
                                    (broadcaster.advert_into_bytes(), &broadcaster.broadcast)
                                {
                                    thread_socket
                                        .send_to(&*data, StdSocketAddr::from(broadcast))
                                        .await
                                        .tap(|t| {
                                            info!(
                                                "Broadcast sent {} bytes to {}: {:?}",
                                                t, broadcast, data
                                            );
                                        })
                                        .tap_err(|e| error!("Broadcast error: {}", e))
                                        .ok();
                                }
                            }
                        }
                    }
                    // Wait for the next broadcast interval
                    sleep(Duration::from_secs(1)).await;
                    if tick == u64::MAX {
                        tick = 0;
                    } else {
                        tick += 1;
                    }
                }
            }
        });

        // Start receiving broadcasts
        let mut buf = [0; 1024];
        loop {
            info!("Waiting for data...");
            match socket.recv_from(&mut buf).await {
                Ok((amt, src)) => {
                    let advert = Bytes::from(buf[..amt].to_vec());
                    let arc_advert = Arc::new(advert);

                    info!("Received {} bytes from {}: {:?}", amt, src, &arc_advert);

                    // Trigger discovery functions
                    for func in arc_self.clone().discovery_functions.iter() {
                        func(&arc_advert).await;
                    }

                    // Process discoverers
                    for service_type in arc_self.service_types.iter() {
                        if let ServiceDiscoveryType::Discoverer(discoverer) = service_type {
                            if let Some(validator) = &discoverer.validator {
                                if validator(&arc_advert).is_ok() {
                                    for processor in &discoverer.processor {
                                        processor(arc_advert.clone(), state.as_ref()).await;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving data: {}", e);
                }
            }
        }

        // Ensure that the send task is aborted when the serve function exits
        //send_task.abort();
    }
}

impl<A: Clone + Send + Sync, S: Clone + Send + Sync> Clone for Service<S, A>
where
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    fn clone(&self) -> Self {
        Service {
            managed: self.managed,
            bind: self.bind.clone(),
            service_types: self.service_types.clone(),
            discovery_functions: self.discovery_functions.clone(),
        }
    }
}
