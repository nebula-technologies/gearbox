use crate::net::socket_addr::SocketAddr;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Default)]
pub struct SocketAddrsWithBuilder {
    pub(crate) bind_addr: Option<HashSet<SocketAddr>>,
    pub(crate) default_bind_addr: Option<HashSet<SocketAddr>>,
    pub(crate) default_port: Option<u16>,
}
