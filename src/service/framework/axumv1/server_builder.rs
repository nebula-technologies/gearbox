use crate::net::socket_addr::SocketAddrs;
use crate::rails::ext::blocking::TapResult;
use crate::service::discovery::service_discovery::{
    ServiceDiscovery, ServiceDiscoveryState, ServiceManagerContainer,
};
use crate::service::framework::axumv1::state_controller::StateController;
use crate::service::framework::axumv1::{
    builders::{spin_h2c_server, spin_http1_server},
    framework_manager::FrameworkManager,
    logger::LogOutput,
    module::definition::ModuleDefinition,
    module::manager::ModuleManager,
};
use crate::{debug, error, info};
use axum::Router;
use bytes::Bytes;
use std::sync::Arc;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub type ServiceDiscoveryType = ServiceDiscovery<
    Arc<ServiceDiscoveryState>,
    Bytes,
    ServiceManagerContainer<Arc<ServiceDiscoveryState>, Bytes>,
>;

pub struct ServerBuilder<S>
where
    S: StateController + Clone + Sync + Send + 'static,
{
    modules: ModuleManager<S>,
    manager: FrameworkManager<S>,
    discovery: ServiceDiscoveryType,
}

impl<S> Default for ServerBuilder<S>
where
    S: StateController + Clone + Sync + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> ServerBuilder<S>
