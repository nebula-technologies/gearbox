use crate::service::discovery::entity::{Advertisement, AdvertiserConfig, DiscovererConfig};
use crate::time::DateTime;
use bytes::Bytes;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct AdvertiserBuilder {
    pub(crate) ip: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) bind_port: Option<u16>,
    pub(crate) interval: Option<usize>,
    pub(crate) service_name: Option<String>,
    pub(crate) advertisement: Advertisement,
}

impl Default for AdvertiserBuilder {
    fn default() -> Self {
        AdvertiserBuilder {
            interval: Some(5),
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            bind_port: Some(9999),
            service_name: Some("Log-service".to_string()),
            advertisement: Advertisement::default(),
        }
    }
}

impl AdvertiserBuilder {
    pub fn set_ip(mut self, ip: IpAddr) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn set_bind_port(mut self, port: u16) -> Self {
        self.bind_port = Some(port);
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

    pub fn set_advertisement(mut self, advert: Advertisement) -> Self {
        self.advertisement = advert;
        self
    }

    pub fn merge(mut self, other: AdvertiserBuilder) -> Self {
        self.ip = other.ip.or(self.ip);
        self.port = other.port.or(self.port);
        self.interval = other.interval.or(self.interval);
        self.service_name = other.service_name.or(self.service_name);
        self
    }

    pub fn into_advertiser<A: Into<Bytes>>(self, message: Option<A>) -> AdvertiserConfig {
        AdvertiserConfig {
            ip: self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            bind_port: self.bind_port.unwrap_or(9999),
            port: self.port.unwrap_or(9998),
            interval: self.interval.map(|t| t as u64).unwrap_or(30),
            version: None,
            service_name: self.service_name,
            advertisement: message
                .map(|t| t.into())
                .unwrap_or(self.advertisement.into()),
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
