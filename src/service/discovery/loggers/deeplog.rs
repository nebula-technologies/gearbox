use crate::common::socket_bind_addr::SocketBindAddr;
use crate::log::tracing::entity::deeplog::DeepLog;
use crate::service::discovery::entity::Advertisement;
use crate::service::discovery::service_discovery::{
    Broadcaster, Discoverer, ServiceDiscoveryTrait,
};
use bytes::Bytes;
use core::str::FromStr;
use spin::RwLock;
use std::net::IpAddr;
use std::sync::mpsc::channel;
use tokio::sync::broadcast;

static DEEPLOG_ENDPOINT: RwLock<Option<SocketBindAddr>> = RwLock::new(None);

impl ServiceDiscoveryTrait for DeepLog {
    fn broadcasters() -> Vec<Broadcaster> {
        let broadcast: Broadcaster<Bytes> = Broadcaster::new()
            .with_service_name("DeepLog")
            .with_port(9999)
            .with_ip(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)))
            .broadcast_mask(IpAddr::V4(std::net::Ipv4Addr::new(255, 255, 255, 0)));

        vec![broadcast]
    }
    fn discoverers() -> Vec<Discoverer> {
        let mut discoverer: Discoverer<Bytes> = Discoverer::new();
        discoverer.add_processor(|data| async {
            Advertisement::try_from(data)
                .ok()
                .and_then(|t| t.ip.zip(t.port))
                .and_then(|(mut ips, port)| ips.pop().zip(Some(port)))
                .map(|(ip, port)| SocketBindAddr::from((ip, port)))
                .and_then(|t| DEEPLOG_ENDPOINT.write().replace(t));
        });

        vec![discoverer]
    }
    fn ip() -> IpAddr {
        std::env::var("DEEPLOG_IP")
            .ok()
            .and_then(|t| IpAddr::from_str(&t).ok())
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)))
    }
    fn port() -> usize {
        std::env::var("DEEPLOG_PORT")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(9999)
    }
}
