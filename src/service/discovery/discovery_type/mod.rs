use crate::service::discovery::advertisement::AdvertisementTransformer;
use crate::service::discovery::discovery_type::broadcaster::Broadcaster;
use crate::service::discovery::discovery_type::discoverer::Discoverer;
use bytes::Bytes;
pub mod broadcaster;
pub mod discoverer;

#[derive(Clone)]
pub enum ServiceDiscoveryType<S, A = Bytes>
where
    A: Clone,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    Broadcaster(Broadcaster<A>),
    Discoverer(Discoverer<S, A>),
}

impl<S, A> ServiceDiscoveryType<S, A>
where
    A: Clone,
    Broadcaster<A>: AdvertisementTransformer<A>,
{
    pub fn is_broadcaster(&self) -> bool {
        match self {
            ServiceDiscoveryType::Broadcaster(_) => true,
            _ => false,
        }
    }

    pub fn is_discoverer(&self) -> bool {
        match self {
            ServiceDiscoveryType::Discoverer(_) => true,
            _ => false,
        }
    }
    pub fn broadcaster() -> Broadcaster<A> {
        Broadcaster::new()
    }

    pub fn discoverer() -> Discoverer<S, A> {
        Discoverer::new()
    }
}
