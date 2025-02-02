use crate::collections::HashMap;
use crate::sync::{CommonKeyContainer, TypeContainer};
#[cfg(feature = "common-merge")]
use crate::{common::merge::DataMerge, sync::container::TypeContainerExtMerge};
use core::fmt::Debug;
use std::any::{Any, TypeId};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct CommonTypeContainer {
    // A map for storing application state keyed by TypeId
    state: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl CommonTypeContainer {
    // Create a new AppState
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CommonTypeContainer {
    fn default() -> Self {
        Self {
            state: RwLock::new(HashMap::new()),
        }
    }
}

impl Clone for CommonTypeContainer {
    fn clone(&self) -> Self {
        // First, try to acquire the read lock
        if let Ok(read_guard) = self.state.read() {
            // If successful, clone the data safely
            let cloned_map = read_guard
                .iter()
                .map(|(key, value)| (key.clone(), Arc::clone(value)))
                .collect();

            return CommonTypeContainer {
                state: RwLock::new(cloned_map),
            };
        }

        // If acquiring the read lock fails, bypass the lock unsafely
        unsafe {
            eprintln!("Read lock failed. Bypassing the lock unsafely.");

            // Get a raw pointer to the inner RwLock
            let raw_inner =
                &self.state as *const RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>;

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

            CommonTypeContainer {
                state: RwLock::new(cloned_map),
            }
        }
    }
}

impl TypeContainer for CommonTypeContainer {
    fn new() -> Self {
        Self::default()
    }

    fn set<T: Any + Send + Sync>(&self, value: T) -> Option<Arc<T>> {
        self.state
            .write()
            .map_err(|e| e.to_string())
            // .log(error!(Err, message: "Failed to get write lock"))
            .ok()
            .and_then(|mut t| t.insert(TypeId::of::<T>(), Arc::new(value)))
            .and_then(|t| t.downcast::<T>().ok())
    }

    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let key = TypeId::of::<T>();
        self.state
            .read()
            .map_err(|e| e.to_string())
            // .log(error!(Err, message: "Failed to get read lock: {:?}"))
            .ok()
            .and_then(|t| t.get(&key).cloned())
            .and_then(|t| t.downcast::<T>().ok())
    }

    fn remove<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.state
            .write()
            .map_err(|e| e.to_string())
            // .log(error!(Err, message: "Failed to get write lock"))
            .ok()
            .and_then(|mut t| t.remove(&TypeId::of::<T>()))
            .and_then(|t| t.downcast::<T>().ok())
    }
}

#[cfg(feature = "common-merge")]
impl TypeContainerExtMerge for CommonTypeContainer {
    fn update<T: DataMerge<Arc<T>> + Any + Send + Sync>(&self, mut other: T) -> Option<Arc<T>> {
        let new_value = if let Some(value) = self.get::<T>() {
            other.data_merge(value);
            other
        } else {
            other
        };

        self.set(new_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tracing_subscriber::{fmt, EnvFilter};

    #[derive(Clone, Debug, PartialEq)]
    struct TestData {
        value: String,
    }

    fn logger_setup() {
        // Set up a tracing subscriber for logs
        let subscriber = fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .finish();
        tracing::subscriber::set_global_default(subscriber).ok();
    }

    #[test]
    fn test_set_and_get() {
        logger_setup();
        let mut container = CommonTypeContainer::default();

        let data = TestData {
            value: "Hello, world!".to_string(),
        };

        // Insert data into the container
        container.set(data.clone());

        // Retrieve data from the container
        let retrieved: Option<Arc<TestData>> = container.get();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, data.value);
    }

    #[test]
    fn test_remove() {
        logger_setup();
        let mut container = CommonTypeContainer::default();

        let data = TestData {
            value: "Hello, world!".to_string(),
        };

        // Insert data into the container
        container.set(data.clone());

        // Remove data from the container
        let removed: Option<Arc<TestData>> = container.remove();

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().value, data.value);

        // Ensure the data is no longer in the container
        let retrieved: Option<Arc<TestData>> = container.get();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_has() {
        logger_setup();
        let mut container = CommonTypeContainer::default();

        let data = TestData {
            value: "Hello, world!".to_string(),
        };

        // Initially, the container should not have the data
        assert!(!container.has::<TestData>());

        // Insert data into the container
        container.set(data);

        // Now, the container should have the data
        assert!(container.has::<TestData>());

        // Remove the data
        container.remove::<TestData>();

        // Ensure the container no longer has the data
        assert!(!container.has::<TestData>());
    }

    #[test]
    fn test_multiple_types() {
        logger_setup();
        let mut container = CommonTypeContainer::default();

        let string_data = "String data".to_string();
        let int_data = 42;

        // Insert multiple types into the container
        container.set(string_data.clone());
        container.set(int_data);

        // Retrieve and verify data of different types
        let retrieved_string: Option<Arc<String>> = container.get();
        let retrieved_int: Option<Arc<i32>> = container.get();

        assert!(retrieved_string.is_some());
        assert_eq!(*retrieved_string.unwrap(), string_data);

        assert!(retrieved_int.is_some());
        assert_eq!(*retrieved_int.unwrap(), 42);
    }
}
