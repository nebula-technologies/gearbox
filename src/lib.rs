//#![no_std]
extern crate alloc;
#[cfg(feature = "net-signature")]
extern crate base64;
#[cfg(feature = "net-signature")]
extern crate bs58;
extern crate core;
extern crate hashbrown;
#[cfg(feature = "net-signature")]
extern crate hex;
#[cfg(feature = "net-signature")]
extern crate hmac;
#[cfg(feature = "http")]
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
#[cfg(feature = "net-signature")]
extern crate sha2;
#[cfg(feature = "net-signature")]
extern crate simple_serde;
extern crate spin;
#[cfg(not(target_arch = "wasm32"))]
extern crate tokio;
extern crate tracing;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
extern crate web_sys;

pub mod common;
pub mod did;
pub mod error;
pub mod log;
pub mod net;
pub mod path;
pub mod rails;
pub mod storage;
pub mod time;

#[allow(unused_imports)]
pub use crate::log::syslog::macros::*;

#[cfg(test)]
mod tests {
    use super::*;
}
