use crate::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::Arc;

pub struct RwAppState {
    pub(crate) state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl RwAppState {
    pub fn add_default<T: Any + Default + Send + Sync>(&mut self) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(T::default()));
        self
    }
    pub fn add<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(t));
        self
    }

    pub(crate) fn into_app_state(self) -> AppState {
        AppState { state: self.state }
    }
}

impl Default for RwAppState {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    // A map for storing application state keyed by TypeId
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl AppState {
    // Create a new AppState
    pub fn new(state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        Self { state }
    }

    // Get a reference to a value in the state by type
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.state
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}
