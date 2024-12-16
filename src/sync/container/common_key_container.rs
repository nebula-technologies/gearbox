use crate::collections::HashMap;
// use crate::error;
use crate::rails::tracing::syslog::RailsSyslog;
use crate::sync::{CommonTypeContainer, KeyContainer};
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
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

impl<K> Clone for CommonKeyContainer<K>
where
    K: Clone + Debug + Send + Sync + std::cmp::Eq + std::hash::Hash,
{
    fn clone(&self) -> Self {
        // First, try to acquire the read lock
        if let Ok(read_guard) = self.state.read() {
            // If successful, clone the data safely
            let cloned_map = read_guard
                .iter()
                .map(|(key, value)| (key.clone(), Arc::clone(value)))
                .collect();

            return CommonKeyContainer {
                state: RwLock::new(cloned_map),
            };
        }

        // If acquiring the read lock fails, bypass the lock unsafely
        unsafe {
            eprintln!("Read lock failed. Bypassing the lock unsafely.");

            // Get a raw pointer to the inner RwLock
            let raw_inner = &self.state as *const RwLock<HashMap<K, Arc<dyn Any + Send + Sync>>>;

            // Forcefully access the underlying data
            let cloned_map = match (*raw_inner).try_read() {
                Ok(read_guard) => read_guard
                    .iter()
                    .map(|(key, value)| (key.clone(), Arc::clone(value)))
                    .collect(),
                Err(_) => {
                    eprintln!("Failed to bypass lock using try_read.");
                    HashMap::new()
                }
            };

            CommonKeyContainer {
                state: RwLock::new(cloned_map),
            }
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

    fn set(&self, k: K, v: V) -> Option<Arc<V>> {
        self.state
            .write()
            // .log(error!(Err, "Failed to get write lock: {:?}"))
            // .log(debug!(Ok, "Got access to write lock: {:?}"))
            .ok()
            .and_then(|mut t| t.insert(k, Arc::new(v)))
            .and_then(|t| {
                t.downcast::<V>()
                    // .log(error!(Err, "Failed to cast type: {}"))
                    .ok()
            })
    }

    fn get(&self, k: &K) -> Option<Arc<V>> {
        println!("{:?}", k);
        self.state
            .read()
            // .log(error!(Err, "Failed to get read lock: {:?}"))
            // .log(debug!(Ok, "Got access to read lock: {:?}"))
            .ok()
            .and_then(|state| state.get(&k).cloned())
            .and_then(|t| {
                t.downcast::<V>()
                    // .log(error!(Err, "Failed to cast type: {}"))
                    .ok()
            })
    }

    fn remove(&self, k: &K) -> Option<Arc<V>> {
        self.state
            .write()
            // .log(error!(Err, "Failed to get write lock: {:?}"))
            // .log(debug!(Ok, "Got access to write lock: {:?}"))
            .ok()
            .and_then(|mut rwl| rwl.remove(&k))
            .and_then(|t| {
                t.downcast::<V>()
                    // .log(error!(Err, message: "Failed to cast type"))
                    .ok()
            })
    }
}
