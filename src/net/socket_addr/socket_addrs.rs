use crate::net::ip::IpAddrs;
use crate::net::socket_addr::{SocketAddr, SocketAddrsError, SocketAddrsWithBuilder};
use crate::rails::ext::blocking::Merge;
use std::collections::HashSet;
use std::io;
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr as StdSocketAddr, ToSocketAddrs as StdToSocketAddrs,
};

// Struct representing multiple IP address and port bindings
#[derive(Debug, Clone, Default)]
pub struct SocketAddrs {
    pub bind_addr: Option<HashSet<SocketAddr>>,
    pub default_bind_addr: Option<HashSet<SocketAddr>>,
    pub default_port: Option<u16>,
}

impl SocketAddrs {
    pub fn with() -> SocketAddrsWithBuilder {
        SocketAddrsWithBuilder::default()
    }

    pub fn as_builder(self) -> SocketAddrsWithBuilder {
        SocketAddrsWithBuilder {
            bind_addr: self.bind_addr,
            default_bind_addr: self.default_bind_addr,
            default_port: self.default_port,
        }
    }

    pub fn add_bind_ipv4_port(&mut self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port);
        self.add_bind_addr(addr);
    }

    pub fn add_bind_ipv6_port(
        &mut self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(o1, o2, o3, o4, o5, o6, o7, o8)),
            port,
        );
        self.add_bind_addr(addr);
    }

    pub fn add_bind_ipaddr_port(&mut self, ip: IpAddr, port: u16) {
        let addr = SocketAddr::new(ip, port);
        self.add_bind_addr(addr);
    }

    /// Adds a new bind address to the list.
    pub fn add_bind_addr(&mut self, addr: SocketAddr) {
        if let Some(ref mut bind_addrs) = self.bind_addr {
            bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.bind_addr = Some(set);
        }
    }

    /// Adds a new default bind address to the list.
    pub fn add_default_bind_addr(&mut self, addr: SocketAddr) {
        if let Some(ref mut default_bind_addrs) = self.default_bind_addr {
            default_bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.default_bind_addr = Some(set);
        }
    }

    pub fn with_default_port(mut self, port: u16) -> Self {
        self.default_port = Some(port);
        self
    }

    pub fn with_default_port_if_none(mut self, port: u16) -> Self {
        if self.default_port.is_none() {
            self.default_port = Some(port);
        }
        self
    }

    /// Creates a `SocketBindAddrs` with a single default address.
    pub fn with_default() -> Self {
        let default_addr = SocketAddr::default_addr();
        let mut set = HashSet::new();
        set.insert(default_addr);
        SocketAddrs {
            bind_addr: None,
            default_bind_addr: Some(set),
            default_port: None,
        }
    }

    /// Merges bind addresses with default bind addresses.
    /// Defaults are added only if no primary bind addresses are provided.
    pub fn merged_defaults(&mut self) -> Vec<SocketAddr> {
        if self.bind_addr.is_none() {
            self.bind_addr = self.default_bind_addr.take();
        }
        self.bind_addr
            .clone()
            .unwrap_or(HashSet::new())
            .into_iter()
            .collect::<Vec<SocketAddr>>()
    }
}

impl IntoIterator for SocketAddrs {
    type Item = SocketAddr;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.merged_defaults().into_iter()
    }
}
trait TryWithSocketAddrs<T> {
    type Error;
    fn try_with_capture_ips(self) -> Result<T, Self::Error>;
    fn try_with_capture_ips_if_none(self) -> Result<T, Self::Error>;
}

impl TryWithSocketAddrs<SocketAddrs> for SocketAddrs {
    type Error = SocketAddrsError;
    fn try_with_capture_ips(mut self) -> Result<Self, Self::Error> {
        self.default_port
            .ok_or(SocketAddrsError::FailedToCaptureIp(
                "No default port".to_string(),
            ))
            .merge(
                IpAddrs::new()
                    .try_with_capture_ips()
                    .map_err(|e| SocketAddrsError::FailedToCaptureIp(format!("{:?}", e))),
                |port, ips| {
                    for ip in ips.into_iter() {
                        self.add_bind_ipaddr_port(ip, port);
                    }
                    Ok(self)
                },
            )
    }

    fn try_with_capture_ips_if_none(mut self) -> Result<Self, Self::Error> {
        if self.bind_addr.is_none() {
            self.try_with_capture_ips()
        } else {
            Ok(self)
        }
    }
}

impl StdToSocketAddrs for SocketAddrs {
    type Iter = std::vec::IntoIter<StdSocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let mut addrs = Vec::new();

        if let Some(bind_addrs) = &self.bind_addr {
            for addr in bind_addrs {
                addrs.push(StdSocketAddr::from(addr.clone()));
            }
        }

        if let Some(default_bind_addrs) = &self.default_bind_addr {
            for addr in default_bind_addrs {
                addrs.push(StdSocketAddr::from(addr.clone()));
            }
        }

        Ok(addrs.into_iter())
    }
}

impl From<SocketAddrs> for Vec<SocketAddr> {
    fn from(bind_addrs: SocketAddrs) -> Self {
        let mut addrs = Vec::new();

        if let Some(bind_addrs) = &bind_addrs.bind_addr {
            for addr in bind_addrs {
                addrs.push(addr.clone());
            }
        }

        if let Some(default_bind_addrs) = &bind_addrs.default_bind_addr {
            for addr in default_bind_addrs {
                addrs.push(addr.clone());
            }
        }

        addrs
    }
}
impl From<SocketAddrs> for Vec<std::net::SocketAddr> {
    fn from(bind_addrs: SocketAddrs) -> Self {
        bind_addrs
            .to_socket_addrs()
            .map(|t| t.collect())
            .unwrap_or(Vec::new())
    }
}

impl From<(IpAddr, u16)> for SocketAddrs {
    fn from((ip, port): (IpAddr, u16)) -> Self {
        Self::from((&ip, &port))
    }
}
impl From<(&IpAddr, &u16)> for SocketAddrs {
    fn from(sock: (&IpAddr, &u16)) -> Self {
        let mut set = HashSet::new();
        set.insert(sock.into());
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}
impl From<(&IpAddr, &usize)> for SocketAddrs {
    fn from(sock: (&IpAddr, &usize)) -> Self {
        let mut set = HashSet::new();
        set.insert(sock.into());
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}
impl From<(IpAddr, usize)> for SocketAddrs {
    fn from(sock: (IpAddr, usize)) -> Self {
        let mut set = HashSet::new();
        set.insert(sock.into());
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}
