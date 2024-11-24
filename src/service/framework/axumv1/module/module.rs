use crate::common::ArcFn;
use crate::service::framework::axum::server_framework_config::ServerFrameworkConfig;
use crate::service::framework::axum::{FrameworkState, RwFrameworkState};
use crate::service::framework::axumv1::module::definition::ModuleDefinition;
use axum::Router;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Module {
    name: &'static str,
    router: fn() -> Router<Arc<FrameworkState>>,
    state: fn(&mut RwFrameworkState),
    nested: Option<&'static str>,
    broadcast: fn(&ServerFrameworkConfig) -> Vec<AdvertiserBuilder>,
    discovery: fn(&ServerFrameworkConfig) -> Vec<DiscovererBuilder>,
    discovery_capture: Option<fn(Arc<FrameworkState>, &Bytes)>,
    readiness: fn() -> Vec<ArcFn<(String, ProbeResult)>>,
    liveness: fn() -> Vec<ArcFn<(String, ProbeResult)>>,
    pre_run: fn() -> Vec<ArcFn<()>>,
    post_run: fn() -> Vec<ArcFn<()>>,
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
