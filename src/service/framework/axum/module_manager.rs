use crate::collections::HashMap;
use crate::service::framework::axum::{
    BoxFn, DiscovererBuilder, FrameworkState, RwFrameworkState, ServerBuilder,
};
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::discovery::service_discovery::ServiceDiscovery;
use crate::service::framework::axum::advertiser_builder::AdvertiserBuilder;
use crate::service::framework::axum::server_framework_config::ServerFrameworkConfig;
use crate::{debug, error, info};
use axum::http::StatusCode;
use bytes::Bytes;

pub trait ModuleDefinition {
    const NAME: &'static str;
    const ROUTER: fn() -> Router<Arc<FrameworkState>>;
    const NESTED: Option<&'static str> = None;
    const BROADCAST: fn(&ServerFrameworkConfig) -> Vec<AdvertiserBuilder> = |_| Vec::new();
    const DISCOVERY: fn(&ServerFrameworkConfig) -> Vec<DiscovererBuilder> = |_| Vec::new();
    const DISCOVERY_CAPTURE: Option<fn(Arc<FrameworkState>, &Bytes)> = None;
    const STATES: fn(&mut RwFrameworkState);
    const READINESS: fn() -> Vec<BoxFn<(String, ProbeResult)>> = Vec::new;
    const LIVENESS: fn() -> Vec<BoxFn<(String, ProbeResult)>> = Vec::new;
    const PRE_RUN: fn() -> Vec<BoxFn<()>> = Vec::new;
    const POST_RUN: fn() -> Vec<BoxFn<()>> = Vec::new;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleStatusResponse(HashMap<String, Vec<StatusResponse>>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponse {
    status: ProbeResult,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProbeResult {
    Success,
    Failure,
    SuccessWith(String),
    FailureWith(String),
}

#[derive(Debug, Clone)]
pub struct Module {
    name: &'static str,
    router: fn() -> Router<Arc<FrameworkState>>,
    state: fn(&mut RwFrameworkState),
    nested: Option<&'static str>,
    broadcast: fn(&ServerFrameworkConfig) -> Vec<AdvertiserBuilder>,
    discovery: fn(&ServerFrameworkConfig) -> Vec<DiscovererBuilder>,
    discovery_capture: Option<fn(Arc<FrameworkState>, &Bytes)>,
    readiness: fn() -> Vec<BoxFn<(String, ProbeResult)>>,
    liveness: fn() -> Vec<BoxFn<(String, ProbeResult)>>,
    pre_run: fn() -> Vec<BoxFn<()>>,
    post_run: fn() -> Vec<BoxFn<()>>,
}

impl<T: ModuleDefinition> From<T> for Module {
    fn from(_: T) -> Self {
        Module {
            pre_run: T::PRE_RUN,
            post_run: T::POST_RUN,
            name: T::NAME,
            state: T::STATES,
            nested: T::NESTED,
            broadcast: T::BROADCAST,
            discovery: T::DISCOVERY,
            discovery_capture: T::DISCOVERY_CAPTURE,
            router: T::ROUTER,
            readiness: T::READINESS,
            liveness: T::LIVENESS,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleManager {
    modules: HashMap<String, Arc<Module>>,
    active_modules: Vec<String>,
}

impl ModuleManager {
    pub fn new() -> Self {
        ModuleManager {
            modules: HashMap::new(),
            active_modules: Vec::new(),
        }
    }

    pub fn active_modules(&mut self, mut v: Vec<String>) -> &mut Self {
        self.active_modules.append(&mut v);
        self
    }

    pub fn add_module<T: ModuleDefinition>(&mut self) -> &mut Self {
        self.modules.insert(
            T::NAME.to_string(),
            Arc::new(Module {
                pre_run: T::PRE_RUN,
                post_run: T::POST_RUN,
                name: T::NAME,
                state: T::STATES,
                nested: T::NESTED,
                broadcast: T::BROADCAST,
                discovery: T::DISCOVERY,
                discovery_capture: T::DISCOVERY_CAPTURE,
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
        service: &mut ServiceDiscovery<Arc<FrameworkState>, Bytes>,
        config: &ServerFrameworkConfig,
    ) -> &mut Self {
        for module in self.active_modules.clone() {
            if let Some(module) = self.modules.get(&module) {
                let func = module.broadcast;
                let func_output = func(config);
                let func_len = func_output.len();
                for t in func_output {
                    if let Ok(bindable) = t.into_broadcaster::<Bytes>(None) {
                        service.add_broadcaster(bindable.bind_owned(), bindable.into_data());
                    } else {
                        error!("Failed to add broadcaster for module: {}", module.name);
                    }
                }
                debug!(
                    "Added {} broadcasters for module: {}",
                    func_len, module.name
                );
            };
        }
        self
    }

    pub(crate) fn setup_discoverer(
        &mut self,
        service: &mut ServiceDiscovery<Arc<FrameworkState>, Bytes>,
        config: &ServerFrameworkConfig,
    ) -> &mut Self {
        for module in self.active_modules.clone() {
            if let Some(module) = self.modules.get(&module) {
                let func = module.discovery;
                let func_output = func(config);
                let func_len = func_output.len();
                for t in func_output {
                    if let Ok(bindable) = t.into_discoverer() {
                        service.add_discoverer(bindable.bind_owned(), bindable.into_data());
                    } else {
                        error!("Failed to add discoverer for module: {}", module.name);
                    }
                }
                debug!("Added {} discoverers for module: {}", func_len, module.name);
            };
        }
        self
    }

    pub(crate) fn has_pre_run(&mut self) -> bool {
        let mut avail_func = Vec::new();
        for module in self.active_modules.clone() {
            self.modules.get(&module).map(|t| {
                let func = t.pre_run;
                func().into_iter().for_each(|_| avail_func.push(()))
            });
        }
        !avail_func.is_empty()
    }

    pub(crate) fn run_pre_run(&self) -> &Self {
        for module in self.active_modules.clone() {
            self.modules.get(&module).map(|t| {
                let func = t.pre_run;
                func().into_iter().for_each(|t| t())
            });
        }
        self
    }

    pub(crate) fn has_post_run(&self) -> bool {
        let mut avail_func = Vec::new();
        for module in &self.active_modules.clone() {
            self.modules.get(module).map(|t| {
                let func = t.post_run;
                func().into_iter().for_each(|_| avail_func.push(()))
            });
        }
        !avail_func.is_empty()
    }

    pub(crate) fn run_post_run(&self) -> &Self {
        for module in &self.active_modules.clone() {
            self.modules.get(module).map(|t| {
                let func = t.post_run;
                func().into_iter().for_each(|t| t())
            });
        }
        self
    }

    pub(crate) fn setup_liveness_router(&self) -> Router<Arc<FrameworkState>> {
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

    pub(crate) fn setup_readiness_router(&self) -> Router<Arc<FrameworkState>> {
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

    pub(crate) fn setup_module_routers(&self) -> Router<Arc<FrameworkState>> {
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

    pub(crate) fn setup_module_states(
        &self,
        mut app_state: RwFrameworkState,
    ) -> Arc<FrameworkState> {
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                (module.state)(&mut app_state);
            }
        }

        Arc::new(FrameworkState::new(app_state.state))
    }

    fn router_config(
        &self,
        path: &str,
        probes: Vec<(String, Vec<BoxFn<(String, ProbeResult)>>)>,
    ) -> Router<Arc<FrameworkState>> {
        let mut router = Router::new();
        router.route(
            path,
            get(|State(state): State<Arc<FrameworkState>>| async move {
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

impl Default for ModuleManager {
    fn default() -> Self {
        ModuleManager::new()
    }
}
