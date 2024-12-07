use crate::prelude::spin::RwLockWriteGuard;
use std::ops::{Deref, DerefMut};

pub struct Deferred<G, T> {
    guard: G,
    data: *mut T,
}

impl<G, T> Deferred<G, T> {
    pub fn new(guard: G, data: &mut T) -> Self {
        Self { guard, data }
    }
}

impl<S, T> Deref for Deferred<S, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<S, T> DerefMut for Deferred<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data }
    }
}
