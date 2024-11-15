use crate::collections::const_hash_map::HashMap as ConstHashMap;
use crate::common::socket_bind_addr::SocketBindAddr;
use crate::externs::tracing::{Event, Subscriber};
use crate::log::tracing::formatter::bunyan::Bunyan;
use crate::log::tracing::formatter::deeplog;
use crate::log::tracing::formatter::deeplog::DeepLogFormatter;
use crate::log::tracing::formatter::syslog::Syslog;
use crate::log::tracing::layer::{LogLayer, Storage, Type};
use crate::log::tracing::LogFormatter;
use crate::service::discovery::service_binding::ServiceBinding;
use crate::service::discovery::service_discovery::{
    Broadcaster, Discoverer, Service, ServiceDiscovery,
};
use crate::service::framework::axum::server_framework_config::ServerFrameworkConfig;
use crate::service::framework::axum::{
    ConnectionBuilder, FrameworkState, FrameworkStateContainer, HyperConfig, LogFormatterBackend,
    LogOutput, ModuleDefinition, ModuleManager, RwFrameworkState, ServerRuntimeError,
    ServerRuntimeStatus, TokioExecutor,
};
use crate::{debug, error, info};
use axum::handler::Handler;
use axum::Router;
use bytes::Bytes;
use hyper::server::conn::{http1, http2};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use spin::rwlock::RwLock;
use std::any::TypeId;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Notify;
use tokio::time::sleep;
use tokio::{signal, task};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::{event, Level};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::SpanRef;
use tracing_subscriber::Registry;

static SERVICE_DISCOVERY: RwLock<
    ConstHashMap<ServiceBinding, Service<Arc<FrameworkState>, Bytes>>,
> = RwLock::new(ConstHashMap::new());

