//! A lock that provides data access to either one writer or many readers.
use super::{
    compare_exchange, DetachedArc, HyperReadArc, HyperWriteArc, ReadArc, RelaxStrategy, Spin,
    UpgradableArc, WriteArc, READER, UPGRADED, WRITER,
};
#[cfg(feature = "with_serde")]
use crate::prelude::serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(target_arch = "wasm32")]
use crate::prelude::wasm_bindgen::{convert::*, describe::*, prelude::*};
use crate::prelude::{
    cell::UnsafeCell,
    fmt,
    marker::PhantomData,
    mem,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub struct RwArc<T: ?Sized, R = Spin> {
    pub(super) inner: Arc<RwArcInner<T, R>>,
}

impl<T, R> RwArc<T, R> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(RwArcInner {
                phantom: PhantomData,
                lock: AtomicUsize::new(0),
                data: UnsafeCell::new(data),
            }),
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let RwArcInner { data, .. } = Arc::try_unwrap(self.inner)
            .ok()
            .expect("Arc::try_unwrap failed");
        data.into_inner()
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.inner.data.get()
    }
}

impl<T: ?Sized, R: RelaxStrategy> RwArc<T, R> {
    #[inline]
    pub fn read(&self) -> ReadArc<T, R> {
        loop {
            match self.try_read() {
                Some(guard) => return guard,
                None => R::relax(),
            }
        }
    }

    #[inline]
    pub fn write(&self) -> WriteArc<T, R> {
        loop {
            match self.try_write_internal(false) {
                Some(guard) => return guard,
                None => R::relax(),
            }
        }
    }

    pub fn upgradeable_read(&self) -> UpgradableArc<T, R> {
        loop {
            match self.try_upgradeable_read() {
                Some(guard) => return guard,
                None => R::relax(),
            }
        }
    }
    #[inline]
    pub fn detach(&self) -> DetachedArc<T, R> {
        DetachedArc {
            inner: self.inner.clone(),
        }
    }
}

impl<T: ?Sized + Clone, R: RelaxStrategy> RwArc<T, R> {
    pub fn hyper_read(&self) -> HyperReadArc<T, R> {
        let data = unsafe { (*self.inner.data.get()).clone() };
        HyperReadArc {
            inner: self.inner.clone(),
            data,
        }
    }

    pub fn hyper_write(&self) -> HyperWriteArc<T, R> {
        let data = unsafe { (*self.inner.data.get()).clone() };
        HyperWriteArc {
            inner: self.inner.clone(),
            data,
        }
    }
}

impl<T: ?Sized, R> RwArc<T, R> {
    pub fn has_readers(&self) -> bool {
        self.inner.has_readers()
    }

    pub fn has_upgradeable(&self) -> bool {
        self.inner.has_upgradeable()
    }

    pub fn has_writer(&self) -> bool {
        self.inner.has_writer()
    }

    pub fn reader_count(&self) -> usize {
        self.inner.reader_count()
    }

    pub fn upgradeable_count(&self) -> usize {
        self.inner.upgradeable_count()
    }

    pub fn writer_count(&self) -> usize {
        self.inner.writer_count()
    }
    pub fn try_read(&self) -> Option<ReadArc<T, R>> {
        let value = self.inner.acquire_reader();

        // We check the UPGRADED bit here so that new readers are prevented when an UPGRADED lock is held.
        // This helps reduce writer starvation.
        if value & (WRITER | UPGRADED) != 0 {
            // Lock is taken, undo.
            self.inner.lock.fetch_sub(READER, Ordering::Release);
            None
        } else {
            Some(ReadArc {
                inner: self.inner.clone(),
            })
        }
    }

    pub unsafe fn force_read_decrement(&self) {
        debug_assert!(self.inner.lock.load(Ordering::Relaxed) & !WRITER > 0);
        self.inner.lock.fetch_sub(READER, Ordering::Release);
    }

    pub unsafe fn force_write_unlock(&self) {
        debug_assert_eq!(
            self.inner.lock.load(Ordering::Relaxed) & !(WRITER | UPGRADED),
            0
        );
        self.inner
            .lock
            .fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }

