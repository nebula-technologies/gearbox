extern crate bytes;
extern crate core;
extern crate serde_derive;
extern crate serde_json;

pub mod entity;
#[cfg(feature = "service-discovery-loggers-impl")]
pub mod loggers;
pub mod service_binding;
pub mod service_discovery;