pub struct ServerBuilder {
    pub(crate) address: IpAddr,
    pub(crate) port: u16,
    pub(crate) worker_pool: Option<usize>,
    pub(crate) router: Router<Arc<FrameworkState>>,
    pub(crate) logger: LogFormatterBackend,
    pub(crate) logger_output: LogOutput,
    pub(crate) logger_discovery: bool,
    pub(crate) trace_layer: bool,
    pub(crate) app_state: RwFrameworkState,
    pub(crate) sub_tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    pub(crate) use_http2: bool,
    pub(crate) certificates: Option<(String, String)>,
    pub(crate) hyper_config: HyperConfig,
    pub(crate) include_subtasks_in_worker_pool: bool,
    pub(crate) module_manager: ModuleManager,
    pub(crate) fallback_response: Option<Router<Arc<FrameworkState>>>,
    pub(crate) service_discovery: ServiceDiscovery<Arc<FrameworkState>, Bytes>,
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
            port: 3000,
            worker_pool: None,
            router,
            logger: LogFormatterBackend::DeepLog,
            logger_output: LogOutput::Full,
            logger_discovery: false,
            trace_layer: false,
            app_state: RwFrameworkState::default(),
            sub_tasks: Vec::new(),
            use_http2: false,
            certificates: None,
            hyper_config: HyperConfig::default(),
            include_subtasks_in_worker_pool: false,
            module_manager: ModuleManager::default(),
            fallback_response: None,
            service_discovery: ServiceDiscovery::managed(&SERVICE_DISCOVERY),
        }
    }

    pub fn with_active_modules(mut self, v: Vec<String>) -> Self {
        self.module_manager.active_modules(v);
        self
    }

    pub fn with_log_output<O: Fn(LogOutput) -> LogOutput>(mut self, o: O) -> Self {
        self.logger_output = o(LogOutput::Full);
        self
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

    pub fn with_module<T: ModuleDefinition>(mut self) -> Self {
        self.module_manager.add_module::<T>();

        self
    }

    pub fn with_trace_layer(mut self) -> Self {
        self.trace_layer = true;
        self
    }

    pub fn with_service_broadcast<
        O: FnOnce(Broadcaster<Bytes>) -> Option<(SocketBindAddr, Broadcaster<Bytes>)>,
    >(
        mut self,
        o: O,
    ) -> Self {
        if let Some((addr, bcast)) = o(Broadcaster::default()) {
            self.service_discovery.add_broadcaster(addr, bcast);
        }
        self
    }

    pub fn with_service_discovery<
        O: FnOnce(
            Discoverer<FrameworkState, Bytes>,
        ) -> Option<(SocketBindAddr, Discoverer<Arc<FrameworkState>, Bytes>)>,
    >(
        mut self,
        o: O,
    ) -> Self {
        if let Some((addr, discover)) = o(Discoverer::default()) {
            self.service_discovery.add_discoverer(addr, discover);
        }
        self
    }

    pub fn enable_log_service_discovery(mut self) -> Self {
        self.logger_discovery = true;
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn add_state<T>(mut self) -> Self
    where
        T: Default + Send + Sync + 'static,
    {
        self.app_state.add_default::<T>();
        self
    }

    pub fn add_state_object<T>(mut self, o: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.app_state.add::<T>(o);
        self
    }

    pub fn with_worker_pool(mut self, max_workers: usize) -> Self {
        self.worker_pool = Some(max_workers);
        self
    }

    pub fn with_fallback<H, T>(mut self, handler: H) -> Self
    where
        H: Handler<T, Arc<FrameworkState>>,
        T: 'static,
    {
        let router = Router::new();
        self.fallback_response = Some(router.fallback(handler));
        self
    }

    fn build_inner<S: FrameworkStateContainer>(mut self, start_server: bool, state_manager: S) {
        let framework_setup_config = ServerFrameworkConfig::from(&self);

        let num_subtasks = self.sub_tasks.len();
        let body = async {
            debug!("Creating app");
            debug!("Initializing FrameworkState");
            let framework_state = self.module_manager.setup_module_states(self.app_state);

            debug!("Building body closure");
            setup_logger(self.logger, self.logger_output, &framework_state);

            debug!("Setting up advertiser and discoverer from modules");
            self.module_manager
                .setup_advertiser(&mut self.service_discovery, &framework_setup_config)
                .setup_discoverer(&mut self.service_discovery, &framework_setup_config);

            debug!("Starting service discovery");

            self.service_discovery.serve(Some(framework_state.clone()));
            debug!("Initializing base router");
            let router_with_state = Router::new();

            debug!("Initializing Merger Router");
            let mut router = Router::new();

            debug!("Adding liveness and readiness routers");
            router = router
                .merge(self.module_manager.setup_liveness_router())
                .merge(self.module_manager.setup_readiness_router());

            debug!("Adding Module Routers");
            router = router.merge(self.module_manager.setup_module_routers());

            if let Some(fallback) = self.fallback_response {
                debug!("Adding fallback router");
                router = router.merge(fallback);
            }

            debug!("Building App router with State");
            let app = self
                .router
                .merge(router)
                .with_state(framework_state.clone());

            debug!("Merging routers into base router");
            let mut app_with_state = router_with_state.merge(app);

            if self.trace_layer {
                debug!("Adding Trace and Timeout Layers");
                app_with_state = app_with_state.layer((
                    TraceLayer::new_for_http(),
                    // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                    // requests don't hang forever.
                    TimeoutLayer::new(Duration::from_secs(10)),
                ));
            }

            debug!("Spawning subtasks");
            for i in self.sub_tasks {
                task::spawn(i);
            }

            debug!("Running module pre-run tasks");
            if self.module_manager.has_pre_run() {
                self.module_manager.run_pre_run();
            }

            debug!("Setting up listener socket address");
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
                    debug!("Starting server");
                    spin_http1_server(listener, self.hyper_config, app_with_state).await
                };

                if let Err(e) = result {
                    error!("{}", e);
                } else if let Ok(t) = result {
                    debug!("{}", t);
                }
            }

            if self.module_manager.has_post_run() {
                self.module_manager.run_post_run();
            }
        };

        debug!("Setting up the thread builder for tokio");
        let mut builder = tokio::runtime::Builder::new_multi_thread();

        if let Some(threads) = self.worker_pool.as_ref() {
            if self.include_subtasks_in_worker_pool {
                debug!("Using defined worker threads");
                builder.worker_threads(*threads);
            } else {
                debug!("Using defined worker threads");
                builder.worker_threads(
                    *threads + num_subtasks + if self.logger_discovery { 1 } else { 0 },
                );
            }
        } else {
            debug!("Using auto-lookup worker threads");
            let num_cores = num_cpus::get();
            if self.include_subtasks_in_worker_pool {
                debug!("Using defined worker threads");
                builder.worker_threads(num_cores);
            } else {
                debug!("Using defined worker threads");
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

    pub fn build<S: FrameworkStateContainer>(self, s: S) {
        self.build_inner(true, s)
    }

    #[cfg(test)]
    pub(crate) fn build_test<S: FrameworkStateContainer>(self, s: S) {
        self.build_inner(false, s)
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
        shutdown_signal_capture(shutdown_triggered_clone, shutdown_notify_clone).await;
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
                let active_conn_clone = active_connections.clone();
                let io = TokioIo::new(stream);
                let conn = http_handler.serve_connection(io, service_app.clone());
                tokio::spawn(async move {
                    // Increment the active connection count
                    active_conn_clone.fetch_add(1, Ordering::SeqCst);
                    info!("Accepted new connection. Active connections: {}", active_conn_clone.load(Ordering::SeqCst));


                    if let Err(e) = conn.await {
                        error!("Error serving connection: {:?}", e);
                    }
                    // Decrement the active connection count when the connection is finished
                    active_conn_clone.fetch_sub(1, Ordering::SeqCst);
                    info!("Connection closed. Active connections: {}", active_conn_clone.load(Ordering::SeqCst));
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

async fn shutdown_signal_capture(
    shutdown_triggered_clone: Arc<AtomicBool>,
    shutdown_notify_clone: Arc<Notify>,
) {
    let ctrl_c_count = Arc::new(AtomicUsize::new(0));

    let ctrl_c = {
        let ctrl_c_count = Arc::clone(&ctrl_c_count);
        let shutdown_triggered_clone = Arc::clone(&shutdown_triggered_clone);
        let shutdown_notify_clone = Arc::clone(&shutdown_notify_clone);

        async move {
            loop {
                signal::ctrl_c()
                    .await
                    .expect("failed to install Ctrl+C handler");

                // Increment the Ctrl+C count
                let count = ctrl_c_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count == 1 {
                    // First Ctrl+C, initiate graceful shutdown
                    println!("Received Ctrl+C signal. Initiating graceful shutdown...");
                    shutdown_triggered_clone.store(true, Ordering::SeqCst); // Set the shutdown flag
                    shutdown_notify_clone.notify_one();
                } else if count >= 3 {
                    // On third Ctrl+C, force shutdown
                    println!("Received 3 Ctrl+C signals, forcing shutdown.");
                    std::process::exit(0);
                } else {
                    println!(
                        "Received Ctrl+C signal again. Press {} more time(s) to force quit.",
                        3 - count
                    );
                }
            }
        }
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

fn setup_logger(logger: LogFormatterBackend, output: LogOutput, state: &Arc<FrameworkState>) {
    let mut formatter: LogFormatterWrapper = match logger {
        LogFormatterBackend::Bunyan => {
            panic!("Bunyan not currently supported")
        }
        LogFormatterBackend::DeepLog => {
            DEFAULT_LOG_BACKEND
                .write()
                .replace(LogFormatterBackend::DeepLog);
            let formatter = DeepLogFormatter::default();
            match output {
                LogOutput::Minimal => formatter.set_output_style(deeplog::LogStyleOutput::Minimal),
                LogOutput::Full => formatter.set_output_style(deeplog::LogStyleOutput::Full),
                LogOutput::Human => formatter.set_output_style(deeplog::LogStyleOutput::Human),
                LogOutput::Default => formatter,
            }
            .into()
        }
        LogFormatterBackend::Syslog => {
            panic!("Syslog not currently supported")
        }
    };

    let log_layer = LogLayer::new(None, std::io::stdout, formatter);
    let subscriber = Registry::default().with(log_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to attach log subscriber");
    event!(
        Level::DEBUG,
        level = "emergency",
        "Testing subscriber with level override"
    );
}

static DEFAULT_LOG_BACKEND: RwLock<Option<LogFormatterBackend>> = RwLock::new(None);

pub enum LogFormatterWrapper {
    DeepLog(DeepLogFormatter),
    Bunyan(Bunyan),
    Syslog(Syslog),
}

impl Default for LogFormatterWrapper {
    fn default() -> Self {
        match DEFAULT_LOG_BACKEND.read().as_ref() {
            Some(LogFormatterBackend::Bunyan) => LogFormatterWrapper::Bunyan(Bunyan::default()),
            Some(LogFormatterBackend::DeepLog) => {
                LogFormatterWrapper::DeepLog(DeepLogFormatter::default())
            }
            Some(LogFormatterBackend::Syslog) => LogFormatterWrapper::Syslog(Syslog::default()),
            None => LogFormatterWrapper::DeepLog(DeepLogFormatter::default()),
        }
    }
}

impl From<DeepLogFormatter> for LogFormatterWrapper {
    fn from(f: DeepLogFormatter) -> Self {
        LogFormatterWrapper::DeepLog(f)
    }
}

impl LogFormatter for LogFormatterWrapper {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        &self,
        layer: &LogLayer<W, F>,
    ) -> Self {
        match self {
            LogFormatterWrapper::DeepLog(formatter) => {
                formatter.log_layer_defaults(layer);
                LogFormatterWrapper::DeepLog(formatter.clone())
            }
            LogFormatterWrapper::Bunyan(formatter) => {
                formatter.log_layer_defaults(layer);
                LogFormatterWrapper::Bunyan(formatter.clone())
            }
            LogFormatterWrapper::Syslog(formatter) => {
                formatter.log_layer_defaults(layer);
                LogFormatterWrapper::Syslog(formatter.clone())
            }
        }
    }

    fn format_event<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String {
        match self {
            LogFormatterWrapper::DeepLog(formatter) => {
                formatter.format_event(current_span, event, event_visitor)
            }
            LogFormatterWrapper::Bunyan(formatter) => {
                formatter.format_event(current_span, event, event_visitor)
            }
            LogFormatterWrapper::Syslog(formatter) => {
                formatter.format_event(current_span, event, event_visitor)
            }
        }
    }

    fn format_span<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String {
        match self {
            LogFormatterWrapper::DeepLog(formatter) => formatter.format_span(span, ty),
            LogFormatterWrapper::Bunyan(formatter) => formatter.format_span(span, ty),
            LogFormatterWrapper::Syslog(formatter) => formatter.format_span(span, ty),
        }
    }
}
