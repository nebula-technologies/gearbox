use super::entity::Advertisement;
use crate::collections::const_hash_map::HashMap;
use crate::net::ip_range::IpRanges;
use crate::net::socket_addr::{SocketAddr, SocketAddrs};
use crate::prelude::spin::RwLock;
use crate::rails::ext::blocking::TapResult;
use crate::service::discovery::advertisement::AdvertisementTransformer;
use crate::service::discovery::discovery_type::broadcaster::Broadcaster;
use crate::service::discovery::discovery_type::discoverer::Discoverer;
use crate::service::discovery::service::Service;
use crate::service::discovery::service_binding::ServiceBinding;
use crate::{debug, error, info, trace};
use bytes::Bytes;
use core::fmt::{Debug, Formatter};
use semver::Version;
use std::any::Any;
use std::any::TypeId;
use std::future::Future;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, SocketAddr as StdSocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use std::u64;
use tokio::net::UdpSocket;
use tokio::time::sleep;

pub static COMMON_SERVICE_DISCOVERY_STATE: RwLock<
    HashMap<ServiceBinding, Service<ServiceDiscoveryState, Bytes>>,
> = RwLock::new(HashMap::new());

#[derive(Clone)]
/// Service Discovery
/// This is the manager that handles the 2 parts of a service discovery system:
/// It also allows for management of the services, if they need to be handled by the Discovery Lock,
/// the discovery lock is used to ensure that there is no overlap in the port used by the system, eg.
/// ig you have a system that both listens for new service broadcasts and also is broadcasting services,
/// this will allow us to pick up the specific service and post multiple and recieving multiple using
/// existing broadcast and discovery systems.
pub struct ServiceDiscovery<S, A, M>
where
    A: 'static + Send + Sync + Clone,
    S: 'static + Send + Sync + Clone,
    M: 'static + Send + Sync + ServiceManagerTrait<S, A>,

    Broadcaster<A>: AdvertisementTransformer<A>,
{
    phantom_data: PhantomData<(S, A)>,
    managed_state: Option<Arc<M>>,
}

