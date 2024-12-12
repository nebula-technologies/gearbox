use crate::log::tracing::entity::deeplog::{Caller, DeepLog, Timestamps};
#[cfg(feature = "net-endpoint-config")]
use crate::net::endpoint_config::EndpointConfig;
use crate::{
    collections::HashMap,
    log::tracing::{
        entity::syslog::Severity,
        layer::{LogLayer, Storage, Type},
        LogFormatter, Value,
    },
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
};
use futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use tracing::{Event, Subscriber};
use tracing_subscriber::{
    fmt::MakeWriter,
    registry::{LookupSpan, SpanRef},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LogStyleOutput {
    Full,
    Minimal,
    Human,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeepLogFormatter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<DeepLog>,
    #[serde(skip)]
    pub output_style: Option<LogStyleOutput>,
    #[serde(skip)]
    #[cfg(feature = "net-endpoint-config")]
    pub endpoint: Option<EndpointConfig>,
}

impl DeepLogFormatter {
    pub fn with_default_fields(name: String, default_fields: HashMap<String, Value>) -> Self {
        let mut payload_data: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (k, v) in default_fields {
            payload_data.insert(k, serde_json::Value::from(&v));
        }

        Self {
            log: Some(DeepLog::default()),
            output_style: None,
            #[cfg(feature = "net-endpoint-config")]
            endpoint: None,
        }
    }

    pub fn set_output_style(mut self, output_style: LogStyleOutput) -> Self {
        self.output_style = Some(output_style);
        self
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

impl Default for DeepLogFormatter {
    fn default() -> Self {
        Self {
            log: Some(DeepLog::default()),
            output_style: None,
            #[cfg(feature = "net-endpoint-config")]
            endpoint: None,
        }
    }
}

impl fmt::Display for DeepLogFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        //<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"] BOMAn application event log entry...
        let default_string = "-".to_string();
        write!(f, "")
    }
}

impl LogFormatter for DeepLogFormatter {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        &self,
        layer: &LogLayer<W, F>,
    ) -> Self {
        Self {
            log: Some(DeepLog::default()),
            output_style: self.output_style.clone(),

            #[cfg(feature = "net-endpoint-config")]
            endpoint: self.endpoint.clone(),
        }
    }
    fn format_event<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String {
        if self.log.is_none() {
            self.log = Some(DeepLog::default())
        }
        let deeplog = if let Some(mut deeplog) = self.log.clone() {
            deeplog.message = Option::from(
                event_visitor
                    .values()
                    .get("message")
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.to_string()),
                        _ => None,
                    })
                    .unwrap_or_else(|| {
                        format!("{} {}", event.metadata().name(), event.metadata().target())
                    }),
            );
            if let Some(span) = &current_span {
                deeplog.span_id = Option::from(span.metadata().name().to_string());
            }
            deeplog.local_id = current_span.as_ref().map(|t| {
                vec![
                    t.id().into_non_zero_u64().to_string(),
                    Type::Event.to_string(),
                ]
            });

            deeplog.caller = Some(Caller {
                function: event.metadata().module_path().map(|t| t.to_owned()),
                file: event.metadata().file().map(|t| t.to_string()),
                line: event.metadata().line().map(|t| t as i64),
            });

            deeplog.severity = event_visitor
                .get("log_level")
                .and_then(|t| t.try_into().ok())
                .or_else(|| event_visitor.get("level").and_then(|t| t.try_into().ok()))
                .or_else(|| Option::from(Severity::from(event.metadata().level())));

            deeplog.facility = event_visitor
                .get("log_facility")
                .and_then(|t| t.try_into().ok())
                .or_else(|| {
                    event_visitor
                        .get("facility")
                        .and_then(|t| t.try_into().ok())
                });

            if let Some(s) = &current_span {
                deeplog.trace_id = s
                    .extensions()
                    .get::<HashMap<String, String>>() // Assuming the fields are stored in a HashMap
                    .and_then(|fields| fields.get("trace_id").cloned());
            } else {
                deeplog.trace_id = event_visitor
                    .get("trace_id")
                    .and_then(|v| v.try_into().ok());
            }
            if let Some(timestamps) = &mut deeplog.timestamps {
                timestamps.received_timestamp = None;
                if timestamps.timestamp.is_none() {
                    timestamps.timestamp = Some(DateTime::now());
                }
            } else {
                deeplog.timestamps = Some(Timestamps {
                    received_timestamp: None,
                    timestamp: Some(DateTime::now()),
                });
            }

            deeplog.clone()
        } else {
            let mut deeplog = DeepLog::default();
            deeplog.message = Some("Internal Log error".to_string());
            deeplog
        };

        match self.output_style.clone().unwrap_or(LogStyleOutput::Full) {
            LogStyleOutput::Full => serde_json::to_string(&deeplog)
                .unwrap_or_else(|e| format!(r#"{{"message":"{}"}}"#, e.to_string())),
            LogStyleOutput::Minimal => {
                let mut map = serde_json::value::Map::new();
                map.insert(
                    "timestamps".to_string(),
                    deeplog
                        .timestamps
                        .and_then(|t| t.timestamp)
                        .map(|t| t.to_rfc3339())
                        .map(|t| serde_json::Value::String(t))
                        .unwrap_or(serde_json::Value::Null),
                );
                map.insert(
                    "severity".to_string(),
                    serde_json::Value::String(
                        deeplog.severity.unwrap_or(Severity::Error).to_string(),
                    ),
                );
                map.insert(
                    "msg".to_string(),
                    serde_json::Value::String(deeplog.message.unwrap_or("No message".to_string())),
                );
                serde_json::to_string(&serde_json::Value::Object(map))
                    .unwrap_or_else(|e| format!(r#"{{"message":"{}"}}"#, e.to_string()))
            }
            LogStyleOutput::Human => {
                let severity = deeplog.severity.unwrap_or(Severity::Error);
                let timestamp = deeplog
                    .timestamps
                    .and_then(|t| t.timestamp)
                    .map(|t| t.to_rfc3339())
                    .unwrap_or("No timestamp".to_string());
                let msg = deeplog.message.unwrap_or("No message".to_string());

                // Extract file and line information
                let file = deeplog
                    .caller
                    .as_ref()
                    .and_then(|caller| {
                        caller.file.as_ref().map(|file_path| {
                            // If the file path is longer than 30 characters, truncate and add "..."
                            if file_path.len() > 40 {
                                format!(" ...{}", &file_path[file_path.len() - 37..])
                            } else {
                                // Pad the file path to the front with spaces if it's less than 30 characters
                                format!(" {:>40}", file_path)
                            }
                        })
                    })
                    .unwrap_or(format!("{:>30}", "No file"));

                let line = deeplog
                    .caller
                    .as_ref()
                    .and_then(|caller| caller.line)
                    .map(|line| line.to_string())
                    .unwrap_or("No line".to_string());

                // Format the log message with aligned severity and fixed-length file path
                format!("{} {:<15} {}:{} {}", timestamp, severity, file, line, msg)
            }
        }
    }

    fn format_span<S: Subscriber + for<'a> LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String {
        let deeplog = if let Some(l) = self.log.clone() {
            l
        } else {
            DeepLog::default()
        };

        let mut map = serde_json::value::Map::new();
        map.insert(
            "timestamps".to_string(),
            deeplog
                .timestamps
                .and_then(|t| t.timestamp)
                .map(|t| t.to_rfc3339())
                .map(|t| serde_json::Value::String(t))
                .unwrap_or(serde_json::Value::Null),
        );
        map.insert(
            "severity".to_string(),
            serde_json::Value::String(deeplog.severity.unwrap_or(Severity::Error).to_string()),
        );
        map.insert(
            "msg".to_string(),
            serde_json::Value::String(deeplog.message.unwrap_or("Enter Span".to_string())),
        );
        map.insert(
            "span_id".to_string(),
            serde_json::Value::String(span.metadata().name().to_string()),
        );

        serde_json::to_string(&serde_json::Value::Object(map))
            .unwrap_or_else(|e| format!(r#"{{"message":"{}"}}"#, e.to_string()))
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
