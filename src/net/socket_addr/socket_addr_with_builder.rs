use std::net::IpAddr;

pub struct SocketAddrWithBuilder {
    pub(crate) ip: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) default_ip: Option<IpAddr>,
    pub(crate) default_port: Option<u16>,
}
