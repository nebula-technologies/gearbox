use crate::collections::HashMap;
use crate::common::ArcFn;
use crate::net::socket_addr::{SocketAddressTryWithBuilder, SocketAddrs};
use crate::service::discovery::service_discovery::{
    ServiceDiscovery, ServiceDiscoveryState, ServiceManagerContainerArc, ServiceManagerTrait,
};
use crate::service::framework::axum::framework_manager::FrameworkManager;
use crate::service::framework::axum::module::definition::ModuleDefinition;
use crate::service::framework::axum::module::module::Module;
use crate::service::framework::axum::probe::probe_result::ProbeResult;
use crate::service::framework::axum::probe::status_response::StatusResponse;
use crate::sync::CommonContainerTrait;
use crate::{debug, info};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use bytes::Bytes;
use std::marker::PhantomData;
use std::sync::Arc;

pub(crate) type ServiceDiscoveryType = ServiceDiscovery<
    Arc<ServiceDiscoveryState>,
    Bytes,
    ServiceManagerContainerArc<Arc<ServiceDiscoveryState>, Bytes>,
>;

#[derive(Debug, Clone)]
pub struct ModuleManager<S>
where
    S: CommonContainerTrait + Clone + Send + Sync + 'static,
{
    modules: HashMap<String, Arc<Module<S>>>,
    active_modules: Vec<String>,
    phantom: PhantomData<S>,
}

