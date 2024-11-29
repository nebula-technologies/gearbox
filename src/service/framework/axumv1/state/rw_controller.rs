use crate::collections::HashMap;
use crate::service::framework::axumv1::StateController;
use crate::sync::rw_arc::RwArc;
use std::any::{Any, TypeId};
use std::sync::Arc;

pub struct RwStateController {
    pub(crate) state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl RwStateController {
    pub fn add_default<T: Any + Default + Send + Sync>(&mut self) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(T::default()));
        self
    }
    pub fn add<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(t));
        self
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.state
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<Arc<T>>())
            .map(|t| t.clone())
    }

    pub(crate) fn into_app_state(self) -> StateController {
        StateController::new(self.state)
    }
}

impl Default for RwStateController {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}
