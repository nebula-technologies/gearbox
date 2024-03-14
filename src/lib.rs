extern crate alloc;
extern crate core;
extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate spin;
#[cfg(not(target_arch = "wasm32"))]
extern crate tokio;
extern crate tracing;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
extern crate web_sys;

pub mod common;
pub mod log;
pub mod net;
pub mod path;
pub mod rails;
pub mod storage;
pub mod time;

pub mod did;

pub use crate::log::syslog::macros::*;

#[cfg(test)]
mod tests {
    use super::*;
}
