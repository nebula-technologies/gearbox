use crate::common::merge::DataMerge;
use bytes::Bytes;
use std::net::{IpAddr, Ipv4Addr};

// Configuration for Discovery
#[derive(Debug, Clone)]
pub struct DiscovererConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub service_name: Option<String>,
    pub version: Option<String>,
    pub capture_interval: u64,
    pub advert_extract: Bytes,
}

impl DiscovererConfig {
    // Initialize discovery config with default values
    pub fn new(
        ip: IpAddr,
        port: u16,
        service_name: Option<String>,
        version: Option<String>,
        capture_interval: u64,
        advert_extract: Bytes,
    ) -> Self {
        Self {
            ip,
            port,
            service_name,
            version,
            capture_interval,
            advert_extract,
        }
    }
}

impl Default for DiscovererConfig {
    fn default() -> Self {
        DiscovererConfig {
            port: 9999,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            service_name: None,
            version: None,
            capture_interval: 5,
            advert_extract: Bytes::new(),
        }
    }
}

impl DataMerge<DiscovererConfig> for DiscovererConfig {
    fn data_merge(&mut self, other: DiscovererConfig) -> &mut Self {
        self.ip = other.ip;
        self.port = other.port;
        self.service_name = other.service_name.or(self.service_name.clone());
        self.version = other.version.or(self.version.clone());
        self.capture_interval = other.capture_interval;
        self.advert_extract = other.advert_extract.clone();

        self
    }
}
