use crate::collections::HashMap;
use crate::service::framework::axumv1::state::CommonStateContainer;
use crate::sync::rw_arc::RwArc;
use core::fmt::Debug;
use std::any::{Any, TypeId};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct StateController {
    // A map for storing application state keyed by TypeId
    state: RwArc<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl StateController {
    // Create a new AppState
    pub fn new(state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        let state = RwArc::new(state);
        Self { state }
    }
}

impl Default for StateController {
    fn default() -> Self {
        Self {
            state: RwArc::new(HashMap::new()),
        }
    }
}

impl CommonStateContainer for StateController {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.write().insert(TypeId::of::<T>(), Arc::new(t));
        self
    }

    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.state
            .read()
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<Arc<T>>())
            .map(|t| t.clone())
    }

    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>> {
        self.state
            .write()
            .remove(&TypeId::of::<T>())
            .and_then(|t| t.downcast().ok())
    }
}
