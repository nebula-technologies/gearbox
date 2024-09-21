use crate::collections::HashMap;
use crate::info;
use crate::log::tracing::entity::syslog::Severity;
use crate::log::tracing::formatter::bunyan::Bunyan;
use crate::log::tracing::formatter::deeplog::DeepLogFormatter;
use crate::log::tracing::formatter::syslog::Syslog;
use crate::log::tracing::layer::LogLayer;
use crate::service::discovery::DiscoveryService;
use axum::serve::IncomingStream;
use axum::{Router, ServiceExt};
use futures::SinkExt;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::RwLock;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::event;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub trait ModuleDefinition {
    const NAME: &'static str;
    const ROUTER: fn() -> Router<Arc<AppState>>;
    const STATES: fn(&mut RwAppState);
}

pub struct RwAppState {
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl RwAppState {
    pub fn add<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(t));
        self
    }
}

impl Default for RwAppState {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    // A map for storing application state keyed by TypeId
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl AppState {
    // Create a new AppState
    pub fn new(state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        Self { state }
    }

    // Get a reference to a value in the state by type
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.state
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

pub struct ModuleBuilder {
    router: Router,
    app_states: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            app_states: HashMap::new(),
        }
    }
    pub fn add_router(mut self, base: &str, router: Router) -> Self {
        self.router = self.router.nest(base, router);
        self
    }

    pub fn add_state<T>(mut self) -> Self
    where
        T: Default + Send + Sync + 'static,
    {
        self.app_states
            .insert(TypeId::of::<T>(), Arc::new(T::default()));
        self
    }

    pub fn add_state_object<T>(mut self, o: T) -> Self
    where
        T: Default + Send + Sync + 'static,
    {
        self.app_states.insert(TypeId::of::<T>(), Arc::new(o));
        self
    }
}
#[derive(Debug, Clone)]
pub struct DiscoveryBuilder {
    ip: Option<IpAddr>,
    port: Option<u16>,
    interval: Option<usize>,
    service_name: Option<String>,
}

impl DiscoveryBuilder {
    pub fn new() -> Self {
        DiscoveryBuilder {
            interval: Some(5),
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            service_name: Some("Log-service".to_string()),
        }
    }

    pub fn set_ip(mut self, ip: IpAddr) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn set_interval(mut self, interval: usize) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn set_service_name(mut self, service_name: &str) -> Self {
        self.service_name = Some(service_name.to_string());
        self
    }
}

pub enum LogStyle {
    Bunyan,
    DeepLog,
    Syslog,
}

