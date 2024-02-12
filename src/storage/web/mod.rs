#[cfg(all(target_arch = "wasm32", feature = "web", feature = "web-storage"))]
pub mod local_storage;
