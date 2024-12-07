use super::entity::Advertisement;
use crate::collections::const_hash_map::HashMap;
use crate::net::ip_range::IpRanges;
use crate::net::socket_addr::{SocketAddr, SocketAddrs};
use crate::prelude::spin::RwLock;
use crate::rails::ext::blocking::TapResult;
use crate::service::discovery::service_binding::ServiceBinding;
use crate::sync::deferred::Deferred;
use crate::{debug, error};
use bytes::Bytes;
use core::fmt::{Debug, Formatter};
use semver::Version;
use spin::RwLockWriteGuard;
use std::any::Any;
use std::any::TypeId;
use std::future::Future;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, SocketAddr as StdSocketAddr, ToSocketAddrs};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use std::u64;
use tokio::net::UdpSocket;
use tokio::time::sleep;

pub static COMMON_SERVICE_DISCOVERY_STATE: RwLock<
    HashMap<ServiceBinding, Service<ServiceDiscoveryState, Bytes>>,
> = RwLock::new(HashMap::new());

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

    pub fn managed<IM: Into<Arc<M>>>(state_ref: IM) -> Self {
        ServiceDiscovery {
            phantom_data: Default::default(),
            managed_state: Some(state_ref.into()),
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
                t.get_or_insert_with(
                    (bind.ip_with_defaults(), bind.port_with_defaults()).into(),
                    || Service {
                        managed: true,
                        bind: bind,
                        service_types: Vec::new(),
                        discovery_functions: vec![],
                    },
                )
                .add_broadcaster(broadcaster.clone())
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
                t.get_or_insert_with(
                    (bind.ip_with_defaults(), bind.port_with_defaults()).into(),
                    || Service {
                        managed: true,
                        bind: bind,
                        service_types: Vec::new(),
                        discovery_functions: vec![],
                    },
                )
                .add_discoverer(discoverer.clone())
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
            t.get_or_insert_with((&bind).into(), || Service {
                managed: true,
                bind,
                service_types: Vec::new(),
                discovery_functions: vec![],
            })
            .add_discovery_function(f)
        });

        self
    }

    pub fn serve(mut self, state: Option<S>) -> () {
        debug!("Starting service discovery...");
        let services = if let Some(managed_state) = self.managed_state {
            debug!("Service discovery system is managed...");
            managed_state.as_owned_services()
        } else {
            HashMap::new()
        };

        for (t, service) in services.into_iter() {
            debug!("Spinning Service Binding set: {}", t);
            let state_cloned = state.clone();
            tokio::spawn(async move {
                service.serve(state_cloned).await.ok();
            });
        }
    }
}

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
        debug!("Adding Broadcaster: {:?}", &broadcaster);
        self.service_types
            .push(ServiceDiscoveryType::Broadcaster(broadcaster));
        self
    }

    pub fn add_discoverer(&mut self, discoverer: Discoverer<S, A>) -> &mut Self {
        debug!("Adding Discoverer: {:?}", &discoverer);
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

        debug!("Binding to: {:?}", bind);
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
            debug!("Starting broadcast task...");
            let thread_socket = socket_send.clone();
            let arc_self = Arc::clone(&arc_self);
            async move {
                let mut tick = 0u64;
                debug!("Broadcasting loop starting...");
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
                                            debug!(
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
            debug!("Waiting for data...");
            match socket.recv_from(&mut buf).await {
                Ok((amt, src)) => {
                    let advert = Bytes::from(buf[..amt].to_vec());
                    let arc_advert = Arc::new(advert);

                    debug!("Received {} bytes from {}: {:?}", amt, src, &arc_advert);

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
pub enum ServiceDiscoveryType<S, A = Bytes>
where
    A: Clone,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    Broadcaster(Broadcaster<A>),
    Discoverer(Discoverer<S, A>),
}

impl<S, A> ServiceDiscoveryType<S, A>
where
    A: Clone,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    pub fn is_broadcaster(&self) -> bool {
        match self {
            ServiceDiscoveryType::Broadcaster(_) => true,
            _ => false,
        }
    }

    pub fn is_discoverer(&self) -> bool {
        match self {
            ServiceDiscoveryType::Discoverer(_) => true,
            _ => false,
        }
    }
    pub fn broadcaster() -> Broadcaster<A> {
        Broadcaster::new()
    }

    pub fn discoverer() -> Discoverer<S, A> {
        Discoverer::new()
    }
}

#[derive(Clone)]
pub struct Broadcaster<A>
where
    A: Clone,
{
    ip: Option<IpAddr>,
    port: Option<u16>,
    broadcast: Option<SocketAddr>,
    service_name: Option<String>,
    version: Option<String>,
    interval: Option<u64>,
    advertisement: Option<A>,
}

impl<A> Broadcaster<A>
where
    A: Clone,
{
    pub fn new() -> Self {
        Broadcaster {
            ip: None,
            port: None,
            broadcast: SocketAddr::default_addr().as_broadcast_addr(None).ok(),
            service_name: None,
            version: None,
            interval: Some(5),
            advertisement: Default::default(),
        }
    }
    pub fn with_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.ip = ip;
        self
    }
    pub fn ip_mut(&mut self) -> &mut Option<IpAddr> {
        &mut self.ip
    }
    pub fn with_port(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }
    pub fn port_mut(&mut self) -> &mut Option<u16> {
        &mut self.port
    }

    pub fn bcast_port(mut self, port: u16) -> Self {
        self.broadcast = self
            .broadcast
            .map(|mut t| {
                t.set_port(port);
                t
            })
            .or_else(|| {
                Some(
                    SocketAddr::default_addr()
                        .as_broadcast_addr(None)
                        .unwrap()
                        .set_port(port)
                        .to_owned(),
                )
            });
        self
    }
    pub fn bcast_ip(mut self, ip: IpAddr) -> Self {
        self.broadcast = self
            .broadcast
            .map(|mut t| {
                t.set_ip(ip);
                t
            })
            .or_else(|| {
                Some(
                    SocketAddr::default_addr()
                        .as_broadcast_addr(None)
                        .unwrap()
                        .set_ip(ip)
                        .to_owned(),
                )
            });
        self
    }

    pub fn with_broadcast_mask(mut self, mask: Option<IpAddr>) -> Self {
        if let (Some(ip), Some(port)) = (&self.ip, &self.port) {
            self.broadcast = SocketAddr::new(*ip, *port).as_broadcast_addr(mask).ok();
        }
        self
    }
    pub fn set_broadcast_mask(&mut self, mask: IpAddr) -> &mut Self {
        if let (Some(ip), Some(port)) = (&self.ip, &self.port) {
            self.broadcast = SocketAddr::new(*ip, *port)
                .as_broadcast_addr(Some(mask))
                .ok();
        }
        self
    }

    pub fn with_broadcast(mut self, broadcast: Option<SocketAddr>) -> Self {
        self.broadcast = broadcast;
        self
    }

    pub fn bcast_mut(&mut self) -> &mut Option<SocketAddr> {
        &mut self.broadcast
    }
    pub fn with_service_name(mut self, service_name: Option<String>) -> Self {
        self.service_name = service_name;
        self
    }
    pub fn service_name_mut(&mut self) -> &mut Option<String> {
        &mut self.service_name
    }
    pub fn with_version(mut self, version: Option<String>) -> Self {
        self.version = version;
        self
    }
    pub fn version_mut(&mut self) -> &mut Option<String> {
        &mut self.version
    }
    pub fn with_interval(mut self, interval: Option<u64>) -> Self {
        self.interval = interval;
        self
    }
    pub fn interval_mut(&mut self) -> &mut Option<u64> {
        &mut self.interval
    }
}

impl<A> Debug for Broadcaster<A>
where
    A: Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Broadcaster")
            .field("ip", &self.ip)
            .field("port", &self.port)
            .field("broadcast", &self.broadcast)
            .field("service_name", &self.service_name)
            .field("version", &self.version)
            .field("interval", &self.interval)
            .field(
                "advertisement",
                &format!("has_data({})", self.advertisement.is_some()),
            )
            .finish()
    }
}

