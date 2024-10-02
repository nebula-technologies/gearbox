use crate::collections::HashMap;
use crate::service::framework::axum::{AppState, BoxFn, RwAppState};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_derive::{Deserialize, Serialize};
use spin::RwLock;
use std::sync::Arc;

use axum::http::StatusCode;

pub trait ModuleDefinition {
    const NAME: &'static str;
    const ROUTER: fn() -> Router<Arc<AppState>>;
    const STATES: fn(&mut RwAppState);
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
    name: String,
    router: fn() -> Router<Arc<AppState>>,
    state: fn(&mut RwAppState),
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
            name: T::NAME.to_string(),
            state: T::STATES,
            router: T::ROUTER,
            readiness: T::READINESS,
            liveness: T::LIVENESS,
        }
    }
}

#[derive(Debug)]
pub struct ModuleManager {
    modules: HashMap<String, Module>,
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
            Module {
                pre_run: T::PRE_RUN,
                post_run: T::POST_RUN,
                name: T::NAME.to_string(),
                state: T::STATES,
                router: T::ROUTER,
                readiness: T::READINESS,
                liveness: T::LIVENESS,
            },
        );
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

    pub(crate) fn setup_liveness_router(&self) -> Router<Arc<AppState>> {
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

    pub(crate) fn setup_readiness_router(&self) -> Router<Arc<AppState>> {
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

    pub(crate) fn setup_module_routers(&self) -> Router<Arc<AppState>> {
        let mut router = Router::new();
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                router = router.merge((module.router)());
            }
        }
        router
    }

    pub(crate) fn setup_module_states(&self, mut app_state: RwAppState) -> Arc<AppState> {
        for module_name in &self.active_modules.clone() {
            if let Some(module) = self.modules.get(module_name) {
                (module.state)(&mut app_state);
            }
        }

        Arc::new(AppState::new(app_state.state))
    }

    fn router_config(
        &self,
        path: &str,
        probes: Vec<(String, Vec<BoxFn<(String, ProbeResult)>>)>,
    ) -> Router<Arc<AppState>> {
        let mut router = Router::new();
        router.route(
            path,
            get(|State(state): State<Arc<AppState>>| async move {
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
