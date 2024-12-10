use crate::collections::{ConstHashMap, HashMap};
use crate::prelude::spin::RwLockWriteGuard;
use spin::{RwLockReadGuard, RwLockUpgradableGuard};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

pub struct Deferred<G, T> {
    guard: Option<G>,
    data: T,
}

impl<G, T> Deferred<G, T> {
    pub fn new(guard: G, data: T) -> Self {
        Self {
            guard: Some(guard),
            data,
        }
    }
}

impl<S, T> Deref for Deferred<S, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<S, T> DerefMut for Deferred<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<K, T> DeferredUpdate<K, T> for Deferred<RwLockUpgradableGuard<'_, HashMap<K, T>>, T>
where
    K: Eq + Hash,
    T: Clone,
{
    fn update(&mut self, key: K) {
        self.guard = self
            .guard
            .take()
            .map(|mut t| t.upgrade())
            .map(|mut t| {
                let data = self.data.clone();
                t.insert(key, data);
                t
            })
            .map(|t| t.downgrade_to_upgradeable());
    }
}

impl<K, T> DeferredUpdate<K, T> for Deferred<RwLockUpgradableGuard<'_, ConstHashMap<K, T>>, T>
where
    K: Eq + Hash,
    T: Clone,
{
    fn update(&mut self, key: K) {
        self.guard = self
            .guard
            .take()
            .map(|mut t| t.upgrade())
            .map(|mut t| {
                let data = self.data.clone();
                t.insert(key, data);
                t
            })
            .map(|t| t.downgrade_to_upgradeable());
    }
}

pub trait DeferredUpdate<K, T> {
    fn update(&mut self, key: K);
}
