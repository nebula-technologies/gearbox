use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct DiscoveryBuilder {
    pub(crate) ip: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) interval: Option<usize>,
    pub(crate) service_name: Option<String>,
}

impl Default for DiscoveryBuilder {
    fn default() -> Self {
        DiscoveryBuilder {
            interval: Some(5),
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            service_name: Some("Log-service".to_string()),
        }
    }
}

impl DiscoveryBuilder {
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
}
