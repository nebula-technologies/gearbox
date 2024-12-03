use crate::common::ArcFn;
use crate::net::socket_addr::SocketAddrs;
use crate::service::discovery::service_discovery::{
    Broadcaster, Discoverer, ServiceDiscoveryState,
};
use crate::service::framework::axumv1::framework_manager::FrameworkManager;
use crate::service::framework::axumv1::probe::probe_result::ProbeResult;
use crate::service::framework::axumv1::state::controller::CommonStateController;
use crate::service::framework::axumv1::state::rw_controller::RwStateController;
use crate::service::framework::axumv1::FrameworkConfig;
use axum::Router;
use bytes::Bytes;
use std::net::IpAddr;
use std::sync::Arc;

pub trait ModuleDefinition {
    /// The name of the module
    const NAME: &'static str;

    /// The router for the module
    const ROUTER: fn() -> Router<Arc<CommonStateController>> = || Router::new();
    const NESTED: Option<&'static str> = None;

    /// The broadcaster for the module
    const BROADCAST: fn(&FrameworkManager) -> Vec<(Broadcaster<Bytes>, Option<SocketAddrs>)> =
        |_| Vec::new();
    const DISCOVERY: fn(
        &FrameworkManager,
    ) -> Vec<(
        Discoverer<Arc<ServiceDiscoveryState>, Bytes>,
        Option<SocketAddrs>,
    )> = |_| Vec::new();
    const DISCOVERY_CAPTURE: Option<fn(Arc<CommonStateController>, &Bytes)> = None;

    const STATES: fn(&mut RwStateController) = |_| {};

    const READINESS: fn() -> Vec<ArcFn<(String, ProbeResult)>> = Vec::new;
    const LIVENESS: fn() -> Vec<ArcFn<(String, ProbeResult)>> = Vec::new;

    const PRE_INIT: fn() -> Vec<ArcFn<()>> = Vec::new;
    const PRE_RUN: fn() -> Vec<ArcFn<()>> = Vec::new;
    const POST_RUN: fn() -> Vec<ArcFn<()>> = Vec::new;
}