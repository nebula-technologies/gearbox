use super::{compare_exchange, RelaxStrategy, RwArcInner, Spin, UPGRADED, WRITER};
use crate::prelude::{
    ops::{Deref, DerefMut},
    sync::{atomic::Ordering, Arc},
};

pub struct HyperWriteArc<T: ?Sized + Clone, R: RelaxStrategy = Spin> {
    pub(super) inner: Arc<RwArcInner<T, R>>,
    pub(super) data: T,
}

impl<T: ?Sized + Clone, R: RelaxStrategy> Drop for HyperWriteArc<T, R> {
    fn drop(&mut self) {
        // Try to acquire the write lock
        loop {
            match compare_exchange(
                &self.inner.lock,
                0,
                WRITER,
                Ordering::Acquire,
                Ordering::Relaxed,
                false,
            ) {
                Ok(_) => break,
                Err(_) => R::relax(),
            }
        }

        // Write the data back to the inner storage
        unsafe {
            *self.inner.data.get() = self.data.clone();
        }

        // Writer is responsible for clearing both WRITER and UPGRADED bits.
        // The UPGRADED bit may be set if an upgradeable lock attempts an upgrade while this lock is held.
        self.inner
            .lock
            .fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }
}

impl<T: ?Sized + Clone, R: RelaxStrategy> Deref for HyperWriteArc<T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        &self.data
    }
}
impl<T: ?Sized + Clone, R: RelaxStrategy> DerefMut for HyperWriteArc<T, R> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: We know statically that only we are referencing data
        &mut self.data
    }
}
