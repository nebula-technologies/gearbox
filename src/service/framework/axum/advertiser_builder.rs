use crate::net::socket_addr::SocketAddr;
use crate::service::discovery::entity::Advertisement;
use crate::service::discovery::service_discovery::{
    AdvertisementTransformer, Broadcaster, Discoverer,
};
use crate::service::framework::axum::bindable::{Bindable, BindableError};
use crate::service::framework::axum::FrameworkState;
use crate::time::DateTime;
use bytes::Bytes;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AdvertiserBuilder {
    pub(crate) ip: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) bind_ip: Option<IpAddr>,
    pub(crate) bind_port: Option<u16>,
    pub(crate) broadcast: Option<SocketAddr>,
    pub(crate) interval: Option<usize>,
    pub(crate) service_name: Option<String>,
    pub(crate) advertisement: Option<Advertisement>,
}

impl Default for AdvertiserBuilder {
    fn default() -> Self {
        AdvertiserBuilder {
            interval: Some(5),
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            bind_ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            bind_port: Some(9999),
            broadcast: None,
            service_name: Some("Log-service".to_string()),
            advertisement: None,
        }
    }
}

impl AdvertiserBuilder {
    pub fn with_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.ip = ip;
        self
    }
    pub fn set_ip(&mut self, ip: IpAddr) -> &mut Self {
        self.ip = Some(ip);
        self
    }

    pub fn with_port(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub fn with_bind_port(mut self, port: Option<u16>) -> Self {
        self.bind_port = port;
        self
    }
    pub fn set_bind_port(&mut self, port: u16) -> &mut Self {
        self.bind_port = Some(port);
        self
    }

    pub fn with_bind_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.bind_ip = ip;
        self
    }
    pub fn set_bind_ip(&mut self, ip: IpAddr) -> &mut Self {
        self.bind_ip = Some(ip);
        self
    }

    pub fn with_interval(mut self, interval: Option<usize>) -> Self {
        self.interval = interval;
        self
    }

    pub fn set_interval(&mut self, interval: usize) -> &mut Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_service_name(mut self, service_name: Option<String>) -> Self {
        self.service_name = service_name;
        self
    }

    pub fn set_service_name(&mut self, service_name: &str) -> &mut Self {
        self.service_name = Some(service_name.to_string());
        self
    }

    pub fn with_advertisement(mut self, advert: Option<Advertisement>) -> Self {
        self.advertisement = advert;
        self
    }
    pub fn set_advertisement(&mut self, advert: Advertisement) -> &mut Self {
        self.advertisement = Some(advert);
        self
    }

    pub fn with_broadcast(mut self, broadcast: Option<SocketAddr>) -> Self {
        self.broadcast = broadcast;
        self
    }

    pub fn set_broadcast(&mut self, broadcast: SocketAddr) -> &mut Self {
        self.broadcast = Some(broadcast);
        self
    }

    pub fn merge(mut self, other: AdvertiserBuilder) -> Self {
        self.ip = other.ip.or(self.ip);
        self.port = other.port.or(self.port);
        self.interval = other.interval.or(self.interval);
        self.service_name = other.service_name.or(self.service_name);
        self
    }

    pub fn into_broadcaster<A: Into<Bytes>>(
        self,
        message: Option<A>,
    ) -> Result<Bindable<Broadcaster<Bytes>>, BindableError>
    where
        Broadcaster<Bytes>: AdvertisementTransformer<Bytes>,
    {
        let mut bind = SocketAddr::default()
            .with_ip(self.bind_ip)
            .with_port(self.bind_port);

        let advertisement = self.advertisement.or_else(|| {
            Some(
                Advertisement::default()
                    .with_ip(self.ip.map(|t| vec![t]))
                    .with_port(self.port)
                    .with_service_id(self.service_name.clone()),
            )
        });

        (
            bind,
            Broadcaster::default()
                .with_interval(self.interval.map(|t| t as u64))
                .with_service_name(self.service_name)
                .with_broadcast(self.broadcast.or_else(|| {
                    SocketAddr::default()
                        .with_detect_ip_match(".*", true)
                        .as_broadcast_addr(None)
                        .ok()
                }))
                .with_advertisement(
                    advertisement
                        .map(|t| t.into())
                        .or_else(|| message.map(|t| t.into())),
                ),
        )
            .try_into()
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
