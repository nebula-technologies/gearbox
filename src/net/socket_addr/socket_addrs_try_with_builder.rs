use crate::net::ip::IpAddrs;
use crate::net::socket_addr::socket_addrs_error::SocketAddrsError;
use crate::net::socket_addr::socket_addrs_with_builder::SocketAddrsWithBuilder;
use crate::net::socket_addr::{Ipv4Raw, Ipv6Raw, SocketAddr, SocketAddrs, SocketTryWithBuilder};
use crate::rails::ext::blocking::Merge;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl SocketTryWithBuilder<SocketAddrsWithBuilder, SocketAddrs> for SocketAddrsWithBuilder {
    type Error = SocketAddrsError;

    fn ipv4_port(mut self, ip: Ipv4Raw, port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3)), port);
        self.addr(addr)
    }

    fn ipv6_port(mut self, ip: Ipv6Raw, port: u16) -> Self {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(
                ip.0, ip.1, ip.2, ip.3, ip.4, ip.5, ip.6, ip.7,
            )),
            port,
        );
        self.addr(addr)
    }

    fn ipaddr_port(mut self, ip: IpAddr, port: u16) -> Self {
        let addr = SocketAddr::new(ip, port);
        self.addr(addr)
    }

    fn addr(mut self, addr: SocketAddr) -> Self {
        if let Some(ref mut bind_addrs) = self.bind_addr {
            bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.bind_addr = Some(set);
        }
        self
    }

    fn default_addr(mut self, addr: SocketAddr) -> Self {
        if let Some(ref mut default_bind_addrs) = self.default_bind_addr {
            default_bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.default_bind_addr = Some(set);
        }
        self
    }

    fn with_default_ipv4(mut self, o: Ipv4Raw, port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o.0, o.1, o.2, o.3)), port);
        self.default_addr(addr)
    }

    fn with_default_ipv6(mut self, o: Ipv6Raw, port: u16) -> Self {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(o.0, o.1, o.2, o.3, o.4, o.5, o.6, o.7)),
            port,
        );
        self.default_addr(addr)
    }

    fn default_port(mut self, port: u16) -> Self {
        self.default_port = Some(port);
        self
    }

    fn if_default_port(mut self, port: u16) -> Self {
        if self.default_port.is_none() {
            self.default_port = Some(port);
        }
        self
    }

    fn try_capture_ip(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
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
                        self.bind_addr
                            .get_or_insert(HashSet::new())
                            .insert(SocketAddr::new(ip, port));
                    }
                    Ok(self)
                },
            )
    }
    fn if_try_capture_ip(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        if self.bind_addr.is_none() {
            self.try_capture_ip()
        } else {
            Ok(self)
        }
    }

    fn try_capture_broadcast(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.default_port
            .ok_or(SocketAddrsError::FailedToCaptureIp(
                "No default port".to_string(),
            ))
            .merge(
                IpAddrs::new()
                    .try_with_capture_broadcast()
                    .map_err(|e| SocketAddrsError::FailedToCaptureIp(format!("{:?}", e))),
                |port, ips| {
                    for ip in ips.into_iter() {
                        self.bind_addr
                            .get_or_insert(HashSet::new())
                            .insert(SocketAddr::new(ip, port));
                    }
                    Ok(self)
                },
            )
    }
    fn if_try_capture_broadcast(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        if self.bind_addr.is_none() {
            self.try_capture_broadcast()
        } else {
            Ok(self)
        }
    }
    fn build(self) -> Result<SocketAddrs, Self::Error> {
        // In this implementation, `build` is infallible because the configuration is
        // intentionally flexible and doesn't enforce constraints on the presence of addresses.
        Ok(SocketAddrs {
            bind_addr: self.bind_addr,
            default_bind_addr: self.default_bind_addr,
            default_port: self.default_port,
        })
    }
}

impl SocketTryWithBuilder<SocketAddrsWithBuilder, SocketAddrs>
    for Result<SocketAddrsWithBuilder, SocketAddrsError>
{
    type Error = SocketAddrsError;

    fn ipv4_port(self, ip: Ipv4Raw, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.ipv4_port(ip, port))
    }

    fn ipv6_port(self, ip: Ipv6Raw, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.ipv6_port(ip, port))
    }

    fn ipaddr_port(self, ip: IpAddr, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.ipaddr_port(ip, port))
    }

    fn addr(self, addr: SocketAddr) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.addr(addr))
    }

    fn default_addr(self, addr: SocketAddr) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.default_addr(addr))
    }

    fn with_default_ipv4(
        self,
        o: Ipv4Raw,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.with_default_ipv4(o, port))
    }

    fn with_default_ipv6(
        self,
        o: Ipv6Raw,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.with_default_ipv6(o, port))
    }

    fn default_port(self, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.default_port(port))
    }

    fn if_default_port(self, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.if_default_port(port))
    }
    fn try_capture_ip(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.try_capture_ip())
    }

    fn if_try_capture_ip(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.if_try_capture_ip())
    }

    fn try_capture_broadcast(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.try_capture_broadcast())
    }

    fn if_try_capture_broadcast(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.if_try_capture_broadcast())
    }

    fn build(self) -> Result<SocketAddrs, Self::Error> {
        // In this implementation, `build` is infallible because the configuration is
        // intentionally flexible and doesn't enforce constraints on the presence of addresses.
        self.and_then(|t| t.build())
    }
}
