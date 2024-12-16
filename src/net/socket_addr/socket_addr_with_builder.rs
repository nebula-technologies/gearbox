use std::net::IpAddr;

pub struct SocketAddrWithBuilder {
    ip: Option<IpAddr>,
    port: Option<u16>,
    default_ip: Option<IpAddr>,
    default_port: Option<u16>,
}