impl<S, A, M> ServiceDiscovery<S, A, M>
where
    A: Send + Sync + Clone,
    S: Send + Sync + Clone,
    M: 'static + Send + Sync + ServiceManagerTrait<S, A>,

    Broadcaster<A>: AdvertisementTransformer<A>,
{
    pub fn unmanaged() -> Self {
        ServiceDiscovery {
            phantom_data: Default::default(),
            managed_state: None,
        }
    }

    pub fn managed(state_ref: Arc<M>) -> Self {
        ServiceDiscovery {
            phantom_data: Default::default(),
            managed_state: Some(state_ref),
        }
    }

    pub fn is_managed(&self) -> bool {
        self.managed_state.is_some()
    }

    pub fn add<T: ServiceDiscoveryTrait<S, A>>(mut self) -> Self {
        let broadcasters = T::broadcasters();
        let discoverers = T::discoverers();
        let discovery_catchall = T::discovery_catchall();
        let port = T::port();
        let ip = T::ip();

        for broadcaster in broadcasters {
            self.add_broadcaster((&ip, &port), broadcaster);
        }
        for discoverer in discoverers {
            self.add_discoverer((&ip, &port), discoverer);
        }

        for catchall in discovery_catchall {
            self.add_discovery_function((&ip, &port), catchall);
        }

        self
    }

    pub fn add_broadcaster<T: Into<SocketAddrs>>(
        &mut self,
        binds: T,
        broadcaster: Broadcaster<A>,
    ) -> &mut Self {
        let binds = binds.into();
        for bind in binds {
            self.managed_state.as_ref().map(|t| {
                let key = (bind.ip_with_defaults(), bind.port_with_defaults()).into();
                let mut v = t.get_or_insert_with(Clone::clone(&key), || Service {
                    managed: true,
                    bind: bind,
                    service_types: Vec::new(),
                    discovery_functions: vec![],
                });
                info!(
                    "Adding Broadcaster: {:?} bound to {:?}",
                    &broadcaster.id, key
                );
                v.add_broadcaster(broadcaster.clone());
                t.insert(key, v);
            });
        }
        self
    }

    pub fn add_discoverer<T: Into<SocketAddrs>>(
        &mut self,
        binds: T,
        discoverer: Discoverer<S, A>,
    ) -> &mut Self {
        let binds = binds.into();

        for bind in binds {
            self.managed_state.as_ref().map(|t| {
                let key = (bind.ip_with_defaults(), bind.port_with_defaults()).into();
                let mut v = t.get_or_insert_with(Clone::clone(&key), || Service {
                    managed: true,
                    bind: bind,
                    service_types: Vec::new(),
                    discovery_functions: vec![],
                });
                v.add_discoverer(discoverer.clone());
                t.insert(key, v);
            });
        }

        self
    }

    pub fn add_discovery_function<T, O, Fut>(&mut self, bind: T, f: O) -> &mut Self
    where
        O: Fn(&Arc<Bytes>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
        T: Into<SocketAddr>,
    {
        let bind = bind.into();
        self.managed_state.as_ref().map(|t| {
            let v = t
                .get_or_insert_with((&bind).into(), || Service {
                    managed: true,
                    bind,
                    service_types: Vec::new(),
                    discovery_functions: vec![],
                })
                .add_discovery_function(f);
        });

        self
    }

    pub fn serve(self, state: Option<S>) -> () {
        info!("Starting service discovery...");
        let services = if let Some(managed_state) = self.managed_state {
            info!("Service discovery system is managed...");
            managed_state.as_owned_services()
        } else {
            HashMap::new()
        };

        for (t, service) in services.into_iter() {
            info!("Spinning Service Binding set: {}", t);
            let state_cloned = state.clone();
            tokio::spawn(async move {
                service.serve(state_cloned).await.ok();
            });
        }
    }
}

pub trait ServiceDiscoveryTrait<S, A: Clone> {
    fn broadcasters() -> Vec<Broadcaster<A>> {
        Vec::new()
    }
    fn discoverers() -> Vec<Discoverer<S, A>> {
        Vec::new()
    }
    fn discovery_catchall() -> Vec<
        Box<dyn Fn(&Arc<Bytes>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>,
    > {
        Vec::new()
    }
    fn port() -> usize {
        9999
    }
    fn ip() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
    }
    fn managed() -> bool {
        true
    }
}

pub trait ServiceDiscoveryStateTrait {
    fn set_state<T: Sync + Send + Any + Copy>(&mut self, key: &str, state: T);
    fn state<T: Sync + Send + Any + Copy>(&self, key: &str) -> Option<T>;
    fn list<T: Sync + Send + Any + Copy>(&self) -> Vec<T>;
}

impl ServiceDiscoveryStateTrait for () {
    fn set_state<T: Sync + Send + Any + Copy>(&mut self, _key: &str, _state: T) {}
    fn state<T: Sync + Send + Any + Copy>(&self, _key: &str) -> Option<T> {
        None
    }
    fn list<T: Sync + Send + Any + Copy>(&self) -> Vec<T> {
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub struct ServiceDiscoveryState {
    state: Arc<RwLock<HashMap<std::any::TypeId, HashMap<String, Box<dyn Any + Send + Sync>>>>>,
}

impl ServiceDiscoveryState {
    pub fn new() -> Self {
        ServiceDiscoveryState {
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

unsafe impl Send for ServiceDiscoveryState {}
unsafe impl Sync for ServiceDiscoveryState {}

impl Default for ServiceDiscoveryState {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceDiscoveryStateTrait for ServiceDiscoveryState {
    fn set_state<T: Sync + Send + Any + Copy>(&mut self, key: &str, state: T) {
        let type_id = TypeId::of::<T>();
        let state = Box::new(state);
        let mut state_rw = self.state.write();
        state_rw
            .entry(type_id)
            .or_insert(HashMap::new())
            .insert(key.to_string(), state);
    }

    fn state<T: Sync + Send + Any + Copy>(&self, key: &str) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let state_r = self.state.read();
        state_r
            .get(&type_id)
            .and_then(|t| t.get(key))
            .and_then(|t| t.downcast_ref::<T>())
            .map(|t| t.to_owned())
    }

    fn list<T: Sync + Send + Any + Copy>(&self) -> Vec<T> {
        let type_id = TypeId::of::<T>();
        let state_r = self.state.read();
        state_r
            .get(&type_id)
            .map(|t| {
                t.values()
                    .filter_map(|t| t.downcast_ref::<T>())
                    .map(|t| t.to_owned())
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub trait ServiceManagerTrait<S, A>
where
    A: Clone + Send + Sync,
    S: Clone + Send + Sync,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    fn insert(&self, k: ServiceBinding, service: Service<S, A>) -> Option<Service<S, A>>;
    fn remove(&self, k: ServiceBinding) -> Option<Service<S, A>>;

    fn get_or_insert_with<F>(&self, k: ServiceBinding, default: F) -> Service<S, A>
    where
        F: FnOnce() -> Service<S, A>;

    fn as_owned_services(&self) -> HashMap<ServiceBinding, Service<S, A>>;
}

pub struct ServiceManagerContainerArc<S, A>(RwLock<HashMap<Arc<ServiceBinding>, Service<S, A>>>)
where
    A: 'static + Send + Sync + Clone,
    S: 'static + Send + Sync,

    Broadcaster<A>: AdvertisementTransformer<A>;

impl<S, A> Default for ServiceManagerContainerArc<S, A>
where
    A: 'static + Send + Sync + Clone,
    S: 'static + Send + Sync,

    Broadcaster<A>: AdvertisementTransformer<A>,
{
    fn default() -> Self {
        ServiceManagerContainerArc(RwLock::new(HashMap::new()))
    }
}

impl<S, A> ServiceManagerTrait<S, A> for ServiceManagerContainerArc<S, A>
where
    A: Clone + Send + Sync,
    S: Clone + Send + Sync,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    fn insert(&self, k: ServiceBinding, service: Service<S, A>) -> Option<Service<S, A>> {
        self.0.write().insert(Arc::from(k), service)
    }
    fn remove(&self, k: ServiceBinding) -> Option<Service<S, A>> {
        self.0.write().remove(&k)
    }

    fn get_or_insert_with<F>(&self, k: ServiceBinding, default: F) -> Service<S, A>
    where
        F: FnOnce() -> Service<S, A>,
    {
        self.0
            .write()
            .entry(Arc::from(k))
            .or_insert_with(default)
            .clone()
    }

    fn as_owned_services(&self) -> HashMap<ServiceBinding, Service<S, A>> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceManagerTrait;
    use super::*;
    use crate::service::discovery::service_discovery;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr};
    use std::ops::Deref;
    use tokio::sync::oneshot;
    use tokio::time::timeout;

    type DiscoveryService = ServiceDiscovery<
        ServiceDiscoveryState,
        Bytes,
        service_discovery::ServiceManagerContainerArc<
            std::sync::Arc<service_discovery::ServiceDiscoveryState>,
            bytes::Bytes,
        >,
    >;

    static SERVICE_DISCOVERY: RwLock<
        HashMap<ServiceBinding, Service<ServiceDiscoveryState, Bytes>>,
    > = RwLock::new(HashMap::new());

    #[tokio::test]
    async fn test_service_binding_constructors() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 8080;
        let binding = ServiceBinding::new(port, ip);

        assert_eq!(binding.port(), port);
        assert_eq!(binding.ip(), ip);

        let from_tuple: ServiceBinding = (ip, port).into();
        assert_eq!(from_tuple, binding);

        let from_ref_tuple: ServiceBinding = (&ip, &port).into();
        assert_eq!(from_ref_tuple, binding);

        let from_port: ServiceBinding = port.into();
        assert_eq!(from_port.port(), port);

        let from_ref_port: ServiceBinding = (&port).into();
        assert_eq!(from_ref_port.port(), port);

        let from_option: ServiceBinding = (Some(ip), port).into();
        assert_eq!(from_option, binding);

        let from_ref_option: ServiceBinding = (&Some(ip), &port).into();
        assert_eq!(from_ref_option, binding);
    }

    #[tokio::test]
    async fn test_service_discovery_constructors() {
        let state_ref = &SERVICE_DISCOVERY;

        let discovery_manager =
            ServiceManagerContainerArc::<Arc<ServiceDiscoveryState>, Bytes>::default();

        let unmanaged = ServiceDiscovery::unmanaged();
        assert!(!unmanaged.is_managed());

        let managed = ServiceDiscovery::managed(Arc::new(discovery_manager));
        assert!(managed.is_managed());
    }

    #[tokio::test]
    async fn test_managed_service_discovery() {
        let state_ref = &SERVICE_DISCOVERY;
        let discovery_manager =
            ServiceManagerContainerArc::<Arc<ServiceDiscoveryState>, Bytes>::default();
        let discovery = ServiceDiscovery::managed(Arc::new(discovery_manager));

        assert!(discovery.managed_state.is_some());
    }

    #[tokio::test]
    async fn test_service_add_broadcaster() {
        let mut discovery: DiscoveryService = ServiceDiscovery::unmanaged();
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port: u16 = 8080;
        let broadcaster = Broadcaster::new().with_service_name(Some("Test Service".to_string()));

        discovery.add_broadcaster((ip, port), broadcaster);
        assert_eq!(discovery.discovery_type.len(), 1);
    }

    #[tokio::test]
    async fn test_service_add_discoverer() {
        let mut discovery: DiscoveryService = ServiceDiscovery::unmanaged();
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 8080u16;
        let discoverer = Discoverer::new();

        discovery.add_discoverer((ip, port), discoverer);
        assert_eq!(discovery.discovery_type.len(), 1);
    }

    #[tokio::test]
    async fn test_service_broadcaster_methods() {
        let broadcaster = Broadcaster::new()
            .with_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 100, 10))))
            .with_port(Some(9999))
            .bcast_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 100, 255)))
            .bcast_port(9990)
            .with_service_name(Some("Test Service".to_string()))
            .with_version(Some("1.0.0".to_string()))
            .with_interval(Some(10))
            .with_advertisement(Some(Bytes::from("Test Broadcast")));

        assert_eq!(
            broadcaster.ip,
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 100, 10)))
        );
        assert_eq!(
            broadcaster.broadcast,
            Some((IpAddr::V4(Ipv4Addr::new(192, 168, 100, 255)), 9990).into())
        );
        assert_eq!(broadcaster.service_name, Some("Test Service".to_string()));
        assert_eq!(broadcaster.version, Some("1.0.0".to_string()));
        assert_eq!(broadcaster.interval, Some(10));
    }

    #[tokio::test]
    async fn test_discoverer_constructors() {
        let discoverer = Discoverer::<ServiceDiscoveryState, Bytes>::new();
        assert_eq!(discoverer.service_name, None);
        assert_eq!(discoverer.version, None);
        assert_eq!(discoverer.interval, None);
        assert_eq!(discoverer.advert, None);
    }

    #[tokio::test]
    async fn test_discoverer_set_validator() {
        let mut discoverer = Discoverer::<ServiceDiscoveryState, Bytes>::new();
        let validator = |advert: &Arc<Bytes>| -> Result<(), DiscovererError> {
            if advert.is_empty() {
                Err(DiscovererError::NoDataAvailable)
            } else {
                Ok(())
            }
        };

        discoverer.set_validator(validator);
        assert!(discoverer.validator.is_some());
    }

    #[tokio::test]
    async fn test_discoverer_add_processor() {
        let mut discoverer = Discoverer::<ServiceDiscoveryState, Bytes>::new();

        discoverer.add_processor(|advert, state| {
            Box::pin(async move {
                info!("Processing advert: {:?}", advert);
            })
        });
        assert_eq!(discoverer.processor.len(), 1);
    }

    #[tokio::test]
    async fn test_service_discovery_full_cycle() {
        let mut service: Service<ServiceDiscoveryState, Bytes> =
            Service::new((IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999), true);
        let broadcaster =
            Broadcaster::new().with_advertisement(Some(Bytes::from("Test Broadcast")));
        let mut discoverer = Discoverer::new();
        discoverer.set_validator(|_advert| Ok(()));

        service.add_broadcaster(broadcaster);
        service.add_discoverer(discoverer);

        assert_eq!(service.service_types.len(), 2);
    }

    #[tokio::test]
    async fn test_discoverer_advert_capture() {
        let discoverer = Discoverer::<ServiceDiscoveryState, Bytes>::new();
        let advert = Advertisement {
            service_id: Some("Test Service".into()),
            version: Some("1.0.0".into()),
            ..Default::default()
        };
        let b_advert = Bytes::from(advert.clone());

        let captured_advert = discoverer.advert_capture(b_advert).unwrap();
        assert_eq!(captured_advert, advert);
    }

    #[tokio::test]
    async fn test_advert_conversion() {
        let discoverer = Discoverer::<ServiceDiscoveryState, Bytes>::new();
        let advert = Advertisement {
            service_id: Some("Test Service".into()),
            version: Some("1.0.0".into()),
            ..Default::default()
        };
        let advert_string = String::from(advert);
        assert_eq!(";;;;;;Test Service;;;;;;1.0.0".to_string(), advert_string);
    }

    #[tokio::test]
    async fn test_service_broadcast_and_discovery() {
        // Setup a broadcaster with a test advertisement
        let advertisement = Advertisement {
            service_id: Some("Test".to_string()),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        };

        let broadcaster = Broadcaster::new()
            .with_interval(Some(1))
            .with_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 10, 100))))
            .with_port(Some(9999))
            .with_broadcast_mask(Some(IpAddr::V4(Ipv4Addr::new(255, 255, 254, 0))))
            .with_advertisement(Some(Bytes::from(advertisement.clone())));

        info!("Broadcasting: {:?}", broadcaster);

        // Setup a discoverer with a simple validator and processor
        let mut discoverer = Discoverer::<ServiceDiscoveryState, Bytes>::new();
        discoverer.set_validator(move |advert| {
            if advert.deref() == &Bytes::from(advertisement.clone()) {
                Ok(())
            } else {
                Err(DiscovererError::NoDataAvailable)
            }
        });
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let arc_tx = Arc::new(tx);

        discoverer.add_processor(move |advert, state| {
            let arc_tx = Arc::clone(&arc_tx);
            let state_clone = state.map(|t| t.to_owned()).clone();
            Box::pin(async move {
                info!("Call processing function: {:?}", advert);
                if !arc_tx.is_closed() {
                    info!("sending message");
                    let _ = arc_tx.send(()).await;
                }
            })
        });

        // Create a service with the broadcaster and discoverer
        let mut service = Service::new((IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9999), true);
        service.add_broadcaster(broadcaster);
        service.add_discoverer(discoverer);

        let discovery_state = ServiceDiscoveryState::new();
        // Run the service in a separate task
        let service_handle = tokio::spawn(async move {
            service
                .serve(Some(discovery_state))
                .await
                .expect("Service failed");
        });

        // Set a timeout to avoid blocking indefinitely
        let result = timeout(Duration::from_secs(5), rx.recv()).await;

        // Check if the discoverer received the broadcast
        match result {
            Ok(_) => {
                info!("Broadcast received successfully, shutting down...");
            }
            Err(_) => {
                panic!("Timeout: Did not receive broadcast in time");
            }
        }

        // Shut down the service
        service_handle.abort();
        assert!(
            service_handle.await.is_err(),
            "Service did not shut down as expected"
        );
    }
}