    #[inline]
    fn try_write_internal(&self, strong: bool) -> Option<WriteArc<T, R>> {
        if compare_exchange(
            &self.inner.lock,
            0,
            WRITER,
            Ordering::Acquire,
            Ordering::Relaxed,
            strong,
        )
        .is_ok()
        {
            Some(WriteArc {
                inner: self.inner.clone(),
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn try_write(&self) -> Option<WriteArc<T, R>> {
        self.try_write_internal(true)
    }

    /// Tries to obtain an upgradeable lock guard.
    #[inline]
    pub fn try_upgradeable_read(&self) -> Option<UpgradableArc<T, R>> {
        if self.inner.lock.fetch_or(UPGRADED, Ordering::Acquire) & (WRITER | UPGRADED) == 0 {
            Some(UpgradableArc {
                inner: self.inner.clone(),
            })
        } else {
            // We can't unflip the UPGRADED bit back just yet as there is another upgradeable or write lock.
            // When they unlock, they will clear the bit.
            None
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        // We know statically that there are no other references to `self`, so
        // there's no need to lock the inner lock.
        unsafe { &mut *self.inner.data.get() }
    }
}

#[cfg(feature = "with_serde")]
impl<T: ?Sized + Serialize, R: RelaxStrategy> Serialize for RwArc<T, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let detached = self.detach();
        detached.serialize(serializer)
    }
}

#[cfg(feature = "with_serde")]
impl<'de, T: ?Sized + Deserialize<'de>, R: RelaxStrategy> Deserialize<'de> for RwArc<T, R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let detached = DetachedArc::<T, R>::deserialize(deserializer)?;
        Ok(RwArc {
            inner: detached.inner,
        })
    }
}

impl<T: ?Sized, R> Clone for RwArc<T, R> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: ?Sized + fmt::Debug, R> fmt::Debug for RwArc<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_read() {
            Some(guard) => write!(f, "RwLock {{ data: ")
                .and_then(|()| (&*guard).fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "RwLock {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default, R> Default for RwArc<T, R> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, R> From<T> for RwArc<T, R> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

pub struct RwArcInner<T: ?Sized, R> {
    pub(super) phantom: PhantomData<R>,
    pub(super) lock: AtomicUsize,
    pub(super) data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send, R> Send for RwArcInner<T, R> {}
unsafe impl<T: ?Sized + Send + Sync, R> Sync for RwArcInner<T, R> {}

impl<T: ?Sized, R> RwArcInner<T, R> {
    // Acquire a read lock, returning the new lock value.
    pub(crate) fn acquire_reader(&self) -> usize {
        // An arbitrary cap that allows us to catch overflows long before they happen
        const MAX_READERS: usize = core::usize::MAX / READER / 2;

        let value = self.lock.fetch_add(READER, Ordering::Acquire);

        if value > MAX_READERS * READER {
            self.lock.fetch_sub(READER, Ordering::Relaxed);
            panic!("Too many lock readers, cannot safely proceed");
        } else {
            value
        }
    }

    // Check if there are any readers
    pub fn has_readers(&self) -> bool {
        self.reader_count() > 0
    }

    // Check if there is an upgradeable lock
    pub fn has_upgradeable(&self) -> bool {
        self.upgradeable_count() > 0
    }

    // Check if there is a writer
    pub fn has_writer(&self) -> bool {
        self.writer_count() > 0
    }

    // Get the number of readers
    pub fn reader_count(&self) -> usize {
        let state = self.lock.load(Ordering::Relaxed);
        state / READER + (state & UPGRADED) / UPGRADED
    }

    // Get the number of upgradeable locks
    pub fn upgradeable_count(&self) -> usize {
        (self.lock.load(Ordering::Relaxed) & UPGRADED) / UPGRADED
    }

    // Get the number of writers
    pub fn writer_count(&self) -> usize {
        (self.lock.load(Ordering::Relaxed) & WRITER) / WRITER
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Barrier};
    use std::thread;

    type RwArc<T> = super::RwArc<T>;

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[test]
    fn smoke() {
        let l = RwArc::new(());
        drop(l.read());
        drop(l.write());
        drop((l.read(), l.read()));
        drop(l.write());
    }

    #[test]
    fn test_rw_arc() {
        let arc = Arc::new(RwArc::new(0));
        let arc2 = arc.clone();
        let (tx, rx) = channel();

        let t = thread::spawn(move || {
            let mut lock = arc2.write();
            for _ in 0..10 {
                let tmp = *lock;
                *lock = -1;
                thread::yield_now();
                *lock = tmp + 1;
            }
            tx.send(()).unwrap();
        });

        // Readers try to catch the writer in the act
        let mut children = Vec::new();
        for _ in 0..5 {
            let arc3 = arc.clone();
            children.push(thread::spawn(move || {
                let lock = arc3.read();
                assert!(*lock >= 0);
            }));
        }

        // Wait for children to pass their asserts
        for r in children {
            assert!(r.join().is_ok());
        }

        // Wait for writer to finish
        rx.recv().unwrap();
        let lock = arc.read();
        assert_eq!(*lock, 10);

        assert!(t.join().is_ok());
    }

    #[test]
    fn test_rw_access_in_unwind() {
        let arc = Arc::new(RwArc::new(1));
        let arc2 = arc.clone();
        let _ = thread::spawn(move || -> () {
            struct Unwinder {
                i: Arc<RwArc<isize>>,
            }
            impl Drop for Unwinder {
                fn drop(&mut self) {
                    let mut lock = self.i.write();
                    *lock += 1;
                }
            }
            let _u = Unwinder { i: arc2 };
            panic!();
        })
        .join();
        let lock = arc.read();
        assert_eq!(*lock, 2);
    }

    #[test]
    fn test_rwlock_try_write() {
        use std::mem::drop;

        let lock = RwArc::new(0isize);
        let read_guard = lock.read();

        let write_result = lock.try_write();
        match write_result {
            None => (),
            Some(_) => assert!(
                false,
                "try_write should not succeed while read_guard is in scope"
            ),
        }

        drop(read_guard);
    }

    #[test]
    fn test_rw_try_read() {
        let m = RwArc::new(0);
        ::std::mem::forget(m.write());
        assert!(m.try_read().is_none());
    }

    #[test]
    fn test_into_inner() {
        let m = RwArc::new(NonCopy(10));
        assert_eq!(m.into_inner(), NonCopy(10));
    }

    #[test]
    fn test_into_inner_drop() {
        struct Foo(Arc<AtomicUsize>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let num_drops = Arc::new(AtomicUsize::new(0));
        let m = RwArc::new(Foo(num_drops.clone()));
        assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        {
            let _inner = m.into_inner();
            assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        }
        assert_eq!(num_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_force_read_decrement() {
        let m = RwArc::new(());
        ::std::mem::forget(m.read());
        ::std::mem::forget(m.read());
        ::std::mem::forget(m.read());
        assert!(m.try_write().is_none());
        unsafe {
            m.force_read_decrement();
            m.force_read_decrement();
        }
        assert!(m.try_write().is_none());
        unsafe {
            m.force_read_decrement();
        }
        assert!(m.try_write().is_some());
    }

    #[test]
    fn test_force_write_unlock() {
        let m = RwArc::new(());
        ::std::mem::forget(m.write());
        assert!(m.try_read().is_none());
        unsafe {
            m.force_write_unlock();
        }
        assert!(m.try_read().is_some());
    }

    #[test]
    fn test_upgrade_downgrade() {
        let m = RwArc::new(());
        {
            let _r = m.read();
            let upg = m.try_upgradeable_read().unwrap();
            assert!(m.try_read().is_none());
            assert!(m.try_write().is_none());
            assert!(upg.try_upgrade().is_err());
        }
        {
            let w = m.write();
            assert!(m.try_upgradeable_read().is_none());
            let _r = w.downgrade();
            assert!(m.try_upgradeable_read().is_some());
            assert!(m.try_read().is_some());
            assert!(m.try_write().is_none());
        }
        {
            let _u = m.upgradeable_read();
            assert!(m.try_upgradeable_read().is_none());
        }

        assert!(m.try_upgradeable_read().unwrap().try_upgrade().is_ok());
    }

    #[test]
    fn concurrent_reads() {
        let lock = Arc::new(RwArc::new(0));
        let barrier = Arc::new(Barrier::new(10));

        let mut handles = vec![];
        for _ in 0..10 {
            let lock = Arc::clone(&lock);
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                let read_guard = lock.read();
                assert_eq!(*read_guard, 0);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn writer_blocks_readers() {
        let lock = Arc::new(RwArc::new(0));
        let barrier = Arc::new(Barrier::new(2));

        let lock2 = Arc::clone(&lock);
        let barrier2 = Arc::clone(&barrier);
        let writer = thread::spawn(move || {
            let _write_guard = lock2.write();
            barrier2.wait();
            thread::sleep(std::time::Duration::from_millis(50));
        });

        let lock3 = Arc::clone(&lock);
        let barrier3 = Arc::clone(&barrier);
        let reader = thread::spawn(move || {
            barrier3.wait();
            let read_guard = lock3.read();
            assert_eq!(*read_guard, 0);
        });

        writer.join().unwrap();
        reader.join().unwrap();
    }

    #[test]
    fn downgrade_write_to_read() {
        let lock = RwArc::new(5);
        {
            let write_guard = lock.write();
            assert_eq!(*write_guard, 5);
            let read_guard = write_guard.downgrade();
            assert_eq!(*read_guard, 5);
        }
        {
            let read_guard = lock.read();
            assert_eq!(*read_guard, 5);
        }
    }

    #[test]
    fn downgrade_write_to_upgradeable() {
        let lock = RwArc::new(5);
        {
            let write_guard = lock.write();
            assert_eq!(*write_guard, 5);
            let upgradable_guard = write_guard.downgrade_to_upgradeable();
            assert_eq!(*upgradable_guard, 5);
        }
        {
            let read_guard = lock.read();
            assert_eq!(*read_guard, 5);
        }
    }

    #[test]
    fn upgradeable_to_write() {
        let lock = RwArc::new(5);
        {
            let upgradable_guard = lock.upgradeable_read();
            assert_eq!(*upgradable_guard, 5);
            let write_guard = upgradable_guard.upgrade();
            assert_eq!(*write_guard, 5);
        }
        {
            let read_guard = lock.read();
            assert_eq!(*read_guard, 5);
        }
    }

    #[test]
    fn recursive_read_locks() {
        let lock = Arc::new(RwArc::new(5));
        let barrier = Arc::new(Barrier::new(2));

        let lock2 = Arc::clone(&lock);
        let barrier2 = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            let read_guard1 = lock2.read();
            assert_eq!(*read_guard1, 5);

            barrier2.wait();

            let read_guard2 = lock2.read();
            assert_eq!(*read_guard2, 5);
        });

        barrier.wait();
        handle.join().unwrap();
    }

    #[test]
    fn read_to_upgradeable_fails() {
        let lock = RwArc::new(5);
        let read_guard1 = lock.read();
        println!("Acquired first read guard: {:?}", read_guard1);
        assert_eq!(*read_guard1, 5);
        println!("number of read locks: {}", lock.reader_count());

        let read_guard2 = lock.read();
        println!("Acquired second read guard: {:?}", read_guard2);
        assert_eq!(*read_guard2, 5);
        println!("number of read locks: {}", lock.reader_count());

        // This operation should fail as we can't upgrade a read lock directly to an upgradeable lock
        let upgradeable = lock.try_upgradeable_read();
        println!(
            "Attempted to upgrade read lock to upgradeable lock: {:?}",
            upgradeable
        );
        println!("Has Upgradable: {}", lock.has_upgradeable());

        let upgradeable2 = lock.try_upgradeable_read();
        println!(
            "Attempted to upgrade read lock to upgradeable lock: {:?}",
            upgradeable2
        );

        assert!(upgradeable2.is_none());

        let failed_upgrade = upgradeable.unwrap().try_upgrade();

        assert!(failed_upgrade.is_err());
    }

    // #[test]
    // fn check_lock_state() {
    //     let lock = RwArc::new(5);
    //     {
    //         let _read_guard = lock.read();
    //         assert!(lock.has_readers());
    //         assert_eq!(lock.reader_count(), 1);
    //         assert!(!lock.has_upgradeable());
    //         assert_eq!(lock.upgradeable_count(), 0);
    //         assert!(!lock.has_writer());
    //         assert_eq!(lock.writer_count(), 0);
    //     }
    //
    //     {
    //         let _upgradable_guard = lock.upgradeable_read();
    //         assert!(!lock.has_readers());
    //         assert_eq!(lock.reader_count(), 0);
    //         assert!(lock.has_upgradeable());
    //         assert_eq!(lock.upgradeable_count(), 1);
    //         assert!(!lock.has_writer());
    //         assert_eq!(lock.writer_count(), 0);
    //     }
    //
    //     {
    //         let _write_guard = lock.write();
    //         assert!(!lock.has_readers());
    //         assert_eq!(lock.reader_count(), 0);
    //         assert!(!lock.has_upgradeable());
    //         assert_eq!(lock.upgradeable_count(), 0);
    //         assert!(lock.has_writer());
    //         assert_eq!(lock.writer_count(), 1);
    //     }
    // }

    #[test]
    fn hyper_write_arc_writes_back_on_drop() {
        let lock = RwArc::new(5);
        {
            let mut hyper_write_guard = lock.hyper_write();
            hyper_write_guard.data = 10;
        }
        {
            let read_guard = lock.read();
            assert_eq!(*read_guard, 10);
        }
    }

    #[test]
    fn hyper_read_arc() {
        let lock = RwArc::new(5);
        let hyper_read_guard = lock.hyper_read();
        assert_eq!(*hyper_read_guard, 5);
    }

    #[test]
    fn doc_write_lock() {
        let lock = RwArc::new(0);
        {
            let mut write_guard = lock.write();
            *write_guard += 1;
            println!("Write: {}", *write_guard);
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod wasm_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_rw_arc() {
        let arc = RwArc::new(0);
        let arc2 = arc.clone();

        {
            let read_guard = arc.read();
            assert_eq!(*read_guard, 0);
        }

        {
            let mut write_guard = arc2.write();
            *write_guard = 1;
        }

        {
            let read_guard = arc.read();
            assert_eq!(*read_guard, 1);
        }
    }

    #[wasm_bindgen_test]
    fn test_rwlock_try_write() {
        let lock = RwArc::new(0isize);
        let read_guard = lock.read();

        let write_result = lock.try_write();
        assert!(write_result.is_none());

        drop(read_guard);

        let write_result = lock.try_write();
        assert!(write_result.is_some());
    }

    #[wasm_bindgen_test]
    fn hyper_write_arc_writes_back_on_drop() {
        let lock = RwArc::new(5);
        {
            let mut hyper_write_guard = lock.hyper_write();
            hyper_write_guard.data = 10;
        }
        {
            let read_guard = lock.read();
            assert_eq!(*read_guard, 10);
        }
    }

    #[wasm_bindgen_test]
    fn hyper_read_arc() {
        let lock = RwArc::new(5);
        let hyper_read_guard = lock.hyper_read();
        assert_eq!(*hyper_read_guard, 5);
    }

    #[wasm_bindgen_test]
    fn concurrent_reads() {
        let lock = Arc::new(RwArc::new(0));
        let barrier = Arc::new(Barrier::new(10));

        let mut handles = vec![];
        for _ in 0..10 {
            let lock = Arc::clone(&lock);
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                let read_guard = lock.read();
                assert_eq!(*read_guard, 0);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[wasm_bindgen_test]
    fn writer_blocks_readers() {
        let lock = Arc::new(RwArc::new(0));
        let barrier = Arc::new(Barrier::new(2));

        let lock2 = Arc::clone(&lock);
        let barrier2 = Arc::clone(&barrier);
        let writer = thread::spawn(move || {
            let _write_guard = lock2.write();
            barrier2.wait();
            thread::sleep(std::time::Duration::from_millis(50));
        });

        let lock3 = Arc::clone(&lock);
        let barrier3 = Arc::clone(&barrier);
        let reader = thread::spawn(move || {
            barrier3.wait();
            let read_guard = lock3.read();
            assert_eq!(*read_guard, 0);
        });

        writer.join().unwrap();
        reader.join().unwrap();
    }
}
