use super::{
    DetachedArc, ReadArc, RelaxStrategy, RwArcInner, Spin, UpgradableArc, UPGRADED, WRITER,
};
use crate::prelude::{
    fmt, mem,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    sync::{atomic::Ordering, Arc},
};

pub struct WriteArc<T: ?Sized, R = Spin> {
    pub(super) inner: Arc<RwArcInner<T, R>>,
}
impl<T, R: RelaxStrategy> WriteArc<T, R> {
    pub fn from_detached(self, detached: DetachedArc<T, R>) -> Self {
        let inner = detached.inner;
        let mut self_data = self.inner.data.get();
        let mut detach_data = inner.data.get();
        // unsafe {
        mem::swap(&mut self_data, &mut detach_data);
        // }
        self
    }
}

unsafe impl<T: ?Sized + Send + Sync, R> Send for WriteArc<T, R> {}
unsafe impl<T: ?Sized + Send + Sync, R> Sync for WriteArc<T, R> {}

impl<T: ?Sized + fmt::Debug, R> fmt::Debug for WriteArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display, R> fmt::Display for WriteArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized, R> Drop for WriteArc<T, R> {
    fn drop(&mut self) {
        debug_assert_eq!(self.inner.lock.load(Ordering::Relaxed) & WRITER, WRITER);

        // Writer is responsible for clearing both WRITER and UPGRADED bits.
        // The UPGRADED bit may be set if an upgradeable lock attempts an upgrade while this lock is held.
        self.inner
            .lock
            .fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }
}

impl<T: ?Sized, R> Deref for WriteArc<T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.inner.data.get() }
    }
}

impl<T: ?Sized, R> DerefMut for WriteArc<T, R> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: We know statically that only we are referencing data
        unsafe { &mut *self.inner.data.get() }
    }
}

impl<'rwlock, T: ?Sized, R> WriteArc<T, R> {
    pub fn downgrade(self) -> ReadArc<T, R> {
        // Reserve the read guard for ourselves
        self.inner.acquire_reader();

        let inner = self.inner.clone();

        // Dropping self removes the UPGRADED bit
        mem::drop(self);

        ReadArc { inner }
    }

    pub fn downgrade_to_upgradeable(self) -> UpgradableArc<T, R> {
        debug_assert_eq!(
            self.inner.lock.load(Ordering::Acquire) & (WRITER | UPGRADED),
            WRITER
        );

        // Reserve the read guard for ourselves
        self.inner.lock.store(UPGRADED, Ordering::Release);

        let inner = self.inner.clone();

        // Dropping self removes the UPGRADED bit
        mem::forget(self);

        UpgradableArc { inner }
    }

    pub fn leak(this: Self) -> &'rwlock mut T {
        let this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &mut *this.inner.data.get() }
    }
}
