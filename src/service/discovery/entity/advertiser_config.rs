use bytes::Bytes;
use std::net::{IpAddr, Ipv4Addr};

// Configuration for Discovery
#[derive(Debug, Clone)]
pub struct AdvertiserConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub service_name: Option<String>,
    pub version: Option<String>,
    pub interval: u64,
    pub advertisement: Bytes,
}

impl AdvertiserConfig {
    // Initialize discovery config with default values
    pub fn new<A: Into<Bytes>>(
        ip: IpAddr,
        port: u16,
        service_name: Option<String>,
        version: Option<String>,
        capture_interval: u64,
        advertisement: A,
    ) -> Self {
        Self {
            ip,
            port,
            service_name,
            version,
            interval: capture_interval,
            advertisement: advertisement.into(),
        }
    }
}

impl Default for AdvertiserConfig {
    fn default() -> Self {
        AdvertiserConfig {
            port: 9999,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            service_name: None,
            version: None,
            interval: 5,
            advertisement: Bytes::new(),
        }
    }
}
