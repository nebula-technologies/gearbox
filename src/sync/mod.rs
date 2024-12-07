pub mod relax;

pub mod deferred;
#[cfg(feature = "sync-rw-arc")]
pub mod rw_arc;

pub(crate) use relax::{RelaxStrategy, Spin};
