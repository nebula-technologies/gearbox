pub mod controller;
pub mod rw_controller;

use std::any::Any;
use std::sync::Arc;

pub use controller::CommonStateController;
pub use rw_controller::RwStateController;

pub trait CommonStateContainer: Clone + Send + Sync {
    fn set<T: Any + Send + Sync>(&mut self, t: T) -> &mut Self;
    fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>>;
    fn remove<T: Any + Send + Sync>(&mut self) -> Option<Arc<T>>;
    fn has<T: Any + Send + Sync>(&self) -> bool {
        self.get::<T>().is_some()
    }
}
