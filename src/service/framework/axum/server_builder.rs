use crate::collections::HashMap;
use crate::log::tracing::formatter::deeplog;
use crate::log::tracing::formatter::deeplog::DeepLogFormatter;
use crate::log::tracing::layer::LogLayer;
use crate::service::discovery::services::common::CommonServiceDiscovery;
use crate::service::discovery::DiscoveryService;
use crate::service::framework::axum::{
    AppState, BoxFn, ConnectionBuilder, DiscoveryBuilder, HyperConfig, LogFormatter, LogOutput,
    ModuleDefinition, ModuleManager, RwAppState, ServerRuntimeError, ServerRuntimeStatus,
    TokioExecutor,
};
use crate::{error, info};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use hyper::server::conn::{http1, http2};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use std::any::{Any, TypeId};
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
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub struct ServerBuilder {
    address: IpAddr,
    port: u16,
    worker_pool: Option<usize>,
    router: Router<Arc<AppState>>,
    logger: LogFormatter,
    logger_output: LogOutput,
    logger_discovery: bool,
    logger_discovery_builder: Option<DiscoveryBuilder>,
    trace_layer: bool,
    app_state: RwAppState,
    sub_tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    service_broadcast: Vec<Option<DiscoveryBuilder>>,
    use_http2: bool,
    certificates: Option<(String, String)>,
    hyper_config: HyperConfig,
    include_subtasks_in_worker_pool: bool,
    module_manager: ModuleManager,
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
            logger: LogFormatter::DeepLog,
            logger_output: LogOutput::Full,
            logger_discovery: false,
            logger_discovery_builder: None,
            trace_layer: false,
            app_state: RwAppState::default(),
            sub_tasks: Vec::new(),
            service_broadcast: Vec::new(),
            use_http2: false,
            certificates: None,
            hyper_config: HyperConfig::default(),
            include_subtasks_in_worker_pool: false,
            module_manager: ModuleManager::default(),
        }
    }

    pub fn active_modules(mut self, v: Vec<String>) -> Self {
        self.module_manager.active_modules(v);
        self
    }

    pub fn set_log_output<O: Fn(LogOutput) -> LogOutput>(mut self, o: O) -> Self {
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

    pub fn add_module<T: ModuleDefinition>(mut self) -> Self {
        self.module_manager.add_module::<T>();

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

    pub fn set_worker_pool(mut self, max_workers: usize) -> Self {
        self.worker_pool = Some(max_workers);
        self
    }

    fn build_inner(mut self, start_server: bool) {
        println!("Building body closure");
        let num_subtasks = self.sub_tasks.len();
        let body = async {
            println!("Creating app");
            let router_with_state = Router::new();

            let mut router = Router::new();

            router = router
                .merge(self.module_manager.setup_liveness_router())
                .merge(self.module_manager.setup_readiness_router())
                .merge(self.module_manager.setup_module_routers());

            let app = self
                .router
                .merge(router)
                .with_state(self.module_manager.setup_module_states(self.app_state));

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
                self.logger_output,
            );

            for i in self.sub_tasks {
                task::spawn(i);
            }

            if self.module_manager.has_pre_run() {
                self.module_manager.run_pre_run();
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

            if self.module_manager.has_post_run() {
                self.module_manager.run_post_run();
            }
        };

        info!("Setting up the thread builder for tokio");
        let mut builder = tokio::runtime::Builder::new_multi_thread();

        if let Some(threads) = self.worker_pool.as_ref() {
            if self.include_subtasks_in_worker_pool {
                info!("Using defined worker threads");
                builder.worker_threads(*threads);
            } else {
                info!("Using defined worker threads");
                builder.worker_threads(
                    *threads + num_subtasks + if self.logger_discovery { 1 } else { 0 },
                );
            }
        } else {
            info!("Using auto-lookup worker threads");
            let num_cores = num_cpus::get();
            if self.include_subtasks_in_worker_pool {
                info!("Using defined worker threads");
                builder.worker_threads(num_cores);
            } else {
                info!("Using defined worker threads");
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

fn setup_logger(
    logger: LogFormatter,
    discovery: bool,
    builder: Option<DiscoveryBuilder>,
    output: LogOutput,
) {
    let mut formatter = match logger {
        LogFormatter::Bunyan => {
            panic!("Bunyan not currently supported")
        }
        LogFormatter::DeepLog => {
            let formatter = DeepLogFormatter::default();
            match output {
                LogOutput::Minimal => formatter.set_output_style(deeplog::LogStyleOutput::Minimal),
                LogOutput::Full => formatter.set_output_style(deeplog::LogStyleOutput::Full),
                LogOutput::Default => formatter,
            }
        }
        LogFormatter::Syslog => {
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
