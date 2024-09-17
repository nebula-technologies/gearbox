use crate::common::get_ips;
use crate::log::service::discovery::entity::discovery::DiscoveryMessage;
use crate::log::service::discovery::entity::{Broadcast, Config};
use crate::log::service::discovery::DiscoveryService;
use crate::log::tracing::formatter::deeplog::DeepLogFormatter;
use crate::time::DateTime;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::interval;

impl DiscoveryService for DeepLogFormatter {
    fn set_service_config<O: Fn(Config) -> Config>(self, o: O) -> Self {
        let mut t = self.discovery_config.write();
        *t = Some(o(t.clone().unwrap_or(Config::default())));

        self
    }

    fn start_broadcast(self) -> (Self, JoinHandle<()>) {
        let broadcast_config = self.discovery_config.clone();
        (
            self,
            task::spawn(async move {
                loop {
                    let (ip, port, bcast_port, bcast_interval) = {
                        let bcast = if let Some(c) =
                            broadcast_config.read().as_ref().map(|t| &t.broadcast)
                        {
                            c.clone()
                        } else {
                            Broadcast::default()
                        };
                        (
                            bcast
                                .clone()
                                .ip
                                .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                            bcast.clone().port.unwrap_or(0),
                            bcast.clone().bcast_port.unwrap_or(9999),
                            bcast.clone().bcast_interval.unwrap_or(5),
                        )
                    };

                    // Bind a socket to send broadcast messages
                    let socket = match UdpSocket::bind(SocketAddr::new(ip, port)).await {
                        Ok(socket) => {
                            if let Err(e) = socket.set_broadcast(true) {
                                println!(
                                    "Failed to switch broadcast socket into broadcast mode: {}",
                                    e
                                );
                                continue;
                            }
                            socket
                        }
                        Err(e) => {
                            println!("Failed to bind the broadcast socket: {}", e);
                            continue;
                        }
                    };

                    loop {
                        let (ip, port, service_name, version) = {
                            let message = if let Some(c) =
                                broadcast_config.read().as_ref().map(|t| &t.message)
                            {
                                c.clone()
                            } else {
                                DiscoveryMessage::default()
                            };
                            (
                                message.ip.unwrap_or(get_ips()),
                                message.port.unwrap_or(5000),
                                message.service_name.unwrap_or("<Unknown>".to_string()),
                                message.version.unwrap_or("v1.0.0".to_string()),
                            )
                        };

                        let discovery_message = DiscoveryMessage {
                            additional_info: None,
                            ip: Some(ip),
                            mac: None,
                            port: Some(port),
                            service_name: Some(service_name),
                            timestamp: Some(DateTime::now()),
                            version: Some(version),
                            http: false,
                            http_api_schema_endpoint: None,
                        };

                        let discovery_payload = if let Ok(t) =
                            serde_json::to_string(&discovery_message)
                        {
                            t
                        } else {
                            println!("Failed to convert payload into a sendable buffer/string");
                            tokio::time::sleep(Duration::from_secs(bcast_interval as u64)).await;
                            continue;
                        };

                        socket
                            .send_to(
                                discovery_payload.as_bytes(),
                                format!("255.255.255.255:{}", bcast_port),
                            )
                            .await
                            .ok();
                        println!(
                            "Broadcasting discovery message: {:<40}",
                            discovery_message.to_string()
                        );

                        tokio::time::sleep(Duration::from_secs(bcast_interval as u64)).await;
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
                    let (ip, port, bcast_port, bcast_interval) = {
                        let bcast = if let Some(c) = config.read().as_ref().map(|t| &t.broadcast) {
                            c.clone()
                        } else {
                            Broadcast::default()
                        };
                        (
                            bcast
                                .clone()
                                .ip
                                .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                            bcast.clone().port.unwrap_or(0),
                            bcast.clone().bcast_port.unwrap_or(9999),
                            bcast.clone().bcast_interval.unwrap_or(5),
                        )
                    };

                    let mut interval = interval(Duration::from_secs(bcast_interval as u64));

                    // Attempt to bind the socket
                    let socket = match UdpSocket::bind(SocketAddr::new(ip, bcast_port)).await {
                        Ok(sock) => {
                            println!("Listening for discovery messages on {}:{}", ip, bcast_port);
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

                        // Receive a message from the socket
                        match socket.recv_from(&mut buf).await {
                            Ok((len, src)) => {
                                let received_msg = String::from_utf8_lossy(&buf[..len]);

                                println!(
                                    "Received discovery message from: {:<20} for {}",
                                    src,
                                    serde_json::from_str::<DiscoveryMessage>(&received_msg)
                                        .map(|t| t.to_string())
                                        .unwrap_or("<failed>".to_string())
                                );

                                let svc_name = config
                                    .read()
                                    .as_ref()
                                    .and_then(|t| t.message.service_name.clone());

                                let discovery_message = if let Ok(t) =
                                    serde_json::from_str::<DiscoveryMessage>(received_msg.as_ref())
                                {
                                    t
                                } else {
                                    continue;
                                };

                                if let (Some(r), Some(s)) =
                                    (discovery_message.service_name.as_ref(), svc_name)
                                {
                                    if r != &s {
                                        continue;
                                    }
                                } else {
                                    continue;
                                }

                                // Lock the config for write access to update the discovery info
                                if let Some(config_rw) = config.write().as_mut() {
                                    config_rw.message = discovery_message.clone();

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
