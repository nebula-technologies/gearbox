use super::SocketAddrWithBuilder;
use crate::net::ip::IpAddrs;
use crate::net::socket_addr::socket_addrs_error::SocketAddrsError;
use crate::net::socket_addr::socket_addrs_with_builder::SocketAddrsWithBuilder;
use crate::net::socket_addr::{Ipv4Raw, Ipv6Raw, SocketAddr, SocketAddrs, SocketTryWithBuilder};
use crate::rails::ext::blocking::Merge;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl SocketTryWithBuilder<SocketAddrWithBuilder> for SocketAddrWithBuilder {
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
        self.ip = addr.ip();
        self.port = addr.port();
        self
    }

    fn default_addr(mut self, addr: SocketAddr) -> Self {
        self.default_ip = addr.ip();
        self.default_port = addr.port();
        self
    }

    fn with_default_ipv4(mut self, ip: Ipv4Raw, port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3)), port);
        self.default_addr(addr)
    }

    fn with_default_ipv6(mut self, ip: Ipv6Raw, port: u16) -> Self {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(
                ip.0, ip.1, ip.2, ip.3, ip.4, ip.5, ip.6, ip.7,
            )),
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
    fn try_capture_ip(mut self) -> Result<SocketAddrWithBuilder, Self::Error> {
        self.default_port
            .ok_or(SocketAddrsError::FailedToCaptureIp(
                "No default port".to_string(),
            ))
            .merge(
                IpAddrs::new()
                    .try_with_capture_ips()
                    .map_err(|e| SocketAddrsError::FailedToCaptureIp(format!("{:?}", e))),
                |port, ips| {
                    if let Some(ip) = ips.last() {
                        self.ip = Some(*ip);
                        self.port = Some(port);
                    }
                    Ok(self)
                },
            )
    }
    fn if_try_capture_ip(mut self) -> Result<SocketAddrWithBuilder, Self::Error> {
        if self.ip.is_none() {
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

impl SocketTryWithBuilder<SocketAddrWithBuilder>
    for Result<SocketAddrWithBuilder, SocketAddrError>
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
        o1: u8,
        o2: u8,
        o3: u8,
        o4: u8,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.with_default_ipv4(o1, o2, o3, o4, port))
    }

    fn with_default_ipv6(
        self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.with_default_ipv6(o1, o2, o3, o4, o5, o6, o7, o8, port))
    }

    fn default_port(self, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.default_port(port))
    }

    fn if_default_port(self, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.if_default_port(port))
    }
    fn try_capture_ips(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.try_capture_ips())
    }

    fn if_try_capture_ips(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.if_try_capture_ips())
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
