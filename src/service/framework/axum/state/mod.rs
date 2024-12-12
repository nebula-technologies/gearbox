use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use core::fmt::Debug;

use crate::sync::{CommonContainerTrait, Container};
use std::sync::Arc;

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
    Arc<Container>: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the StateController from the Axum state
        let controller = Arc::<Container>::from_ref(state);

        // Try to get the requested type from the controller
        if let Some(state) = controller.get::<T>() {
            Ok(DynoState { state })
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}