impl<S: CommonContainerTrait + Clone + Send + Sync + 'static> ModuleManager<S> {
    pub fn new() -> Self {
        ModuleManager {
            modules: HashMap::new(),
            active_modules: Vec::new(),
            phantom: PhantomData::default(),
        }
    }

    pub fn activate_module(&mut self, v: &str) -> &mut Self {
        self.active_modules.push(v.to_string());
        self
    }

    pub fn active_modules(&mut self, mut v: Vec<String>) -> &mut Self {
        self.active_modules.append(&mut v);
        self
    }

    pub fn add_module<T: ModuleDefinition<S>>(&mut self) -> &mut Self {
        self.modules.insert(
            T::NAME.to_string(),
            Arc::new(Module {
                pre_init: T::PRE_INIT,
                pre_run: T::PRE_RUN,
                post_run: T::POST_RUN,
                name: T::NAME,
                state: T::STATES,
                nested: T::NESTED,
                broadcast: T::BROADCAST,
                discovery: T::DISCOVERY,
                // discovery_capture: T::DISCOVERY_CAPTURE,
                router: T::ROUTER,
                readiness: T::READINESS,
                liveness: T::LIVENESS,
            }),
        );
        self
    }

    /// TODO: Currently as config we are using ServerBuilder, we should move all config out and use a proper configurator

    pub(crate) fn setup_advertiser(
        &mut self,
        service: &mut ServiceDiscoveryType,
        config: &FrameworkManager<S>,
    ) -> &mut Self {
        for module in self.active_modules.clone() {
            if let Some(module) = self.modules.get(&module) {
                let func = module.broadcast;
                let func_output = func(config);
                let func_len = func_output.len();
                for (bcast, addr) in func_output {
                    let addr_fixed = addr
                        .map(|t| {
                            t.as_builder()
                                .if_default_port(9999)
                                .if_try_capture_ips()
                                .build()
                                .expect("Failed to build the socket addr")
                        })
                        .unwrap_or(
                            SocketAddrs::with()
                                .if_default_port(9999)
                                .if_try_capture_ips()
                                .build()
                                .expect("Failed to build SocketAddrs"),
                        );
                    service.add_broadcaster(addr_fixed, bcast);
                }
                info!(
                    "Added {} broadcasters for module: {}",
                    func_len, module.name
                );
            };
        }
        self
    }

    pub(crate) fn setup_discoverer(
        &mut self,
        service: &mut ServiceDiscoveryType,
        config: &FrameworkManager<S>,
    ) -> &mut Self {
        for module in self.active_modules.clone() {
            if let Some(module) = self.modules.get(&module) {
                let func = module.discovery;
                let func_output = func(config);
                let func_len = func_output.len();
                for (discover, addr) in func_output {
                    let addr_fixed = addr
                        .map(|t| {
                            t.as_builder()
                                .if_default_port(9999)
                                .if_try_capture_ips()
                                .build()
                                .expect("Failed to build the socket addr")
                        })
                        .unwrap_or(
                            SocketAddrs::with()
                                .default_port(9999)
                                .try_capture_ips()
                                .build()
                                .expect("Failed to build SocketAddrs"),
                        );
                    service.add_discoverer(addr_fixed, discover.clone());
                }
                info!("Added {} discoverers for module: {}", func_len, module.name);
            };
        }
        self
    }

    pub(crate) fn has_pre_run(&mut self, config: &FrameworkManager<S>) -> bool {
        let mut avail_func = Vec::new();
        for module in self.active_modules.clone() {
            self.modules.get(&module).map(|t| {
                let func = t.pre_run;
                func(config).into_iter().for_each(|_| avail_func.push(()))
            });
        }
        !avail_func.is_empty()
    }

    pub(crate) fn run_pre_run(&self, config: &FrameworkManager<S>) -> &Self {
        for module in self.active_modules.clone() {
            self.modules.get(&module).map(|t| {
                let func = t.pre_run;
                func(config).into_iter().for_each(|t| t())
            });
        }
        self
    }

    pub(crate) fn has_post_run(&self, config: &FrameworkManager<S>) -> bool {
        let mut avail_func = Vec::new();
        for module in &self.active_modules.clone() {
            self.modules.get(module).map(|t| {
                let func = t.post_run;
                func(config).into_iter().for_each(|_| avail_func.push(()))
            });
        }
        !avail_func.is_empty()
    }

    pub(crate) fn run_post_run(&self, config: &FrameworkManager<S>) -> &Self {
        for module in &self.active_modules.clone() {
            self.modules.get(module).map(|t| {
                let func = t.post_run;
                func(config).into_iter().for_each(|t| t())
            });
        }
        self
    }

    // pub(crate) fn has_pre_init(&self, config: &FrameworkManager<S>) -> bool {
    //     let mut avail_func = Vec::new();
    //     for module in &self.active_modules.clone() {
    //         self.modules.get(module).map(|t| {
    //             let func = t.pre_init;
    //             func(config).into_iter().for_each(|_| avail_func.push(()))
    //         });
    //     }
    //     !avail_func.is_empty()
    // }

    pub(crate) fn run_pre_init(&self, config: &FrameworkManager<S>) -> &Self {
        for module in &self.active_modules.clone() {
            self.modules.get(module).map(|t| {
                let func = t.pre_init;
                func(config).into_iter().for_each(|t| t())
            });
        }
        self
    }

    pub(crate) fn setup_liveness_router(&self) -> Router<S> {
        let mut probes = Vec::new();
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                let readiness_funcs = module.liveness;
                let module_probes = readiness_funcs();
                probes.push((module_name.clone(), module_probes));
            }
        }
        self.router_config("/health/liveness", probes)
    }

    pub(crate) fn setup_readiness_router(&self) -> Router<S> {
        let mut probes = Vec::new();
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                let readiness_funcs = module.readiness;
                let module_probes = readiness_funcs();
                probes.push((module_name.clone(), module_probes));
            }
        }
        self.router_config("/health/readiness", probes)
    }

    pub(crate) fn setup_module_routers(&self) -> Router<S> {
        let mut router = Router::new();
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                if let Some(nested) = module.nested {
                    router = router.nest(nested, (module.router)());
                } else {
                    router = router.merge((module.router)());
                }
            }
        }

        router
    }

    pub(crate) fn setup_module_states(&self, app_state: &mut S) {
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                (module.state)(app_state);
            }
        }
    }

    fn router_config(
        &self,
        path: &str,
        probes: Vec<(String, Vec<ArcFn<(String, ProbeResult)>>)>,
    ) -> Router<S> {
        let router = Router::new();
        router.route(
            path,
            get(|State(_state): State<S>| async move {
                let mut module_status_map = HashMap::new();
                for (module_name, vec_func) in probes {
                    let mut module_status = Vec::new();
                    for func in vec_func {
                        let (name, status) = func();
                        module_status.push(StatusResponse { name, status });
                    }
                    module_status_map.insert(module_name, module_status);
                }
                StatusCode::OK
            }),
        )
    }
}

impl<S> Default for ModuleManager<S>
where
    S: CommonContainerTrait + Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        ModuleManager::new()
    }
}
