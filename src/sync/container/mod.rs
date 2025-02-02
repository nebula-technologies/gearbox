pub mod common_key_container;
pub mod common_type_container;

#[cfg(feature = "common-merge")]
use crate::common::merge::DataMerge;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait TypeContainer: Clone + Default + Debug + Send + Sync {
    fn new() -> Self;
    fn set<T: Any + Send + Sync>(&self, t: T) -> Option<Arc<T>>;
    fn add<T: Any + Send + Sync>(&self, t: T) -> &Self {
        self.set(t);
        self
    }
    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>>;
    fn remove<T: Any + Send + Sync>(&self) -> Option<Arc<T>>;
    fn has<T: Any + Send + Sync>(&self) -> bool {
        self.get::<T>().is_some()
    }
}

#[cfg(feature = "common-merge")]
pub trait TypeContainerExtMerge: TypeContainer {
    fn update<T: DataMerge<Arc<T>> + Any + Send + Sync>(&self, t: T) -> Option<Arc<T>>;
}

pub trait KeyContainer<K, V>: Clone + Default + Debug + Send + Sync {
    fn new() -> Self;
    fn set(&self, key: K, value: V) -> Option<Arc<V>>;
    fn add(&self, key: K, value: V) -> &Self {
        self.set(key, value);
        self
    }
    fn get(&self, key: &K) -> Option<Arc<V>>;
    fn remove(&self, key: &K) -> Option<Arc<V>>;
    fn has(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

#[cfg(feature = "common-merge")]
pub trait KeyContainerExtMerge<K, V>: KeyContainer<K, V> {
    fn update(&self, key: K, other: V) -> Option<Arc<V>>;
}
