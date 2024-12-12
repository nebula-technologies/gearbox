use crate::service::discovery::discovery_type::discoverer::error::DiscovererError;
use bytes::Bytes;

pub trait DiscovererAdvertCapture<A> {
    fn advert_capture(self, b: Bytes) -> Result<A, DiscovererError>;
}
