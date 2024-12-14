use crate::collections::HashMap;
use crate::rails::tracing::syslog::RailsSyslog;
use crate::sync::KeyContainer;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct CommonKeyContainer<K> {
    // A map for storing application state keyed by TypeId
    state: RwLock<HashMap<K, Arc<dyn Any + Send + Sync>>>,
}

impl<K> CommonKeyContainer<K> {
    // Create a new AppState
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K> Default for CommonKeyContainer<K> {
    fn default() -> Self {
        Self {
            state: RwLock::new(HashMap::new()),
        }
    }
}

impl<K, V> KeyContainer<K, V> for CommonKeyContainer<K>
where
    V: Any + Send + Sync + Clone + Debug,
    K: std::cmp::Eq + std::hash::Hash + Clone + Debug + Send + Sync,
{
    fn new() -> Self {
        Self::default()
    }

    fn set(&self, k: K, v: V) -> Option<V> {
        self.state
            .write()
            .log(error!(Err, "Failed to get write lock: {:?}"))
            .log(debug!(Ok, "Got access to write lock: {:?}"))
            .ok()
            .and_then(|mut t| t.insert(k, Arc::new(v)))
    }

    fn get(&self, k: &K) -> Option<Arc<V>> {
        println!("{:?}", k);
        self.state
            .read()
            .log(error!(Err, "Failed to get read lock: {:?}"))
            .log(debug!(Ok, "Got access to read lock: {:?}"))
            .and_then(|state| state.get(&k))
            .ok()
            .cloned()
            .and_then(|t| {
                t.downcast::<V>()
                    .log(error!(Err, "Failed to cast type: {}"))
                    .ok()
            })
    }

    fn remove(&self, k: &K) -> Option<Arc<V>> {
        self.state
            .write()
            .log(error!(Err, "Failed to get write lock: {:?}"))
            .log(debug!(Ok, "Got access to write lock: {:?}"))
            .ok()
            .and_then(|mut rwl| rwl.remove(&k))
            .and_then(|t| {
                t.downcast::<V>()
                    .log(error!(Err, "Failed to cast type: {}"))
                    .ok()
            })
    }
}
