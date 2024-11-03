use crate::common::ip_range::IpRanges;
use crate::service::discovery::entity::Advertisement;
use crate::service::discovery::service_discovery::Discoverer;
use crate::service::framework::axum::FrameworkState;
use crate::time::DateTime;
use bytes::Bytes;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct DiscovererBuilder {
    pub(crate) ip: Option<IpRanges>,
    pub(crate) port: Option<u16>,
    pub(crate) interval: Option<usize>,
    pub(crate) service_name: Option<String>,
    pub(crate) advertisement: Advertisement,
}

impl Default for DiscovererBuilder {
    fn default() -> Self {
        DiscovererBuilder {
            interval: Some(5),
            ip: Some(IpRanges::default()),
            port: Some(9999),
            service_name: Some("Log-service".to_string()),
            advertisement: Advertisement::default(),
        }
    }
}

impl DiscovererBuilder {
    pub fn set_ip<T: Into<IpRanges>>(mut self, ip: T) -> Self {
        self.ip = Some(ip.into());
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

    pub fn set_advertisement(mut self, advert: Advertisement) -> Self {
        self.advertisement = advert;
        self
    }

    pub fn merge(mut self, other: DiscovererBuilder) -> Self {
        self.ip = other.ip.or(self.ip);
        self.port = other.port.or(self.port);
        self.interval = other.interval.or(self.interval);
        self.service_name = other.service_name.or(self.service_name);
        self
    }

    pub fn into_discoverer(self) -> Discoverer<FrameworkState, Bytes> {
        Discoverer::new()
            .with_interval(self.interval.map(|t| t as u64))
            .with_service_name(self.service_name)
            .with_ip(self.ip)
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