impl<A: Into<Vec<u8>>> Default for Broadcaster<A>
where
    A: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait AdvertisementTransformer<A>
where
    A: Clone,
    Self: Sized,
{
    fn with_advertisement(self, a: Option<A>) -> Self;
    fn set_advertisement(&mut self, a: A) -> &mut Self;
    fn advertisement(&self) -> Option<&A>;

    fn advert_from_bytes(&mut self, a: Bytes) -> &Self;
    fn advert_into_bytes(&self) -> Bytes;
}

impl AdvertisementTransformer<Advertisement> for Broadcaster<Advertisement> {
    fn with_advertisement(mut self, a: Option<Advertisement>) -> Self {
        self.advertisement = a.map(|t| t.into());
        self
    }
    fn set_advertisement(&mut self, a: Advertisement) -> &mut Self {
        self.advertisement = Some(a);
        self
    }
    fn advertisement(&self) -> Option<&Advertisement> {
        self.advertisement.as_ref()
    }
    fn advert_from_bytes(&mut self, a: Bytes) -> &Self {
        self.advertisement = Advertisement::try_from(a).ok();
        self
    }
    fn advert_into_bytes(&self) -> Bytes {
        self.advertisement
            .as_ref()
            .map(|t| t.to_string().into())
            .unwrap_or_default()
    }
}
impl AdvertisementTransformer<Bytes> for Broadcaster<Bytes> {
    fn with_advertisement(mut self, a: Option<Bytes>) -> Self {
        self.advertisement = a.map(|t| t.into());
        self
    }
    fn set_advertisement(&mut self, a: Bytes) -> &mut Self {
        self.advertisement = Some(a);
        self
    }
    fn advertisement(&self) -> Option<&Bytes> {
        self.advertisement.as_ref()
    }
    fn advert_from_bytes(&mut self, a: Bytes) -> &Self {
        self.advertisement = Some(a);
        self
    }
    fn advert_into_bytes(&self) -> Bytes {
        self.advertisement.clone().unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct Discoverer<S, A> {
    ip: IpRanges,
    service_name: Option<String>,
    version: Option<String>,
    interval: Option<u64>,
    advert: Option<A>,
    validator: Option<Arc<dyn Fn(&Arc<Bytes>) -> Result<(), DiscovererError> + Send + Sync>>,
    processor: Vec<
        Arc<
            dyn Fn(
                    Arc<Bytes>,
                    Option<&S>,
                ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
                + Send
                + Sync
                + 'static,
        >,
    >,
}

impl<S, A> Discoverer<S, A> {
    pub fn new() -> Self {
        Discoverer {
            ip: IpRanges::default(),
            service_name: None,
            version: None,
            interval: None,
            advert: None,
            validator: None,
            processor: Vec::new(),
        }
    }
    pub fn with_ip<T: Into<IpRanges>>(mut self, ip: Option<T>) -> Self {
        self.ip = ip.map(|t| t.into()).unwrap_or(self.ip);
        self
    }
    pub fn set_ip<T: Into<IpRanges>>(&mut self, ip: T) -> &mut Self {
        self.ip = ip.into();
        self
    }

    pub fn with_service_name(mut self, service_name: Option<String>) -> Self {
        self.service_name = service_name;
        self
    }
    pub fn set_service_name(&mut self, service_name: &str) -> &mut Self {
        self.service_name = Some(service_name.to_string());
        self
    }
    pub fn with_version(mut self, version: Option<String>) -> Self {
        self.version = version;
        self
    }
    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }
    pub fn with_interval(mut self, interval: Option<u64>) -> Self {
        self.interval = interval;
        self
    }
    pub fn set_interval(&mut self, interval: u64) -> &mut Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_validator<
        O: Fn(&Arc<Bytes>) -> Result<(), DiscovererError> + Send + Sync + 'static,
    >(
        mut self,
        validator: O,
    ) -> Self {
        self.validator = Some(Arc::new(validator));
        self
    }

    pub fn set_validator<
        O: Fn(&Arc<Bytes>) -> Result<(), DiscovererError> + Send + Sync + 'static,
    >(
        &mut self,
        validator: O,
    ) -> &mut Self {
        self.validator = Some(Arc::new(validator));
        self
    }
}

impl<S, A> Discoverer<S, A>
where
    S: ServiceDiscoveryStateTrait,
{
    pub fn add_processor<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Arc<Bytes>, Option<&S>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
            + Send
            + Sync
            + 'static,
    {
        self.processor.push(Arc::new(f));
        self
    }
}

impl<S, A> Default for Discoverer<S, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, A> Debug for Discoverer<S, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Discoverer")
            .field("ip", &self.ip)
            .field("service_name", &self.service_name)
            .field("version", &self.version)
            .field("interval", &self.interval)
            .field("advert", &format!("has_data({})", self.advert.is_some()))
            .field("validator", &"Available".to_string())
            .field("processor", &format!("{} processors", self.processor.len()))
            .finish()
    }
}

