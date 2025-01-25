pub mod socket_addr;
pub mod socket_addr_error;
pub mod socket_addr_try_with_builder;
pub mod socket_addr_with_builder;
pub mod socket_addrs;
pub mod socket_addrs_error;
pub mod socket_addrs_try_with_builder;
pub mod socket_addrs_with_builder;

use std::net::IpAddr;

pub use {
    socket_addr::SocketAddr, socket_addr_with_builder::SocketAddrWithBuilder,
    socket_addrs::SocketAddrs, socket_addrs_error::SocketAddrsError,
    socket_addrs_with_builder::SocketAddrsWithBuilder,
};

pub type Ipv4Raw = (u8, u8, u8, u8);
pub type Ipv6Raw = (u16, u16, u16, u16, u16, u16, u16, u16);

pub trait SocketTryWithBuilder<T, O>
where
    Self: Sized,
{
    type Error;
    fn ipv4_port(self, ip: Ipv4Raw, port: u16) -> Self;
    fn ipv6_port(self, ip: Ipv6Raw, port: u16) -> Self;

    fn ipaddr_port(self, ip: IpAddr, port: u16) -> Self;

    fn addr(self, addr: SocketAddr) -> Self;

    fn default_addr(self, addr: SocketAddr) -> Self;

    fn with_default_ipv4(self, ip: Ipv4Raw, port: u16) -> Self;

    fn with_default_ipv6(self, ipv6raw: Ipv6Raw, port: u16) -> Self;

    fn default_port(self, port: u16) -> Self;

    fn if_default_port(self, port: u16) -> Self;
    fn try_capture_ip(self) -> Result<T, Self::Error>;
    fn if_try_capture_ip(self) -> Result<T, Self::Error>;
    fn try_capture_broadcast(self) -> Result<T, Self::Error>;
    fn if_try_capture_broadcast(self) -> Result<T, Self::Error>;
    fn build(self) -> Result<O, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr as StdSocketAddr, ToSocketAddrs};

    #[test]
    fn test_socket_bind_addr_new() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
        assert_eq!(addr.port(), Some(8080));
    }

    #[test]
    fn test_socket_bind_addr_default() {
        let addr = SocketAddr::default_addr();
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))));
        assert_eq!(addr.port(), Some(9999));
    }

    #[test]
    fn test_socket_bind_addr_with_ip() {
        let addr =
            SocketAddr::default_addr().with_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
    }

    #[test]
    fn test_socket_bind_addr_with_port() {
        let addr = SocketAddr::default_addr().with_port(Some(8080));
        assert_eq!(addr.port(), Some(8080));
    }

    #[test]
    fn test_socket_bind_addr_to_socket_addrs() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut addrs_iter = addr.to_socket_addrs().unwrap();
        assert_eq!(
            addrs_iter.next().unwrap(),
            StdSocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    fn test_from_socket_bind_addr_to_socket_addr() {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080);
        let socket_addr: StdSocketAddr = bind_addr.into();
        assert_eq!(
            socket_addr,
            StdSocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080)
        );
    }

    #[test]
    fn test_socket_bind_addrs_add_bind_ipv4_port() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv4_port((192, 168, 1, 1), 8080);
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_add_bind_ipv6_port() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv6_port((0xfe80, 0, 0, 0, 0, 0, 0, 1), 8080);
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_with_default() {
        let bind_addrs = SocketAddrs::with_default();
        assert!(bind_addrs.default_bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_merge_defaults() {
        let mut bind_addrs = SocketAddrs::with_default();
        bind_addrs.merged_defaults();
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_to_socket_addrs_for_socket_bind_addrs() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv4_port((127, 0, 0, 1), 8080);
        let addrs_iter = bind_addrs.to_socket_addrs().unwrap();
        let addrs: Vec<StdSocketAddr> = addrs_iter.collect();
        assert_eq!(addrs.len(), 1);
        assert_eq!(
            addrs[0],
            StdSocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    fn test_from_socket_bind_addrs_to_vec() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv4_port((127, 0, 0, 1), 8080);
        let vec: Vec<SocketAddr> = bind_addrs.into();
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_socket_bind_addr_as_broadcast_ipv4() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)), 8080);
        let broadcast = addr
            .as_broadcast_addr(Some(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0))))
            .ok();
        assert_eq!(
            broadcast.unwrap().ip(),
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255)))
        );
    }

    #[test]
    fn test_socket_bind_addr_as_broadcast_ipv6() {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
            8080,
        );
        let broadcast = addr.as_broadcast_addr(Some(IpAddr::V6(Ipv6Addr::new(
            0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0,
        ))));
        assert_eq!(
            broadcast.unwrap().ip(),
            Some(IpAddr::V6(Ipv6Addr::new(
                0x2001, 0xdb8, 0, 0, 0xffff, 0xffff, 0xffff, 0xffff
            )))
        );
    }
}
