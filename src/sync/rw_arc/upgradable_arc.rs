use super::{
    compare_exchange, ReadArc, RelaxStrategy, RwArcInner, Spin, WriteArc, READER, UPGRADED, WRITER,
};
use crate::externs::{
    fmt, mem,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    sync::{atomic::Ordering, Arc},
};

pub struct UpgradableArc<T: ?Sized, R = Spin> {
    pub(super) inner: Arc<RwArcInner<T, R>>,
}

unsafe impl<T: ?Sized + Send + Sync, R> Send for UpgradableArc<T, R> {}
unsafe impl<T: ?Sized + Send + Sync, R> Sync for UpgradableArc<T, R> {}

impl<T: ?Sized, R: RelaxStrategy> UpgradableArc<T, R> {
    #[inline]
    pub fn upgrade(mut self) -> WriteArc<T, R> {
        loop {
            self = match self.try_upgrade_internal(false) {
                Ok(guard) => return guard,
                Err(e) => e,
            };

            R::relax();
        }
    }
}
impl<'rwlock, T: ?Sized, R> UpgradableArc<T, R> {
    #[inline(always)]
    fn try_upgrade_internal(self, strong: bool) -> Result<WriteArc<T, R>, Self> {
        if compare_exchange(
            &self.inner.lock,
            UPGRADED,
            WRITER,
            Ordering::Acquire,
            Ordering::Relaxed,
            strong,
        )
        .is_ok()
        {
            let inner = self.inner.clone();

            // Forget the old guard so its destructor doesn't run (before mutably aliasing data below)
            mem::forget(self);

            // Upgrade successful
            Ok(WriteArc { inner })
        } else {
            Err(self)
        }
    }

    pub fn try_upgrade(self) -> Result<WriteArc<T, R>, Self> {
        self.try_upgrade_internal(true)
    }

    pub fn downgrade(self) -> ReadArc<T, R> {
        // Reserve the read guard for ourselves
        self.inner.acquire_reader();

        let inner = self.inner.clone();

        // Dropping self removes the UPGRADED bit
        mem::drop(self);

        ReadArc { inner }
    }

    pub fn leak(this: Self) -> &'rwlock T {
        let this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &*this.inner.data.get() }
    }
}

impl<T: ?Sized + fmt::Debug, R> fmt::Debug for UpgradableArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display, R> fmt::Display for UpgradableArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized, R> Deref for UpgradableArc<T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.inner.data.get() }
    }
}

impl<T: ?Sized, R> Drop for UpgradableArc<T, R> {
    fn drop(&mut self) {
        debug_assert_eq!(
            self.inner.lock.load(Ordering::Relaxed) & (WRITER | UPGRADED),
            UPGRADED
        );
        self.inner.lock.fetch_sub(UPGRADED, Ordering::AcqRel);
    }
}
