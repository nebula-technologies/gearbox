use crate::common::ArcFn;
use std::sync::Arc;

pub trait ModuleDefinition {
    /// The name of the module
    const NAME: &'static str;

    /// The router for the module
    const ROUTER: fn() -> Router<Arc<FrameworkState>>;
    const NESTED: Option<&'static str> = None;

    /// The broadcaster for the module
    const BROADCAST: fn(&ServerFrameworkConfig) -> Vec<AdvertiserBuilder> = |_| Vec::new();
    const DISCOVERY: fn(&ServerFrameworkConfig) -> Vec<DiscovererBuilder> = |_| Vec::new();
    const DISCOVERY_CAPTURE: Option<fn(Arc<FrameworkState>, &Bytes)> = None;

    const STATES: fn(&mut RwFrameworkState);

    const READINESS: fn() -> Vec<ArcFn<(String, ProbeResult)>> = Vec::new;
    const LIVENESS: fn() -> Vec<ArcFn<(String, ProbeResult)>> = Vec::new;

    const PRE_RUN: fn() -> Vec<ArcFn<()>> = Vec::new;
    const POST_RUN: fn() -> Vec<ArcFn<()>> = Vec::new;
}
