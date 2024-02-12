extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
#[cfg(not(target_arch = "wasm32"))]
extern crate tokio;
extern crate tracing;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
extern crate web_sys;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::env;
use std::fs::File;

pub mod common;
pub mod path;
pub mod rails;
pub mod storage;

#[cfg(test)]
mod tests {
    use super::*;
}
