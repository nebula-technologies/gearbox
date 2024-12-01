use std::any::Any;
use std::sync::Arc;

pub trait StateController: Default + Clone {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self;

    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>>;

    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>>;
}
