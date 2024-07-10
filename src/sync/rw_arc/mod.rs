pub mod detatched_rw_arc;
pub mod hyper_read_arc;
pub mod hyper_write_arc;
pub mod read_arc;
pub mod rw_arc;
pub mod upgradable_arc;
pub mod write_arc;

pub(crate) use super::{RelaxStrategy, Spin};
use crate::externs::sync::atomic::{AtomicUsize, Ordering};
pub use detatched_rw_arc::DetachedArc;
pub use hyper_read_arc::HyperReadArc;
pub use hyper_write_arc::HyperWriteArc;
pub use read_arc::ReadArc;
pub use rw_arc::RwArc;
pub use rw_arc::RwArcInner;
pub use upgradable_arc::UpgradableArc;
pub use write_arc::WriteArc;

#[inline(always)]
fn compare_exchange(
    atomic: &AtomicUsize,
    current: usize,
    new: usize,
    success: Ordering,
    failure: Ordering,
    strong: bool,
) -> Result<usize, usize> {
    if strong {
        atomic.compare_exchange(current, new, success, failure)
    } else {
        atomic.compare_exchange_weak(current, new, success, failure)
    }
}

const READER: usize = 1 << 2;
const UPGRADED: usize = 1 << 1;
const WRITER: usize = 1;
