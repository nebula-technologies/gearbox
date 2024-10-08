use super::discovery::Advertisement;
use crate::common::merge::DataMerge;
use crate::service::discovery::entity::advertiser_config::AdvertiserConfig;
use crate::service::discovery::entity::{Advertiser, Discoverer, DiscovererConfig};
use core::net::IpAddr;

// Main Config for the service
#[derive(Debug, Clone)]
pub struct Config {
    // This is where the service from the discovery message can be reached
    pub found_endpoint: Endpoint,
    // This is the config for the discovery service
    pub discoverer: Option<DiscovererConfig>,
    pub advertiser: Option<AdvertiserConfig>,
}

impl Config {
    pub(crate) async fn update_endpoint_from_discovery(&mut self, msg: &Advertisement) {
        if let (Some(ip), Some(port), Some(resolvable_name)) =
            (msg.ip.clone(), msg.port.clone(), msg.service_name.clone())
        {
            self.found_endpoint = Endpoint::Http(HttpEndpoint {
                resolvable_name,
                port,
                ip,
            })
        }
    }
}

impl DataMerge<Config> for Config {
    fn data_merge(&mut self, other: Config) -> &mut Self {
        self.found_endpoint = other.found_endpoint;
        self.discoverer = other.discoverer.or(self.discoverer.clone());
        self.advertiser = other.advertiser.or(self.advertiser.clone());

        self
    }
}
impl DataMerge<AdvertiserConfig> for Config {
    fn data_merge(&mut self, other: AdvertiserConfig) -> &mut Self {
        self.advertiser.data_merge(other);

        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            found_endpoint: Endpoint::None,
            discoverer: None,
            advertiser: None,
        }
    }
}

// Enum representing the type of endpoint
#[derive(Debug, Clone)]
pub enum Endpoint {
    None,
    Http(HttpEndpoint),
}

// Structure for an HTTP Endpoint
#[derive(Debug, Clone)]
pub struct HttpEndpoint {
    pub ip: Vec<IpAddr>,
    pub resolvable_name: String,
    pub port: u16,
}

impl Config {
    // Create a new configuration with optional discovery config
    pub fn new(
        found_endpoint: Endpoint,
        discovery: Option<DiscovererConfig>,
        broadcast: Option<AdvertiserConfig>,
    ) -> Self {
        Self {
            found_endpoint,
            discoverer: discovery,
            advertiser: broadcast,
        }
    }
}
