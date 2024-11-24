use crate::net::socket_addr::SocketAddr;
use core::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ServiceBinding {
    port: usize,
    ip: IpAddr,
}

impl ServiceBinding {
    pub fn new(port: usize, ip: IpAddr) -> Self {
        ServiceBinding { port, ip }
    }

    pub fn port(&self) -> usize {
        self.port
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }
}

impl Display for ServiceBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl From<SocketAddr> for ServiceBinding {
    fn from(bind: SocketAddr) -> Self {
        ServiceBinding::new(bind.port_with_defaults() as usize, bind.ip_with_defaults())
    }
}

impl From<&SocketAddr> for ServiceBinding {
    fn from(bind: &SocketAddr) -> Self {
        ServiceBinding::new(bind.port_with_defaults() as usize, bind.ip_with_defaults())
    }
}

impl From<(IpAddr, u16)> for ServiceBinding {
    fn from((ip, port): (IpAddr, u16)) -> Self {
        ServiceBinding::new(port as usize, ip)
    }
}
impl From<(&IpAddr, &u16)> for ServiceBinding {
    fn from((ip, port): (&IpAddr, &u16)) -> Self {
        ServiceBinding::new(*port as usize, *ip)
    }
}

impl From<(&IpAddr, u16)> for ServiceBinding {
    fn from((ip, port): (&IpAddr, u16)) -> Self {
        ServiceBinding::new(port as usize, *ip)
    }
}
impl From<(IpAddr, &u16)> for ServiceBinding {
    fn from((ip, port): (IpAddr, &u16)) -> Self {
        ServiceBinding::new(*port as usize, ip)
    }
}

impl From<(IpAddr, usize)> for ServiceBinding {
    fn from((ip, port): (IpAddr, usize)) -> Self {
        ServiceBinding::new(port, ip)
    }
}
impl From<(&IpAddr, &usize)> for ServiceBinding {
    fn from((ip, port): (&IpAddr, &usize)) -> Self {
        ServiceBinding::new(*port, *ip)
    }
}
impl From<usize> for ServiceBinding {
    fn from(port: usize) -> Self {
        ServiceBinding::new(port, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
    }
}
impl From<&usize> for ServiceBinding {
    fn from(port: &usize) -> Self {
        ServiceBinding::new(*port, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
    }
}
impl From<(Option<IpAddr>, usize)> for ServiceBinding {
    fn from((bind_ip, port): (Option<IpAddr>, usize)) -> Self {
        ServiceBinding::new(
            port,
            bind_ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
        )
    }
}
impl From<(&Option<IpAddr>, &usize)> for ServiceBinding {
    fn from((ip, port): (&Option<IpAddr>, &usize)) -> Self {
        ServiceBinding::new(*port, ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
    }
}
