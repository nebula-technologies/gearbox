use crate::net::socket_addr::SocketAddr;
use std::fmt::{Debug, Formatter};
use std::net::IpAddr;

#[derive(Clone)]
pub struct Broadcaster<A>
where
    A: Clone,
{
    pub(crate) id: Option<String>,
    pub(crate) ip: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) broadcast: Option<SocketAddr>,
    pub(crate) service_name: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) interval: Option<u64>,
    pub(crate) advertisement: Option<A>,
}

impl<A> Broadcaster<A>
where
    A: Clone,
{
    pub fn new() -> Self {
        Broadcaster {
            id: None,
            ip: None,
            port: None,
            broadcast: SocketAddr::default_addr().as_broadcast_addr(None).ok(),
            service_name: None,
            version: None,
            interval: Some(5),
            advertisement: Default::default(),
        }
    }

    pub fn with_id(mut self, id: Option<String>) -> Self {
        self.id = id;
        self
    }

    pub fn with_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.ip = ip;
        self
    }
    pub fn ip_mut(&mut self) -> &mut Option<IpAddr> {
        &mut self.ip
    }
    pub fn with_port(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }
    pub fn port_mut(&mut self) -> &mut Option<u16> {
        &mut self.port
    }

    pub fn bcast_port(mut self, port: u16) -> Self {
        self.broadcast = self
            .broadcast
            .map(|mut t| {
                t.set_port(port);
                t
            })
            .or_else(|| {
                Some(
                    SocketAddr::default_addr()
                        .as_broadcast_addr(None)
                        .unwrap()
                        .set_port(port)
                        .to_owned(),
                )
            });
        self
    }
    pub fn bcast_ip(mut self, ip: IpAddr) -> Self {
        self.broadcast = self
            .broadcast
            .map(|mut t| {
                t.set_ip(ip);
                t
            })
            .or_else(|| {
                Some(
                    SocketAddr::default_addr()
                        .as_broadcast_addr(None)
                        .unwrap()
                        .set_ip(ip)
                        .to_owned(),
                )
            });
        self
    }

    pub fn with_broadcast_mask(mut self, mask: Option<IpAddr>) -> Self {
        if let (Some(ip), Some(port)) = (&self.ip, &self.port) {
            self.broadcast = SocketAddr::new(*ip, *port).as_broadcast_addr(mask).ok();
        }
        self
    }
    pub fn set_broadcast_mask(&mut self, mask: IpAddr) -> &mut Self {
        if let (Some(ip), Some(port)) = (&self.ip, &self.port) {
            self.broadcast = SocketAddr::new(*ip, *port)
                .as_broadcast_addr(Some(mask))
                .ok();
        }
        self
    }

    pub fn with_broadcast(mut self, broadcast: Option<SocketAddr>) -> Self {
        self.broadcast = broadcast;
        self
    }

    pub fn bcast_mut(&mut self) -> &mut Option<SocketAddr> {
        &mut self.broadcast
    }
    pub fn with_service_name(mut self, service_name: Option<String>) -> Self {
        self.service_name = service_name;
        self
    }
    pub fn service_name_mut(&mut self) -> &mut Option<String> {
        &mut self.service_name
    }
    pub fn with_version(mut self, version: Option<String>) -> Self {
        self.version = version;
        self
    }
    pub fn version_mut(&mut self) -> &mut Option<String> {
        &mut self.version
    }
    pub fn with_interval(mut self, interval: Option<u64>) -> Self {
        self.interval = interval;
        self
    }
    pub fn interval_mut(&mut self) -> &mut Option<u64> {
        &mut self.interval
    }
}

impl<A> Debug for Broadcaster<A>
where
    A: Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Broadcaster")
            .field("ip", &self.ip)
            .field("port", &self.port)
            .field("broadcast", &self.broadcast)
            .field("service_name", &self.service_name)
            .field("version", &self.version)
            .field("interval", &self.interval)
            .field(
                "advertisement",
                &format!("has_data({})", self.advertisement.is_some()),
            )
            .finish()
    }
}

impl<A: Into<Vec<u8>>> Default for Broadcaster<A>
where
    A: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
