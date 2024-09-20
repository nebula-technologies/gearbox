use crate::collections::HashMap;
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

#[derive(Clone)]
pub struct AppStateBuilder {
    // A map for storing application state keyed by TypeId
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl AppStateBuilder {
    pub fn add<T: 'static + Default + Send + Sync>(&mut self) -> &mut Self {
        let state = T::default();
        self.state.insert(state.type_id(), Arc::new(T::default()));
        self
    }

    pub fn build(self) -> Arc<AppState> {
        Arc::new(AppState {
            state: self.clone().state,
        })
    }
}

impl Default for AppStateBuilder {
    fn default() -> Self {
        AppStateBuilder {
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
    pub async fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.state
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

pub struct ModuleBuilder<'a> {
    router: &'a mut Option<Router>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(router: &'a mut Option<Router>) -> Self {
        Self { router }
    }
    pub fn router<O: FnOnce(&mut Option<Router>)>(mut self, o: O) -> Self {
        o(&mut self.router);
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
    router: Router,
    logger: LogStyle,
    logger_discovery: bool,
    logger_discovery_builder: Option<DiscoveryBuilder>,
    trace_layer: bool,
    app_state: Option<AppStateBuilder>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        let router = Router::new().with_state(Arc::new(AppState::new(HashMap::new())));

        Self {
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 3000,
            worker_pool: None,
            router,
            logger: LogStyle::DeepLog,
            logger_discovery: false,
            logger_discovery_builder: None,
            trace_layer: false,
            app_state: None,
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

    pub fn add_router(mut self, r: Router) -> Self {
        self.router = self.router.nest("", r);
        self
    }

    pub fn set_worker_pool(mut self, max_workers: usize) -> Self {
        self.worker_pool = Some(max_workers);
        self
    }

    pub fn build(self) {
        println!("Building body closure");
        let body = async {
            println!("Creating app");
            let mut app = self.router;

            if self.trace_layer {
                app = app.layer((
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

            println!("Setting up listener socket address");
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

            println!("Starting server");
            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap()
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
