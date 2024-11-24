use crate::collections::HashMap;
use crate::sync::rw_arc::RwArc;
use bytes::Bytes;
use spin::RwLock;
use std::any::{Any, TypeId};
use std::sync::Arc;

pub struct RwFrameworkState {
    pub(crate) state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl RwFrameworkState {
    pub fn add_default<T: Any + Default + Send + Sync>(&mut self) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(T::default()));
        self
    }
    pub fn add<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(t));
        self
    }

    pub(crate) fn into_app_state(self) -> FrameworkState {
        FrameworkState {
            state: RwArc::new(self.state),
        }
    }
}

impl Default for RwFrameworkState {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

pub trait FrameworkStateContainer: Clone + Send + Sync {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self;
    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>>;
    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>>;
    fn has<T: Any + Send + Sync>(&self) -> bool {
        self.get::<T>().is_some()
    }
}

#[derive(Clone)]
pub struct FrameworkState {
    // A map for storing application state keyed by TypeId
    state: RwArc<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl FrameworkState {
    // Create a new AppState
    pub fn new(state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        let state = RwArc::new(state);
        Self { state }
    }
}

impl Default for FrameworkState {
    fn default() -> Self {
        Self {
            state: RwArc::new(HashMap::new()),
        }
    }
}

impl FrameworkStateContainer for FrameworkState {
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
