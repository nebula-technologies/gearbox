use crate::collections::HashMap;
use crate::log::tracing::formatter::deeplog::DeepLogFormatter;
use crate::log::tracing::layer::LogLayer;
use crate::service::discovery::services::common::CommonServiceDiscovery;
use crate::service::discovery::DiscoveryService;
use crate::{error, info};
use axum::Router;
use core::fmt::{Display, Formatter};
use hyper::rt::Executor;
use hyper::server::conn::http1::Builder;
use hyper::server::conn::{http1, http2};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use std::any::{Any, TypeId};
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Notify, Semaphore};
use tokio::time::sleep;
use tokio::{signal, task};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::event;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, Registry};

enum ConnectionBuilder {
    Http1(http1::Builder),
    Http2(http2::Builder<TokioExecutor>),
    H2C(http2::Builder<TokioExecutor>),
}

impl From<http1::Builder> for ConnectionBuilder {
    fn from(t: Builder) -> Self {
        Self::Http1(t)
    }
}

impl From<http2::Builder<TokioExecutor>> for ConnectionBuilder {
    fn from(t: http2::Builder<TokioExecutor>) -> Self {
        Self::Http2(t)
    }
}

impl ConnectionBuilder {
    pub fn serve_connection(
        &self,
        stream: TokioIo<TcpStream>,
        app: TowerToHyperService<Router>,
    ) -> Connection {
        match self {
            Self::Http1(t) => Connection::Http1(t.serve_connection(stream, app)),
            Self::Http2(t) => Connection::Http2(t.serve_connection(stream, app)),
            Self::H2C(t) => Connection::Http2(t.serve_connection(stream, app)),
        }
    }
}

enum Connection {
    Http1(http1::Connection<TokioIo<TcpStream>, TowerToHyperService<Router>>),
    Http2(http2::Connection<TokioIo<TcpStream>, TowerToHyperService<Router>, TokioExecutor>),
    H2C(http2::Connection<TokioIo<TcpStream>, TowerToHyperService<Router>, TokioExecutor>),
}

impl Future for Connection {
    type Output = Result<(), hyper::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Connection::Http1(ref mut fut) => Pin::new(fut).poll(cx),
            Connection::Http2(ref mut fut) => Pin::new(fut).poll(cx),
            Connection::H2C(ref mut fut) => Pin::new(fut).poll(cx),
        }
    }
}

#[derive(Debug)]
pub enum ServerRuntimeError {
    UnknownRuntimeError,
}
impl Display for ServerRuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownRuntimeError => write!(f, "Unknown Runtime Error"),
        }
    }
}

#[derive(Debug)]
pub enum ServerRuntimeStatus {
    GracefulShutdown,
}

impl Display for ServerRuntimeStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GracefulShutdown => write!(f, "Graceful Shutdown of server"),
        }
    }
}

// Define an executor to run tasks
#[derive(Clone)]
struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}

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

impl Default for ModuleBuilder {
    fn default() -> Self {
        Self {
            router: Router::new(),
            app_states: HashMap::new(),
        }
    }
}

