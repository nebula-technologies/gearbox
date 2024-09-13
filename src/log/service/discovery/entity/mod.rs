pub mod discovery;

pub use crate::log::service::discovery::entity::discovery::DiscoveryMessage;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub broadcast: Broadcast,
    pub message: DiscoveryMessage,
}

/// Broadcasting
///
/// Common for broadcasting is that its 255.255.255.255 as this is the common broadcast address.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub ip: Option<IpAddr>,
    pub port: Option<u16>,
    pub bcast_port: Option<u16>,
    pub bcast_interval: Option<u16>,
}

impl Default for Broadcast {
    fn default() -> Self {
        Self {
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(0),
            bcast_port: Some(9999),
            bcast_interval: Some(5),
        }
    }
}
