use crate::log::tracing::entity::deeplog::DeepLog;
use crate::net::socket_bind_addr::SocketAddr;
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

static DEEPLOG_ENDPOINT: RwLock<Option<SocketAddr>> = RwLock::new(None);

impl ServiceDiscoveryTrait<(), Bytes> for DeepLog {
    fn broadcasters() -> Vec<Broadcaster<Bytes>> {
        let broadcast: Broadcaster<Bytes> = Broadcaster::new()
            .with_service_name(Some("DeepLog".to_string()))
            .with_port(Some(9999))
            .with_ip(Some(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))))
            .with_broadcast_mask(Some(IpAddr::V4(std::net::Ipv4Addr::new(255, 255, 255, 0))));

        vec![broadcast]
    }
    fn discoverers() -> Vec<Discoverer<(), Bytes>> {
        let mut discoverer: Discoverer<(), Bytes> = Discoverer::new();
        discoverer.add_processor(|data, _| {
            Box::pin(async {
                Advertisement::try_from(data)
                    .ok()
                    .and_then(|t| t.ip.zip(t.port))
                    .and_then(|(mut ips, port)| ips.pop().zip(Some(port)))
                    .map(|(ip, port)| SocketAddr::from((ip, port)))
                    .and_then(|t| DEEPLOG_ENDPOINT.write().replace(t));
            })
        });

        vec![discoverer]
    }
    fn port() -> usize {
        std::env::var("DEEPLOG_PORT")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(9999)
    }
    fn ip() -> IpAddr {
        std::env::var("DEEPLOG_IP")
            .ok()
            .and_then(|t| IpAddr::from_str(&t).ok())
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)))
    }
}
