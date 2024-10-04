use crate::common::get_ips;
use crate::service::discovery::entity::discovery::Advertisement;
use crate::service::discovery::entity::{AdvertiserConfig, Config, DiscovererConfig};
use crate::service::discovery::DiscoveryService;
use crate::sync::rw_arc::RwArc;
use crate::time::DateTime;
use crate::{debug, error, info};
use bytes::Bytes;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::interval;

pub struct CommonServiceDiscovery {
    pub discovery_config: RwArc<Option<Config>>,
}

impl Default for CommonServiceDiscovery {
    fn default() -> Self {
        Self {
            discovery_config: RwArc::new(None),
        }
    }
}

impl DiscoveryService for CommonServiceDiscovery {
    fn set_service_config<O: Fn(Config) -> Config>(self, o: O) -> Self {
        let mut t = self.discovery_config.write();
        *t = Some(o(t.clone().unwrap_or(Config::default())));

        self
    }

    fn start_broadcast(self) -> (Self, JoinHandle<()>) {
        let broadcast_config = self.discovery_config.clone();
        info!("Starting broadcast service");
        (
            self,
            task::spawn(async move {
                loop {
                    let (ip, port, interval, advertisement) = {
                        let bcast = if let Some(c) =
                            broadcast_config.read().as_ref().map(|t| &t.advertiser)
                        {
                            c.clone()
                        } else {
                            Some(AdvertiserConfig::default())
                        };
                        let bcast_ref = bcast.as_ref();
                        (
                            bcast_ref
                                .map(|t| t.ip)
                                .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                            bcast_ref.map(|t| t.port).unwrap_or(9999),
                            bcast_ref.map(|t| t.interval).unwrap_or(5),
                            bcast_ref
                                .map(|t| t.advertisement.clone())
                                .unwrap_or(Bytes::new()),
                        )
                    };

                    // Bind a socket to send broadcast messages
                    let socket = match UdpSocket::bind(SocketAddr::new(ip, port)).await {
                        Ok(socket) => {
                            if let Err(e) = socket.set_broadcast(true) {
                                error!(
                                    "Failed to switch broadcast socket into broadcast mode: {}",
                                    e
                                );
                                continue;
                            }
                            socket
                        }
                        Err(e) => {
                            error!("Failed to bind the broadcast socket: {}", e);
                            continue;
                        }
                    };

                    loop {
                        socket
                            .send_to(&*advertisement, format!("255.255.255.255:{}", port))
                            .await
                            .ok();
                        debug!("Broadcasting discovery message",);

                        tokio::time::sleep(Duration::from_secs(interval as u64)).await;
                    }
                }
            }),
        )
    }

    fn start_discovery(self) -> (Self, JoinHandle<()>) {
        // Clone the logging config Arc<RwLock>
        let config = if let Some(_) = self.discovery_config.detach().clone() {
            self.discovery_config.clone()
        } else {
            *self.discovery_config.write() = Some(Config::default());
            self.discovery_config.clone()
        };

        // Spawn a background task to listen for discovery messages and update the config
        (
            self,
            task::spawn(async move {
                // Retrieve the discovery config from the cloned logging config
                loop {
                    let (ip, port, retry_interval) = {
                        let bcast = if let Some(c) = config.read().as_ref().map(|t| &t.discoverer) {
                            c.clone()
                        } else {
                            Some(DiscovererConfig::default())
                        };
                        let bcast_ref = bcast.as_ref();
                        (
                            bcast_ref
                                .map(|t| t.ip)
                                .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                            bcast_ref.map(|t| t.port).unwrap_or(9999),
                            bcast_ref.map(|t| t.capture_interval).unwrap_or(9999),
                        )
                    };

                    let mut interval = interval(Duration::from_secs(retry_interval as u64));

                    // Attempt to bind the socket
                    let socket = match UdpSocket::bind(SocketAddr::new(ip, port)).await {
                        Ok(sock) => {
                            println!("Listening for discovery messages on {}:{}", ip, port);
                            sock
                        }
                        Err(e) => {
                            println!("Failed to bind socket on {}: {}. Retrying...", ip, e);
                            interval.tick().await;
                            continue; // Retry the loop on failure
                        }
                    };

                    let mut buf = [0u8; 1024]; // Buffer to store the incoming message

                    loop {
                        interval.tick().await;

                        match socket.recv_from(&mut buf).await {
                            Ok((len, src)) => {
                                info!("Received discovery message from: {:<20}", src);
                                let bytes_ref = (&buf[..len]).to_vec();
                                let bytes = Bytes::from(bytes_ref);

                                // Lock the config for write access to update the discovery info
                                if let Some(config_rw) = config.write().as_mut() {
                                    if let Some(d) = &mut config_rw.discoverer {
                                        d.advert_extract = bytes;
                                    }
                                    println!("Updated config with discovery message from {}", src);
                                } else {
                                    println!("No discovery config found in logging config.");
                                }
                            }
                            Err(e) => {
                                println!(
                                    "Error receiving discovery message: {}. Restarting socket...",
                                    e
                                );
                                break; // Break to restart the socket
                            }
                        }
                    }
                }
            }),
        )
    }
}
