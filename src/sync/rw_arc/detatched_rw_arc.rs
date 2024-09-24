use super::{ReadArc, RelaxStrategy, RwArc, RwArcInner, Spin, WriteArc};
#[cfg(feature = "with_serde")]
use crate::externs::serde::{Deserialize, Serialize};
use crate::externs::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicUsize, Arc},
};
#[cfg(target_arch = "wasm32")]
use crate::{
    externs::wasm_bindgen::{__rt::IntoJsResult, convert::*, describe::*, prelude::*},
    serde::wasm_bindgen::to_value,
};

#[derive(Clone)]
pub struct DetachedArc<T: ?Sized, R = Spin> {
    pub(super) inner: Arc<RwArcInner<T, R>>,
}

impl<T, R: RelaxStrategy> DetachedArc<T, R> {
    pub fn new(data: T) -> Self {
        let inner = Arc::new(RwArcInner {
            phantom: PhantomData,
            lock: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        });
        DetachedArc { inner }
    }

    pub fn attach_read(self, original: &RwArc<T, R>) -> Option<ReadArc<T, R>> {
        let writer = original.try_write()?;
        let updated_writer = writer.from_detached(self);

        Some(updated_writer.downgrade())
    }

    pub fn attach_write(self, original: &RwArc<T, R>) -> Option<WriteArc<T, R>> {
        let writer = original.try_write()?;
        let updated_writer = writer.from_detached(self);

        Some(updated_writer)
    }
}

impl<T, R> Deref for DetachedArc<T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.inner.data.get() }
    }
}

impl<T, R> DerefMut for DetachedArc<T, R> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.data.get() }
    }
}

#[cfg(feature = "with_serde")]
impl<T: ?Sized + Serialize, R: RelaxStrategy> Serialize for DetachedArc<T, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = unsafe { &*self.inner.data.get() };
        data.serialize(serializer)
    }
}

#[cfg(feature = "with_serde")]
impl<'de, T: Deserialize<'de>, R: RelaxStrategy> Deserialize<'de> for DetachedArc<T, R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = T::deserialize(deserializer)?;
        Ok(Self::new(data))
    }
}

#[cfg(target_arch = "wasm32")]
impl<T: WasmDescribe, R: RelaxStrategy> WasmDescribe for DetachedArc<T, R> {
    fn describe() {
        <T as WasmDescribe>::describe();
    }
}

#[cfg(target_arch = "wasm32")]
impl<T: FromWasmAbi + WasmDescribe, R: RelaxStrategy> FromWasmAbi for DetachedArc<T, R> {
    type Abi = <T as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        let data = T::from_abi(js);
        Self::new(data)
    }
}

/// TODO! This needs to be implemented better to remove the need for the clone!
#[cfg(target_arch = "wasm32")]
impl<T: Clone + IntoWasmAbi + WasmDescribe, R: RelaxStrategy> IntoWasmAbi for DetachedArc<T, R> {
    type Abi = u32;

    fn into_abi(self) -> Self::Abi {
        self.inner.data.get() as u32
    }
}