#[derive(Debug)]
pub enum DiscovererError {
    AdvertParsingError(String),
    InvalidDiscoveryData(String),
    ServiceNameMismatch,
    VersionParsingError(String),
    VersionMismatch,
    NoDataAvailable,
}

pub trait DiscovererAdvertValidation {
    fn validate_advert(self) -> Result<(), DiscovererError>;
}

pub trait DiscovererAdvertCapture<A> {
    fn advert_capture(self, b: Bytes) -> Result<A, DiscovererError>;
}

impl<S, A> DiscovererAdvertCapture<Advertisement> for Discoverer<S, A> {
    fn advert_capture(self, b: Bytes) -> Result<Advertisement, DiscovererError> {
        Advertisement::try_from(b)
            .map_err(|e| DiscovererError::AdvertParsingError(e.to_string()))
            .and_then(|t| // Check service_name
                {
                    if let (Some(service_name), Some(advert_service_id)) = (&self.service_name, &t.service_id) {
                        if service_name != advert_service_id {
                            return Err(DiscovererError::ServiceNameMismatch);
                        }
                    } else if self.service_name.is_some() {
                        return Err(DiscovererError::InvalidDiscoveryData("Service name is missing in the advertisement".to_string()));
                    }

                    // Check version using semver
                    if let (Some(required_version), Some(advert_version)) = (&self.version, &t.version) {
                        let parsed_required_version = Version::parse(required_version)
                            .map_err(|e| DiscovererError::VersionParsingError(e.to_string()))?;

                        let parsed_advert_version = Version::parse(advert_version)
                            .map_err(|e| DiscovererError::VersionParsingError(e.to_string()))?;

                        // Check if advert version satisfies the required version (exact match)
                        if parsed_advert_version != parsed_required_version {
                            return Err(DiscovererError::VersionMismatch);
                        }
                    } else if self.version.is_some() {
                        return Err(DiscovererError::InvalidDiscoveryData("Version is missing in the advertisement".to_string()));
                    }

                    Ok(t)
                })
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
    fn set_state<T: Sync + Send + Any + Copy>(&mut self, key: &str, state: T) {}
    fn state<T: Sync + Send + Any + Copy>(&self, key: &str) -> Option<T> {
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

    fn get_or_insert_with<F>(
        &self,
        k: ServiceBinding,
        default: F,
    ) -> Deferred<'_, RwLockWriteGuard<HashMap<ServiceBinding, Service<S, A>>>, Service<S, A>>
    where
        F: FnOnce() -> Service<S, A>;

    fn as_owned_services(&self) -> HashMap<ServiceBinding, Service<S, A>>;
}

pub type ServiceManagerContainer<S, A> = RwLock<HashMap<ServiceBinding, Service<S, A>>>;
impl<S, A> ServiceManagerTrait<S, A> for ServiceManagerContainer<S, A>
where
    A: Clone + Send + Sync,
    S: Clone + Send + Sync,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    fn insert(&self, k: ServiceBinding, service: Service<S, A>) -> Option<Service<S, A>> {
        self.write().insert(k, service)
    }
    fn remove(&self, k: ServiceBinding) -> Option<Service<S, A>> {
        self.write().remove(&k)
    }

    fn get_or_insert_with<F>(
        &self,
        k: ServiceBinding,
        default: F,
    ) -> Deferred<RwLockWriteGuard<HashMap<ServiceBinding, Service<S, A>>>, Service<S, A>>
    where
        F: FnOnce() -> Service<S, A>,
    {
        let mut guard = self.write();
        if !guard.contains_key(&k) {
            guard.insert(k.clone(), default());
        }
        let data = guard.get_mut(&k).unwrap();

        Deferred::new(guard, &k)
    }

    fn as_owned_services(&self) -> HashMap<ServiceBinding, Service<S, A>> {
        self.read()
            .iter()
            .map(|(k, v)| ((*k).clone(), (*v).clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr};
    use std::ops::Deref;
    use tokio::sync::oneshot;
    use tokio::time::timeout;

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

        let unmanaged: ServiceDiscovery<ServiceDiscoveryState, Bytes> =
            ServiceDiscovery::unmanaged();
        assert!(!unmanaged.is_managed());

        let managed = ServiceDiscovery::managed(&state_ref);
        assert!(managed.is_managed());
    }

    #[tokio::test]
    async fn test_managed_service_discovery() {
        let state_ref = &SERVICE_DISCOVERY;
        let discovery = ServiceDiscovery::managed(&state_ref);

        assert!(discovery.managed_state.is_some());
    }

    #[tokio::test]
    async fn test_service_add_broadcaster() {
        let mut discovery: ServiceDiscovery<ServiceDiscoveryState, Bytes> =
            ServiceDiscovery::unmanaged();
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port: u16 = 8080;
        let broadcaster = Broadcaster::new().with_service_name(Some("Test Service".to_string()));

        discovery.add_broadcaster((ip, port), broadcaster);
        assert_eq!(discovery.discovery_type.len(), 1);
    }

    #[tokio::test]
    async fn test_service_add_discoverer() {
        let mut discovery: ServiceDiscovery<ServiceDiscoveryState, Bytes> =
            ServiceDiscovery::unmanaged();
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
                debug!("Processing advert: {:?}", advert);
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

        debug!("Broadcasting: {:?}", broadcaster);

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
                debug!("Call processing function: {:?}", advert);
                if !arc_tx.is_closed() {
                    debug!("sending message");
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
                debug!("Broadcast received successfully, shutting down...");
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
