use super::{compare_exchange, RelaxStrategy, RwArcInner, Spin, READER, UPGRADED, WRITER};
use crate::externs::{
    fmt,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    sync::{atomic::Ordering, Arc},
};

pub struct ReadArc<T: ?Sized, R> {
    pub(super) inner: Arc<RwArcInner<T, R>>,
}

unsafe impl<T: ?Sized + Sync, R> Send for ReadArc<T, R> {}
unsafe impl<T: ?Sized + Sync, R> Sync for ReadArc<T, R> {}

impl<'rwlock, T: ?Sized, R> ReadArc<T, R> {
    #[inline]
    pub fn leak(this: Self) -> &'rwlock T {
        let this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &*this.inner.data.get() }
    }
}

impl<T: ?Sized + fmt::Debug, R> fmt::Debug for ReadArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display, R> fmt::Display for ReadArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized, R> Deref for ReadArc<T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.inner.data.get() }
    }
}

impl<T: ?Sized, R> Drop for ReadArc<T, R> {
    fn drop(&mut self) {
        debug_assert!(self.inner.lock.load(Ordering::Relaxed) & !(WRITER | UPGRADED) > 0);
        self.inner.lock.fetch_sub(READER, Ordering::Release);
    }
}
