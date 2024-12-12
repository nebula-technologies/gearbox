use crate::service::discovery::discovery_type::discoverer::error::DiscovererError;

pub trait DiscovererAdvertValidation {
    fn validate_advert(self) -> Result<(), DiscovererError>;
}
