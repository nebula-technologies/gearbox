#[cfg(feature = "common-boxed-future")]
pub mod boxed_future;
#[cfg(feature = "common-merge")]
pub mod merge;
#[cfg(feature = "common-process")]
pub mod process;
pub mod serde_checks;
#[cfg(feature = "common-try-default")]
pub mod try_default;
#[cfg(feature = "common-vec-ext")]
pub mod vec_ext;

#[cfg(feature = "common-ips")]
pub use crate::net::ips::get_ips;
#[cfg(feature = "common-boxed-future")]
pub use boxed_future::BoxedFuture;
#[cfg(feature = "common-process")]
pub use process::id as process_id;
use std::sync::Arc;
#[cfg(feature = "common-try-default")]
pub use try_default::TryDefault;

pub type ArcFn<T> = Arc<dyn Fn() -> T + Send + Sync>;
