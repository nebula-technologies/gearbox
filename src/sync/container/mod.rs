pub mod common_key_container;
pub mod common_type_container;

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait TypeContainer<T>: Clone + Default + Debug + Send + Sync
where
    T: Any + Send + Sync,
{
    fn new() -> Self;
    fn set(&mut self, t: T) -> &mut Self;
    fn get(&self) -> Option<Arc<T>>;
    fn remove(&mut self) -> Option<Arc<T>>;
    fn has(&self) -> bool {
        self.get().is_some()
    }
}

pub trait KeyContainer<K, V>: Clone + Default + Debug + Send + Sync {
    fn new() -> Self;
    fn set(&mut self, key: K, value: V) -> &mut Self;
    fn get(&self, key: &K) -> Option<Arc<V>>;
    fn remove(&mut self, key: &K) -> Option<Arc<V>>;
    fn has(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}