pub struct ServerBuilder {
    address: IpAddr,
    port: u16,
    worker_pool: Option<usize>,
    router: Router<Arc<AppState>>,
    logger: LogStyle,
    logger_discovery: bool,
    logger_discovery_builder: Option<DiscoveryBuilder>,
    trace_layer: bool,
    app_state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerBuilder {
    pub fn new() -> Self {
        let router = Router::new();

        Self {
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 65000,
            worker_pool: None,
            router,
            logger: LogStyle::DeepLog,
            logger_discovery: false,
            logger_discovery_builder: None,
            trace_layer: false,
            app_state: HashMap::new(),
        }
    }

    pub fn set_address(mut self, ip: &[u16]) -> Self {
        if ip.len() == 4 {
            self.address = IpAddr::V4(Ipv4Addr::new(
                ip[0] as u8,
                ip[1] as u8,
                ip[2] as u8,
                ip[3] as u8,
            ));
        } else if ip.len() != 16 {
            self.address = IpAddr::V6(Ipv6Addr::new(
                ip[0], ip[1], ip[2], ip[3], ip[4], ip[5], ip[6], ip[7],
            ));
        } else {
            panic!("Invalid IP address used");
        }

        self
    }

    pub fn add_module<T: ModuleDefinition>(mut self) -> Self {
        let router = T::ROUTER();
        self.router = self.router.merge(router);

        let mut rw_app_state = RwAppState::default();
        T::STATES(&mut rw_app_state);
        for (key, val) in rw_app_state.state {
            self.app_state.insert(key, val);
        }

        self
    }

    pub fn with_trace_layer(mut self) -> Self {
        self.trace_layer = true;
        self
    }
    pub fn with_log_service_discovery<O: FnOnce(DiscoveryBuilder) -> Option<DiscoveryBuilder>>(
        mut self,
        o: O,
    ) -> Self {
        self.trace_layer = true;
        self.logger_discovery_builder = o(DiscoveryBuilder::new());
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn add_state<T>(mut self) -> Self
    where
        T: Default + Send + Sync + 'static,
    {
        self.app_state
            .insert(TypeId::of::<T>(), Arc::new(T::default()));
        self
    }

    pub fn add_state_object<T>(mut self, o: T) -> Self
    where
        T: Default + Send + Sync + 'static,
    {
        self.app_state.insert(TypeId::of::<T>(), Arc::new(o));
        self
    }

    pub fn set_worker_pool(mut self, max_workers: usize) -> Self {
        self.worker_pool = Some(max_workers);
        self
    }

    fn build_inner(self, start_server: bool) {
        println!("Building body closure");
        let body = async {
            println!("Creating app");
            let mut router_with_state = Router::new();

            let app = self
                .router
                .with_state(Arc::new(AppState::new(self.app_state)));
            let mut app_with_state = router_with_state.merge(app);

            if self.trace_layer {
                app_with_state = app_with_state.layer((
                    TraceLayer::new_for_http(),
                    // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                    // requests don't hang forever.
                    TimeoutLayer::new(Duration::from_secs(10)),
                ));
            }

            setup_logger(
                self.logger,
                self.logger_discovery,
                self.logger_discovery_builder,
            );

            info!("Setting up listener socket address");
            let socket_addr = SocketAddr::new(self.address, self.port);
            let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();

            async fn shutdown_signal() {
                let ctrl_c = async {
                    signal::ctrl_c()
                        .await
                        .expect("failed to install Ctrl+C handler");
                };

                #[cfg(unix)]
                let terminate = async {
                    signal::unix::signal(signal::unix::SignalKind::terminate())
                        .expect("failed to install signal handler")
                        .recv()
                        .await;
                };

                #[cfg(not(unix))]
                let terminate = std::future::pending::<()>();

                tokio::select! {
                    _ = ctrl_c => {},
                    _ = terminate => {},
                }
            }

            if start_server {
                info!("Starting server");

                axum::serve(listener, app_with_state)
                    .with_graceful_shutdown(shutdown_signal())
                    .await
                    .unwrap()
            }
        };

        println!("Setting up the thread builder for tokio");
        let mut builder = tokio::runtime::Builder::new_multi_thread();

        if let Some(threads) = self.worker_pool.as_ref() {
            println!("Using defined worker threads");
            builder.worker_threads(*threads);
        } else {
            println!("Using auto-lookup worker threads");
        }

        builder
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }

    pub fn build(self) {
        self.build_inner(true)
    }

    pub(crate) fn build_test(self) {
        self.build_inner(false)
    }
}

fn setup_logger(logger: LogStyle, discovery: bool, builder: Option<DiscoveryBuilder>) {
    let mut formatter = match logger {
        LogStyle::Bunyan => {
            panic!("Bunyan not currently supported")
        }
        LogStyle::DeepLog => DeepLogFormatter::default(),
        LogStyle::Syslog => {
            panic!("Syslog not currently supported")
        }
    };

    if discovery {
        let (fmt, handle) = formatter
            .set_service_config(|mut t| {
                let mut current_discovery_builder = builder.clone();
                let discovery_builder =
                    current_discovery_builder.get_or_insert(DiscoveryBuilder::new());

                t.broadcast.ip = discovery_builder.ip;
                t.broadcast.bcast_port = discovery_builder.port;
                t.message.service_name = discovery_builder.service_name.clone();
                t.broadcast.bcast_interval = discovery_builder.interval.map(|t| t as u16);

                t
            })
            .start_discovery();
        formatter = fmt;
    }

    let formatter = LogLayer::new(None, std::io::stdout, formatter);
    let subscriber = Registry::default().with(formatter);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to attach log subscriber");
    event!(
        Level::DEBUG,
        level = "emergency",
        "Testing subscriber with level override"
    );
}

#[cfg(test)]
mod test {
    use super::{AppState, ModuleDefinition, RwAppState};
    use axum::Router;
    use std::sync::Arc;

    pub struct TestState {}
    pub struct TestModule {}

    impl TestModule {
        pub fn routes() -> Router<Arc<AppState>> {
            Router::new()
        }
        pub fn states(state: &mut RwAppState) {
            state.add(TestState {});
        }
    }

    impl ModuleDefinition for TestModule {
        const NAME: &'static str = "TestModule";
        const ROUTER: fn() -> Router<Arc<AppState>> = TestModule::routes;
        const STATES: fn(&mut RwAppState) = TestModule::states;
    }

    #[test]
    fn setup_builder() {
        //
        // fn audit_module(m: ModuleBuilder)
        //
        // let mut state = AppState::default();
        // state.add::<crate::AppState>();
        //
        // let router = Router::new()
        //     .nest("/", modules::audit_log::routes())
        //     .with_state(Arc::new(state.build()));

        let server_builder = crate::service::framework::axum::ServerBuilder::new();
        server_builder
            .add_module::<TestModule>()
            .set_worker_pool(30)
            .with_log_service_discovery(|t| None)
            .build_test();
    }
}
