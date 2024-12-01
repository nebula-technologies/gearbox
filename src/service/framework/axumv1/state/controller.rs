use crate::collections::HashMap;
use crate::service::framework::axumv1;
use crate::service::framework::axumv1::StateController;
use crate::sync::rw_arc::RwArc;
use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use core::fmt::Debug;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CommonStateController {
    // A map for storing application state keyed by TypeId
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl CommonStateController {
    // Create a new AppState
    pub fn new(state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        Self { state }
    }
}

impl Default for CommonStateController {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

impl StateController for CommonStateController {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(t));
        self
    }

    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.state
            .get(&TypeId::of::<T>())
            .and_then(|t| t.downcast_ref::<Arc<T>>())
            .map(Clone::clone)
    }

    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>> {
        self.state
            .remove(&TypeId::of::<T>())
            .and_then(|t| t.downcast::<T>().ok())
    }
}

pub struct DynoState<T> {
    pub state: Arc<T>,
}

impl<T> DynoState<T> {
    pub fn new(state: Arc<T>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl<S, T> FromRequestParts<S> for DynoState<T>
where
    S: Send + Sync,
    T: 'static + Send + Sync,
    Arc<CommonStateController>: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the StateController from the Axum state
        let controller = Arc::<CommonStateController>::from_ref(state);

        // Try to get the requested type from the controller
        if let Some(state) = controller.get::<T>() {
            Ok(DynoState { state })
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}
