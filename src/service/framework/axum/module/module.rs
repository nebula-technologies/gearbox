use crate::common::ArcFn;
use crate::net::socket_addr::SocketAddrs;
use crate::service::discovery::service_discovery::{
    Broadcaster, Discoverer, ServiceDiscoveryState,
};
use crate::service::framework::axum::framework_manager::FrameworkManager;
use crate::service::framework::axum::module::definition::ModuleDefinition;
use crate::service::framework::axum::probe::probe_result::ProbeResult;
use crate::sync::CommonContainerTrait;
use axum::Router;
use bytes::Bytes;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Module<S>
where
    S: CommonContainerTrait + Clone + Send + Sync,
{
    pub(crate) name: &'static str,
    pub(crate) router: fn() -> Router<S>,
    pub(crate) state: fn(&mut S),
    pub(crate) nested: Option<&'static str>,
    pub(crate) broadcast:
        fn(&FrameworkManager<S>) -> Vec<(Broadcaster<Bytes>, Option<SocketAddrs>)>,
    pub(crate) discovery: fn(
        &FrameworkManager<S>,
    ) -> Vec<(
        Discoverer<Arc<ServiceDiscoveryState>, Bytes>,
        Option<SocketAddrs>,
    )>,
    // pub(crate) discovery_capture: Option<fn(Arc<S>, &Bytes)>,
    pub(crate) readiness: fn() -> Vec<ArcFn<(String, ProbeResult)>>,
    pub(crate) liveness: fn() -> Vec<ArcFn<(String, ProbeResult)>>,
    pub(crate) pre_init: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>>,
    pub(crate) pre_run: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>>,
    pub(crate) post_run: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>>,
}

impl<S> Module<S>
where
    S: CommonContainerTrait + Clone + Send + Sync + 'static,
{
    pub fn from_definition<T: ModuleDefinition<S>>() -> Self {
        Module {
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
        }
    }
}
