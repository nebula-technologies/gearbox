pub mod relax;

#[cfg(feature = "sync-rw-arc")]
pub mod rw_arc;

pub(crate) use relax::{RelaxStrategy, Spin};
