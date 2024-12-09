use crate::common::ArcFn;
use crate::net::socket_addr::SocketAddrs;
use crate::service::discovery::service_discovery::{
    Broadcaster, Discoverer, ServiceDiscoveryState,
};
use crate::service::framework::axum::framework_manager::FrameworkManager;
use crate::service::framework::axum::probe::probe_result::ProbeResult;
use crate::sync::CommonContainerTrait;
use axum::Router;
use bytes::Bytes;
use std::sync::Arc;

pub trait ModuleDefinition<S>
where
    S: CommonContainerTrait + Clone + Send + Sync + 'static,
{
    /// The name of the module
    const NAME: &'static str;

    /// The router for the module
    const ROUTER: fn() -> Router<S> = || Router::new();
    const NESTED: Option<&'static str> = None;

    /// The broadcaster for the module
    const BROADCAST: fn(&FrameworkManager<S>) -> Vec<(Broadcaster<Bytes>, Option<SocketAddrs>)> =
        |_| Vec::new();
    const DISCOVERY: fn(
        &FrameworkManager<S>,
    ) -> Vec<(
        Discoverer<Arc<ServiceDiscoveryState>, Bytes>,
        Option<SocketAddrs>,
    )> = |_| Vec::new();
    const DISCOVERY_CAPTURE: Option<fn(Arc<S>, &Bytes)> = None;

    const STATES: fn(&mut S) = |_| {};

    const READINESS: fn() -> Vec<ArcFn<(String, ProbeResult)>> = Vec::new;
    const LIVENESS: fn() -> Vec<ArcFn<(String, ProbeResult)>> = Vec::new;

    const PRE_INIT: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>> = |_| Vec::new();
    const PRE_RUN: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>> = |_| Vec::new();
    const POST_RUN: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>> = |_| Vec::new();
}
