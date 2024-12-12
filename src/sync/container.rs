use crate::collections::HashMap;
use core::fmt::Debug;
use std::any::{Any, TypeId};
use std::sync::Arc;

pub trait CommonContainerTrait: Clone + Default + Debug + Send + Sync {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self;
    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>>;
    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>>;
    fn has<T: Any + Send + Sync>(&self) -> bool {
        self.get::<T>().is_some()
    }
}

#[derive(Clone, Debug)]
pub struct Container {
    // A map for storing application state keyed by TypeId
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    // Create a new AppState
    pub fn new(state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        Self { state }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

impl CommonContainerTrait for Container {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(t));
        self
    }

    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let t = TypeId::of::<T>();
        println!("{:?}", t);
        self.state.get(&t).and_then(|t| {
            println!("Data available");
            t.clone().downcast::<T>().ok().or_else(|| {
                println!("Downcast Failure");
                None
            })
        })
    }

    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>> {
        self.state
            .remove(&TypeId::of::<T>())
            .and_then(|t| t.downcast::<T>().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Clone, Debug, PartialEq)]
    struct TestData {
        value: String,
    }

    #[test]
    fn test_set_and_get() {
        let mut container = Container::default();

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
        let mut container = Container::default();

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
        let mut container = Container::default();

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
        let mut container = Container::default();

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
