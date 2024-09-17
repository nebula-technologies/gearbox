use crate::log::service::discovery::entity::discovery::DiscoveryMessage;
use core::net::IpAddr;
use core::num::NonZeroU16;
use std::net::Ipv4Addr;

// Main Config for the service
#[derive(Debug, Clone)]
pub struct Config {
    // This is where the service from the discovery message can be reached
    pub found_endpoint: Endpoint,
    // This is the config for the discovery service
    pub discovery: Option<DiscoveryConfig>,
    pub broadcast: Option<DiscoveryConfig>,
}

impl Config {
    pub(crate) async fn update_endpoint_form_discovery(&mut self, msg: &DiscoveryMessage) {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            found_endpoint: Endpoint::None,
            discovery: None,
            broadcast: None,
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
// Configuration for Discovery
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub service_name: Option<String>,
    pub version: Option<String>,
    pub capture_interval: u64,
}

impl DiscoveryConfig {
    // Initialize discovery config with default values
    pub fn new(
        ip: IpAddr,
        port: u16,
        service_name: Option<String>,
        version: Option<String>,
        capture_interval: u64,
    ) -> Self {
        Self {
            ip,
            port,
            service_name,
            version,
            capture_interval,
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        DiscoveryConfig {
            port: 9999,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            service_name: None,
            version: None,
            capture_interval: 5,
        }
    }
}

impl Config {
    // Create a new configuration with optional discovery config
    pub fn new(
        found_endpoint: Endpoint,
        discovery: Option<DiscoveryConfig>,
        broadcast: Option<DiscoveryConfig>,
    ) -> Self {
        Self {
            found_endpoint,
            discovery,
            broadcast,
        }
    }
}