where
    S: StateController + Clone + Sync + Send + 'static,
{
    pub fn new() -> Self {
        let router = Router::new();
        let discovery_state = ServiceDiscoveryState::default();
        let mut state_controller = S::default();
        state_controller.set(discovery_state);
        let arc_discovery: Arc<ServiceDiscoveryState> = state_controller.get().unwrap();

        Self {
            modules: ModuleManager::default(),
            manager: FrameworkManager::default(),
            discovery: ServiceDiscovery::managed(arc_discovery),
        }
    }

    pub fn with_active_modules(mut self, v: Vec<String>) -> Self {
        self.modules.active_modules(v);
        self
    }

    pub fn with_log_output<O: Fn(LogOutput) -> LogOutput>(mut self, o: O) -> Self {
        self.manager.config_mut().logger_output = o(LogOutput::Full);
        self
    }

    pub fn include_subtasks_in_worker_pool(mut self, b: bool) -> Self {
        self.manager.config_mut().include_subtasks_in_worker_pool = b;
        self
    }
    //
    // pub fn add_subtask<F>(mut self, f: F) -> Self
    // where
    //     F: Future<Output = ()> + Send + 'static,
    // {
    //     self.sub_tasks.push(Box::pin(f));
    //     self
    // }
    //
    // pub fn use_http2(mut self) -> Self {
    //     self.use_http2 = true;
    //     self
    // }
    //
    // pub fn set_address(mut self, ip: &[u16]) -> Self {
    //     if ip.len() == 4 {
    //         self.address = IpAddr::V4(Ipv4Addr::new(
    //             ip[0] as u8,
    //             ip[1] as u8,
    //             ip[2] as u8,
    //             ip[3] as u8,
    //         ));
    //     } else if ip.len() != 16 {
    //         self.address = IpAddr::V6(Ipv6Addr::new(
    //             ip[0], ip[1], ip[2], ip[3], ip[4], ip[5], ip[6], ip[7],
    //         ));
    //     } else {
    //         panic!("Invalid IP address used");
    //     }
    //
    //     self
    // }

    pub fn with_bind(mut self, sock: SocketAddrs) -> Self {
        self.manager.config_mut().socket = sock;
        self
    }

    pub fn with_port_default(mut self, port: u16) -> Self {
        self.manager.config_mut().port_default = port;
        self
    }

    pub fn with_module<T: ModuleDefinition<S>>(mut self) -> Self {
        self.modules.add_module::<T>();

        self
    }

    pub fn with_trace_layer(mut self) -> Self {
        self.manager.config_mut().trace_layer = true;
        self
    }

    pub fn enable_log_service_discovery(mut self) -> Self {
        self.manager.config_mut().logger_discovery = true;
        self
    }

    pub fn add_state<T>(mut self) -> Self
    where
        T: Default + Send + Sync + 'static,
    {
        self.manager.state_mut().set(T::default());
        self
    }

    pub fn add_state_object<T>(mut self, t: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.manager.state_mut().set(t);
        self
    }

    pub fn with_worker_pool(mut self, max_workers: usize) -> Self {
        self.manager.config_mut().worker_pool = Some(max_workers);
        self
    }
    //
    // pub fn with_fallback<H, T>(mut self, handler: H) -> Self
    // where
    //     H: Handler<T, Arc<StateController>>,
    //     T: 'static,
    // {
    //     let router = Router::new();
    //     self.fallback_response = Some(router.fallback(handler));
    //     self
    // }

    fn build_inner(mut self, start_server: bool) {
        // let num_subtasks = self.sub_tasks.len();
        let body = async {
            debug!("Module Pre-Init");
            self.modules.run_pre_init();

            debug!("Creating app");
            debug!("Initializing FrameworkState");
            let framework_state = self.modules.setup_module_states(self.manager.state_mut());

            debug!("Building framework state");
            let mut framework_manager = Arc::new(self.manager.clone());
            framework_manager.set_state(framework_state.into_app_state());

            debug!("Setting up advertiser and discoverer from modules");
            self.modules
                .setup_advertiser(&mut self.discovery, &framework_manager)
                .setup_discoverer(&mut self.discovery, &framework_manager);

            debug!("Starting service discovery");

            self.discovery.serve(framework_manager.state().get());
            debug!("Initializing base router");
            let router_with_state = Router::new();

            debug!("Initializing Merger Router");
            let mut router: Router<()> = Router::new();

            debug!("Adding liveness and readiness routers");
            router = router
                .merge(self.modules.setup_liveness_router())
                .merge(self.modules.setup_readiness_router());

            debug!("Adding Module Routers");
            router = router.merge(self.modules.setup_module_routers());

            if let Some(fallback) = self.modules.fallback_response {
                debug!("Adding fallback router");
                router = router.merge(fallback);
            }

            debug!("Building App router with State");
            let app = Router::new()
                .merge(router)
                .with_state(framework_state.clone());

            debug!("Merging routers into base router");
            let mut app_with_state = router_with_state.merge(app);

            if self.manager.config().trace_layer {
                debug!("Adding Trace and Timeout Layers");
                app_with_state = app_with_state.layer((
                    TraceLayer::new_for_http(),
                    // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                    // requests don't hang forever.
                    TimeoutLayer::new(Duration::from_secs(10)),
                ));
            }

            debug!("Spawning subtasks");
            // for i in self.sub_tasks {
            //     task::spawn(i);
            // }

            debug!("Running module pre-run tasks");
            if self.modules.has_pre_run() {
                self.modules.run_pre_run();
            }

            debug!("Setting up listener socket address");
            let listener = tokio::net::TcpListener::bind(&self.manager.config().socket)
                .await
                .tap_err(|e| {
                    error!(
                        "Failed to bind to the expected socket address: {} with the error {}",
                        self.manager.config().socket,
                        e
                    )
                })
                .unwrap();

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

            if self.modules.has_post_run() {
                self.modules.run_post_run();
            }
        };

        debug!("Setting up the thread builder for tokio");
        let mut builder = tokio::runtime::Builder::new_multi_thread();

        if let Some(threads) = self.manager.config().worker_pool.as_ref() {
            if self.manager.config().include_subtasks_in_worker_pool {
                debug!("Using defined worker threads");
                builder.worker_threads(*threads);
            } else {
                debug!("Using defined worker threads");
                info!("## Discovery system threads detection not implemented");
                // TODO: Implem ent the discovery system threads detection
                builder.worker_threads(*threads);
            }
        } else {
            debug!("Using auto-lookup worker threads");
            let num_cores = num_cpus::get();
            if self.manager.config().include_subtasks_in_worker_pool {
                debug!("Using defined worker threads");
                builder.worker_threads(num_cores);
            } else {
                debug!("Using defined worker threads");
                info!("## Discovery system threads detection not implemented");
                // TODO: Implem ent the discovery system threads detection
                builder.worker_threads(num_cores);
            }
        }

        builder
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body)
    }

    pub fn build(self, s: S) {
        self.build_inner(true)
    }

    #[cfg(test)]
    pub(crate) fn build_test(self, s: S) {
        self.build_inner(false)
    }
}
