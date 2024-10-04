use crate::service::discovery::entity::{Advertisement, AdvertiserConfig, DiscovererConfig};
use crate::time::DateTime;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct BroadcastBuilder {
    pub(crate) ip: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) interval: Option<usize>,
    pub(crate) service_name: Option<String>,
}

impl Default for BroadcastBuilder {
    fn default() -> Self {
        BroadcastBuilder {
            interval: Some(5),
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            service_name: Some("Log-service".to_string()),
        }
    }
}

impl BroadcastBuilder {
    pub fn set_ip(mut self, ip: IpAddr) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn set_interval(mut self, interval: usize) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn set_service_name(mut self, service_name: &str) -> Self {
        self.service_name = Some(service_name.to_string());
        self
    }

    pub fn merge(mut self, other: BroadcastBuilder) -> Self {
        self.ip = other.ip.or(self.ip);
        self.port = other.port.or(self.port);
        self.interval = other.interval.or(self.interval);
        self.service_name = other.service_name.or(self.service_name);
        self
    }

    pub fn into_discoverer(self) -> DiscovererConfig {
        DiscovererConfig {
            ip: self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: self.port.unwrap_or(9999),
            version: None,
            service_name: self.service_name,
            capture_interval: self.interval.map(|t| t as u64).unwrap_or(30),
            advert_extract: Default::default(),
        }
    }

    pub fn into_advertiser(self, message: Advertisement) -> AdvertiserConfig {
        AdvertiserConfig {
            ip: self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: self.port.unwrap_or(9999),
            interval: self.interval.map(|t| t as u64).unwrap_or(30),
            version: None,
            service_name: self.service_name,
            advertisement: Default::default(),
        }
    }
}

pub struct DiscoveryMessageBuilder {
    pub additional_info: Option<DiscoveryMessageAdditionalInfo>,
    pub ip: Option<Vec<IpAddr>>,
    pub mac: Option<String>,
    pub port: Option<u16>,
    pub service_name: Option<String>,
    pub timestamp: Option<DateTime>,
    pub version: Option<String>,
}

pub struct DiscoveryMessageAdditionalInfo {
    pub service_type: Option<String>,
    pub status: Option<String>,
}
