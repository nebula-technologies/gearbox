pub mod entity;

use crate::log::tracing::subscriber::layer::fmt::discover_log::entity::SystemInfo;
use crate::{
    collections::HashMap,
    log::tracing::subscriber::{
        entity::{Facility, Severity},
        layer::{
            fmt::discover_log::entity::{
                discovery::{Config, DiscoveryConfig, DiscoveryMessage, Endpoint},
                Caller, Device, ProcessInfo, Service, Timestamps,
            },
            LogLayer, Storage, Type,
        },
        LogFormatter, Value,
    },
    sync::rw_arc::RwArc,
    time::DateTime,
};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{
    fmt,
    ops::{Deref, DerefMut},
    time::Duration,
};
use futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::{io, net::UdpSocket, task, time::interval};
use tracing::{Event, Subscriber};
use tracing_subscriber::{
    fmt::MakeWriter,
    registry::{LookupSpan, SpanRef},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscoveryLog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<Caller>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aid: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub payload_data: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stacktrace: Vec<String>,
    pub timestamps: Timestamps,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facility: Option<Facility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
    #[serde(skip)]
    pub logging_config: RwArc<Option<Config>>,
}

impl DiscoveryLog {
    pub fn set_discovery<O: Fn(DiscoveryConfig) -> DiscoveryConfig>(self, o: O) -> Self {
        let updated_config = if let Some(c) = self
            .logging_config
            .detach()
            .as_ref()
            .and_then(|t| t.discovery.clone())
        {
            o(c)
        } else {
            o(DiscoveryConfig::default())
        };

        self.logging_config
            .write()
            .as_mut()
            .map(|t| t.discovery = Some(updated_config));

        self
    }

