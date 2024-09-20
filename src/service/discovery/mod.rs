use crate::service::discovery::entity::Config;
use tokio::task::JoinHandle;

pub mod entity;
pub mod services;

pub trait DiscoveryService {
    fn set_service_config<O: Fn(Config) -> Config>(self, o: O) -> Self;
    fn start_discovery(self) -> (Self, JoinHandle<()>)
    where
        Self: Sized;
    fn start_broadcast(self) -> (Self, JoinHandle<()>)
    where
        Self: Sized;
}