impl ModuleBuilder {
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

impl Default for DiscoveryBuilder {
    fn default() -> Self {
        DiscoveryBuilder {
            interval: Some(5),
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            service_name: Some("Log-service".to_string()),
        }
    }
}

impl DiscoveryBuilder {
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

#[derive(Default)]
pub struct HyperConfig {}

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
    sub_tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    service_broadcast: Vec<Option<DiscoveryBuilder>>,
    use_http2: bool,
    certificates: Option<(String, String)>,
    hyper_config: HyperConfig,
    include_subtasks_in_worker_pool: bool,
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
            sub_tasks: Vec::new(),
            service_broadcast: Vec::new(),
            use_http2: false,
            certificates: None,
            hyper_config: HyperConfig::default(),
            include_subtasks_in_worker_pool: false,
        }
    }

    pub fn include_subtasks_in_worker_pool(mut self, b: bool) -> Self {
        self.include_subtasks_in_worker_pool = b;
        self
    }

    pub fn add_subtask<F>(mut self, f: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.sub_tasks.push(Box::pin(f));
        self
    }

    pub fn use_http2(mut self) -> Self {
        self.use_http2 = true;
        self
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

    pub fn add_service_broadcast<O: FnOnce(DiscoveryBuilder) -> Option<DiscoveryBuilder>>(
        mut self,
        o: O,
    ) -> Self {
        self.service_broadcast.push(o(DiscoveryBuilder::default()));
        self
    }

    pub fn with_log_service_discovery<O: FnOnce(DiscoveryBuilder) -> Option<DiscoveryBuilder>>(
        mut self,
        o: O,
    ) -> Self {
        self.trace_layer = true;
        self.logger_discovery_builder = o(DiscoveryBuilder::default());
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
        let num_subtasks = self.sub_tasks.len();
        let body = async {
            println!("Creating app");
            let router_with_state = Router::new();

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

            for i in self.service_broadcast {
                let mut current_discovery_builder = i.clone();
                let discovery_builder =
                    current_discovery_builder.get_or_insert(DiscoveryBuilder::default());

                let common_broadcaster = CommonServiceDiscovery::default();
                common_broadcaster.set_service_config(|mut t| {
                    t.broadcast.ip = discovery_builder.ip;
                    t.broadcast.bcast_port = discovery_builder.port;
                    t.message.service_name = discovery_builder.service_name.clone();
                    t.broadcast.bcast_interval = discovery_builder.interval.map(|t| t as u16);

                    t
                });
            }

            setup_logger(
                self.logger,
                self.logger_discovery,
                self.logger_discovery_builder,
            );

            for i in self.sub_tasks {
                task::spawn(i);
            }

            info!("Setting up listener socket address");
            let socket_addr = SocketAddr::new(self.address, self.port);
            let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();

            if start_server {
                let result = if self.use_http2 {
                    if self.certificates.is_none() {
                        spin_h2c_server(listener, self.hyper_config, app_with_state).await
                    } else {
                        panic!("not implemented")
                    }
                } else {
                    info!("Starting server");
                    spin_http1_server(listener, self.hyper_config, app_with_state).await
                };

                if let Err(e) = result {
                    error!("{}", e);
                } else if let Ok(t) = result {
                    info!("{}", t);
                }
            }
        };

        println!("Setting up the thread builder for tokio");
        let mut builder = tokio::runtime::Builder::new_multi_thread();

        if let Some(threads) = self.worker_pool.as_ref() {
            if self.include_subtasks_in_worker_pool {
                println!("Using defined worker threads");
                builder.worker_threads(*threads);
            } else {
                println!("Using defined worker threads");
                builder.worker_threads(
                    *threads + num_subtasks + if self.logger_discovery { 1 } else { 0 },
                );
            }
        } else {
            println!("Using auto-lookup worker threads");
            let num_cores = num_cpus::get();
            if self.include_subtasks_in_worker_pool {
                println!("Using defined worker threads");
                builder.worker_threads(num_cores);
            } else {
                println!("Using defined worker threads");
                builder.worker_threads(
                    num_cores + num_subtasks + if self.logger_discovery { 1 } else { 0 },
                );
            }
        }

        builder
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body)
    }

    pub fn build(self) {
        self.build_inner(true)
    }

    #[cfg(test)]
    pub(crate) fn build_test(self) {
        self.build_inner(false)
    }
}
async fn spin_h2c_server(
    listener: TcpListener,
    hyper_config: HyperConfig, // Placeholder, can be expanded
    app: Router,
) -> Result<ServerRuntimeStatus, ServerRuntimeError> {
    let h2 = http2::Builder::new(TokioExecutor);
    spin_server(h2, listener, hyper_config, app).await
}

async fn spin_http1_server(
    listener: TcpListener,
    hyper_config: HyperConfig, // Not used in this example, but you can expand this struct
    app: Router,
) -> Result<ServerRuntimeStatus, ServerRuntimeError> {
    let http = http1::Builder::new();
    spin_server(http, listener, hyper_config, app).await
}

async fn spin_server<H: Into<ConnectionBuilder>>(
    http_handler_into: H,
    listener: TcpListener,
    hyper_config: HyperConfig,
    app: Router,
) -> Result<ServerRuntimeStatus, ServerRuntimeError> {
    let http_handler = http_handler_into.into();
    info!("Starting server");

    // Atomic counter to track active connections
    let active_connections = Arc::new(AtomicUsize::new(0));
    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_triggered = Arc::new(AtomicBool::new(false));

    let shutdown_notify_clone = shutdown_notify.clone();
    let shutdown_triggered_clone = shutdown_triggered.clone();

    // Spawn a task to capture shutdown signals (SIGINT or SIGTERM)
    tokio::spawn(async move {
        shutdown_signal_capture().await;
        shutdown_triggered_clone.store(true, Ordering::SeqCst); // Set the shutdown flag
        shutdown_notify_clone.notify_one();
    });

    let shutdown_triggered_clone = shutdown_triggered.clone();

    let service_app = TowerToHyperService::new(app);

    // Server accept loop
    loop {
        tokio::select! {
            // Accept new connections
            Ok((stream, _addr)) = listener.accept() => {
                if shutdown_triggered_clone.load(Ordering::SeqCst) {
                    error!("Server is awaiting shutdown, no new connections allowed");
                    continue;
                }

                // Increment the active connection count
                active_connections.fetch_add(1, Ordering::SeqCst);
                info!("Accepted new connection. Active connections: {}", active_connections.load(Ordering::SeqCst));

                let io = TokioIo::new(stream);
                let conn = http_handler.serve_connection(io, service_app.clone());

                // Spawn a task to handle each connection
                let active_connections_clone = active_connections.clone();
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        error!("Error serving connection: {:?}", e);
                    }
                    // Decrement the active connection count when the connection is finished
                    active_connections_clone.fetch_sub(1, Ordering::SeqCst);
                    info!("Connection closed. Active connections: {}", active_connections_clone.load(Ordering::SeqCst));
                });
            },
            // Shutdown signal received, wait for it asynchronously
            _ = shutdown_notify.notified() => {
                info!("Shutdown signal received.");
                break;
            }
        }
    }

    // Wait for all active connections to finish
    info!("Waiting for active connections to finish...");
    while active_connections.load(Ordering::SeqCst) > 0 {
        sleep(Duration::from_secs(1)).await;
    }

    info!("All connections closed, shutting down gracefully.");
    Ok(ServerRuntimeStatus::GracefulShutdown)
}

async fn shutdown_signal_capture() {
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
                    current_discovery_builder.get_or_insert(DiscoveryBuilder::default());

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