    pub fn start_discovery(self) -> Self {
        // Clone the logging config Arc<RwLock>
        let config = if let Some(_) = self.logging_config.detach().clone() {
            self.logging_config.clone()
        } else {
            *self.logging_config.write() = Some(Config::default());
            self.logging_config.clone()
        };

        // Spawn a background task to listen for discovery messages and update the config
        task::spawn(async move {
            // Retrieve the discovery config from the cloned logging config
            loop {
                let (addr, interval_sec) = {
                    let config_read = config.read();
                    if let Some(discovery_config) =
                        config_read.as_ref().and_then(|t| t.discovery.as_ref())
                    {
                        let addr = SocketAddr::new(discovery_config.ip, discovery_config.port);
                        let interval_sec = discovery_config.capture_interval;
                        (addr, interval_sec)
                    } else {
                        // If no discovery config is present, retry after a default interval
                        (SocketAddr::new("0.0.0.0".parse().unwrap(), 8080), 5)
                    }
                };

                let mut interval = interval(Duration::from_secs(interval_sec));

                // Attempt to bind the socket
                let socket = match UdpSocket::bind(addr).await {
                    Ok(sock) => {
                        println!("Listening for discovery messages on {}", addr);
                        sock
                    }
                    Err(e) => {
                        println!("Failed to bind socket on {}: {}. Retrying...", addr, e);
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

                            println!("Received discovery message from {}: {}", src, received_msg);

                            // Process the discovery message
                            if received_msg.contains("DISCOVERY") {
                                let discovery_message = DiscoveryMessage {
                                    ip: None,
                                    port: None,
                                    service_name: None,
                                    version: None,
                                    http: false,
                                    timestamp: None,
                                    additional_info: None,
                                    mac: None,
                                    http_api_schema_endpoint: None,
                                };

                                // Lock the config for write access to update the discovery info
                                if let Some(config_write) = config.write().as_mut() {
                                    config_write.update_endpoint_form_discovery(&discovery_message);
                                    println!("Updated config with discovery message from {}", src);
                                } else {
                                    println!("No discovery config found in logging config.");
                                }
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
        });

        self
    }

    pub fn with_default_fields(name: String, default_fields: HashMap<String, Value>) -> Self {
        let mut payload_data: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (k, v) in default_fields {
            payload_data.insert(k, serde_json::Value::from(&v));
        }

        Self {
            version: None,
            caller: None,
            correlation_id: None,
            device: Some(Device::default()),
            duration: None,
            environment: None,
            id: None,
            aid: None,
            local_id: Some(vec![name]),
            message: None,
            payload_data,
            service: None,
            severity: None,
            span_id: None,
            stacktrace: vec![],
            timestamps: Timestamps::default(),
            trace_id: None,
            facility: None,
            process: None,
            system_info: Some(SystemInfo::default()),
            logging_config: RwArc::new(None),
        }
    }

    fn format_event_message<
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    >(
        &self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String {
        let mut message = event_visitor
            .values()
            .get("message")
            .and_then(|v| match v {
                Value::String(s) => Some(format!("{} {}", event.metadata().name(), s.as_str())),
                _ => None,
            })
            .unwrap_or_else(|| {
                format!("{} {}", event.metadata().name(), event.metadata().target())
            });

        if let Some(span) = &current_span {
            message = format!(
                "{} {}",
                self.format_span_context(span, &Type::Event),
                message
            );
        }

        message
    }

    fn format_span_context<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &self,
        span: &SpanRef<S>,
        _ty: &Type,
    ) -> String {
        format!("({})", span.metadata().name())
    }
}

impl Default for DiscoveryLog{
    fn default() -> Self {
        Self {
            version: None,
            caller: None,
            correlation_id: None,
            device: Some(Device::default()),
            duration: None,
            environment: None,
            id: None,
            aid: None,
            local_id: Some(vec![]),
            message: None,
            payload_data: serde_json::Map::new(),
            service: None,
            severity: None,
            span_id: None,
            stacktrace: vec![],
            timestamps: Timestamps::default(),
            trace_id: None,
            facility: None,
            process: None,
            system_info: Some(SystemInfo::default()),
            logging_config: RwArc::new(None),
        }
    }
}

impl fmt::Display for DiscoveryLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        //<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"] BOMAn application event log entry...
        let default_string = "-".to_string();
        write!(f, "")
    }
}

impl LogFormatter for DiscoveryLog {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        &self,
        layer: &LogLayer<W, F>,
    ) -> Self {
        Self {
            facility: None,
            severity: None,
            span_id: None,
            version: Some(1),
            caller: None,
            correlation_id: None,
            device: None,
            duration: None,
            timestamps: Timestamps {
                timestamp: Some(DateTime::now()),
                received_timestamp: None,
            },
            process: Some(ProcessInfo {
                application: layer.application().to_owned(),
                process_id: layer.proc_id().to_owned(),
            }),
            id: None,
            aid: None,
            local_id: None,
            message: None,
            payload_data: Default::default(),
            service: None,
            environment: None,
            stacktrace: vec![],
            trace_id: None,
            logging_config: RwArc::new(None),
            system_info: None,
        }
    }
    fn format_event<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String {
        // let mut buffer = Vec::new();
        //
        self.message =
            Option::from(self.format_event_message(&current_span, event, &event_visitor));
        self.local_id = current_span.as_ref().map(|t| {
            vec![
                t.id().into_non_zero_u64().to_string(),
                Type::Event.to_string(),
            ]
        });

        self.caller = Some(Caller {
            function: event.metadata().module_path().map(|t| t.to_owned()),
            file: event.metadata().file().map(|t| t.to_string()),
            line: event.metadata().line().map(|t| t as i64),
        });

        self.severity = event_visitor
            .get("severity")
            .and_then(|t| t.try_into().ok())
            .or_else(|| event_visitor.get("level").and_then(|t| t.try_into().ok()))
            .or_else(|| Option::from(Severity::from(event.metadata().level())));

        self.facility = event_visitor
            .get("log_facility")
            .and_then(|t| t.try_into().ok())
            .or_else(|| {
                event_visitor
                    .get("facility")
                    .and_then(|t| t.try_into().ok())
            });

        if let Some(s) = &current_span {
            self.trace_id = s
                .extensions()
                .get::<HashMap<String, String>>() // Assuming the fields are stored in a HashMap
                .and_then(|fields| fields.get("trace_id").cloned());
        } else {
            self.trace_id = event_visitor
                .get("trace_id")
                .and_then(|v| v.try_into().ok());
        }

        self.to_string()
    }

    fn format_span<S: Subscriber + for<'a> LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String {
        "".to_string()
    }
}

#[derive(Debug)]
pub struct StructuredData(HashMap<String, Value>);

impl ToString for StructuredData {
    fn to_string(&self) -> String {
        let mut strings = Vec::new();
        for (key, val) in self.deref() {
            strings.push(format!(
                r#"{}="{}""#,
                key,
                val.to_string().replace(r#"""#, r#"\\""#)
            ));
        }
        strings.join(" ")
    }
}

impl Default for StructuredData {
    fn default() -> Self {
        StructuredData(Default::default())
    }
}

impl Deref for StructuredData {
    type Target = HashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StructuredData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
