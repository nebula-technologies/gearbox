//! The `rw_arc` module provides a robust and flexible mechanism for handling concurrent data access in Rust, allowing either multiple readers or a single writer to access the data. This module is designed to provide high performance and safety in multi-threaded environments. It includes various structures such as `ReadArc`, `DetachedArc`, `HyperWriteArc`, `UpgradableArc`, and `WriteArc`, each tailored for specific concurrent access patterns. It combines the functionality of `Arc` and `RwLock` while avoiding the constraints of lifetimes, offering more versatility and some inherent risks.
//!
//! ## Overview
//!
//! ### Features
//!
//! - **Reader-Writer Locks**: Allows multiple readers or a single writer to access the data, ensuring efficient data access without contention.
//! - **Upgradeable Locks**: Enables a lock to be upgraded from read to write access, allowing more flexible and efficient lock management.
//! - **DetachedArcs**: Provides a way to create detached instances of locks that can be later attached to the main lock, enabling flexible lock management across different contexts.
//! - **HyperLocks (Write-on-Destruct)**: Efficient write operations that ensure data is written back to the main storage upon destruction, optimizing for cases where write operations are deferred.
//!
//! ### Benefits and Risks
//!
//! - **Versatility**: By combining the features of `Arc` and `RwLock`, `rw_arc` allows for cross-clones and other operations that are not constrained by lifetimes. This increases the flexibility of concurrent programming.
//! - **Danger**: The absence of lifetime constraints can make the `rw_arc` module more dangerous to use, as it relies on manual guarantees of safety that are normally enforced by the Rust compiler's borrow checker.
//!
//! ## Architecture
//!
//! ```mermaid
//! graph TD;
//!     RwArc -->|Has| RwArcInner;
//!     RwArcInner -->|Contains| Arc;
//!     Arc -->|References| Data;
//!     RwArc -->|Provides| ReadArc;
//!     RwArc -->|Provides| WriteArc;
//!     RwArc -->|Provides| UpgradableArc;
//!     RwArc -->|Provides| DetachedArc;
//!     RwArc -->|Provides| HyperReadArc;
//!     RwArc -->|Provides| HyperWriteArc;
//! ```
//!
//! ## Usage Example
//!
//! ```rust
//! use gearbox::sync::rw_arc::RwArc;
//! use std::sync::Arc;
//!
//! let lock: RwArc<i32> = RwArc::new(0);
//!
//! // Read access
//! {
//!     let read_guard = lock.read();
//!     println!("Read: {}", *read_guard);
//! }
//!
//! // Write access
//! {
//!     let mut write_guard = lock.write();
//!     *write_guard += 1;
//!     println!("Write: {}", *write_guard);
//! }
//!
//! // Upgradeable read//!
//! {
//!     let upgradable_guard = lock.upgradeable_read();
//!     let write_guard = upgradable_guard.upgrade();
//!     println!("Upgradeable to Write: {}", *write_guard);
//! }
//!
//! ```
//!
//! ## Detailed Descriptions
//!
//! ### `ReadArc`
//! A read-only lock guard that allows multiple concurrent readers. Even if the underlying data is dropped, `ReadArc` will continue to hold the data, making it a true clone of the existing data.
//!
//! #### Example
//! ```rust
//! use gearbox::sync::rw_arc::RwArc;
//! use std::sync::Arc;
//!
//! let lock : RwArc<i32>= RwArc::new(0);
//!
//! let read_arc = lock.read();
//! println!("ReadArc value: {}", *read_arc);
//! ```
//!
//! ### `DetachedArc`
//! A detached lock guard that can be attached to an existing `RwArc`. This allows for flexible lock management where locks can be created and attached later.
//!
//! #### Example
//! ```rust
//! use gearbox::sync::rw_arc::{DetachedArc, RwArc};
//!
//! let rw_arc: RwArc<i32> = RwArc::new(10);
//! let detached_arc = DetachedArc::new(20);
//!
//! if let Some(read_arc) = detached_arc.attach_read(&rw_arc) {
//!     println!("Attached ReadArc value: {}", *read_arc);
//! }
//! ```
//!
//! ### `HyperWriteArc`
//! A write lock guard that ensures data written is properly stored back into the main data structure upon destruction. This is known as Write-on-Destruct (WOD) and is designed for efficient deferred write operations.
//!
//! #### Example
//! ```rust
//! use gearbox::sync::rw_arc::RwArc;
//!
//! let lock: RwArc<i32> = RwArc::new(5);
//! {
//!     let mut hyper_write_guard = lock.hyper_write();
//!     *hyper_write_guard = 10;
//! }
//! {
//!     let read_guard = lock.read();
//!     assert_eq!(*read_guard, 10);
//! }
//! ```
//!
//! ### `UpgradableArc`
//! An upgradable read lock guard that can be upgraded to a write lock, allowing more flexible and efficient lock management.
//!
//! #### Example
//! ```rust
//! use gearbox::sync::rw_arc::RwArc;
//!
//! let lock: RwArc<i32> = RwArc::new(5);
//! {
//!     let upgradable_guard = lock.upgradeable_read();
//!     let write_guard = upgradable_guard.upgrade();
//!     assert_eq!(*write_guard, 5);
//! }
//! {
//!     let read_guard = lock.read();
//!     assert_eq!(*read_guard, 5);
//! }
//! ```
//!
//! ### `WriteArc`
//! A write lock guard that allows modifying the data while ensuring that the modifications are safely committed.
//!
//! #### Example
//! ```rust
//! use gearbox::sync::rw_arc::RwArc;
//!
//! let lock: RwArc<i32> = RwArc::new(0);
//! {
//!     let mut write_guard = lock.write();
//!     *write_guard += 1;
//!     println!("Write: {}", *write_guard);
//! }
//! ```
//!

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
