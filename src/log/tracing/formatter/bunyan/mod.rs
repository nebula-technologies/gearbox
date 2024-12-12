use crate::log::tracing::layer::{LogLayer, Storage, Type};
use crate::log::tracing::{LogFormatter, Value};
use crate::time::{DateTime, SecondsFormat};
use hashbrown::HashMap;
use serde::ser::{SerializeMap, Serializer};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::{LookupSpan, SpanRef};

/// Keys for core fields of the Bunyan format (https://github.com/trentm/node-bunyan#core-fields)
const BUNYAN_VERSION: &str = "v";
const LEVEL: &str = "level";
const NAME: &str = "name";
const HOSTNAME: &str = "hostname";
const PID: &str = "pid";
const TIME: &str = "time";
const MESSAGE: &str = "msg";
const _SOURCE: &str = "src";

const BUNYAN_RESERVED_FIELDS: [&str; 7] =
    [BUNYAN_VERSION, LEVEL, NAME, HOSTNAME, PID, TIME, MESSAGE];

/// Convert from log levels to Bunyan's levels.
fn to_bunyan_level(level: &tracing::Level) -> u16 {
    match *level {
        tracing::Level::ERROR => 50,
        tracing::Level::WARN => 40,
        tracing::Level::INFO => 30,
        tracing::Level::DEBUG => 20,
        tracing::Level::TRACE => 10,
    }
}

/// This layer is exclusively concerned with formatting information using the [Bunyan format](https://github.com/trentm/node-bunyan).
/// It relies on the upstream `JsonStorageLayer` to get access to the fields attached to
/// each span.
#[derive(Debug, Clone)]
pub struct Bunyan {
    pid: u32,
    hostname: String,
    bunyan_version: u8,
    name: String,
    default_fields: HashMap<String, Value>,
}

impl Bunyan {
    /// Create a new `BunyanFormattingLayer`.

    pub fn new(name: String) -> Self {
        Self::with_default_fields(name, HashMap::new())
    }

    pub fn with_default_fields(name: String, default_fields: HashMap<String, Value>) -> Self {
        Self {
            name,
            pid: crate::common::process::id(),
            hostname: crate::net::hostname::gethostname()
                .to_string_lossy()
                .into_owned(),
            bunyan_version: 0,
            default_fields,
        }
    }

    fn serialize_bunyan_core_fields(
        &self,
        map_serializer: &mut impl SerializeMap<Error = serde_json::Error>,
        message: &str,
        level: &Level,
    ) -> Result<(), std::io::Error> {
        map_serializer.serialize_entry(BUNYAN_VERSION, &self.bunyan_version)?;
        map_serializer.serialize_entry(NAME, &self.name)?;
        map_serializer.serialize_entry(MESSAGE, &message)?;
        map_serializer.serialize_entry(LEVEL, &to_bunyan_level(level))?;
        map_serializer.serialize_entry(HOSTNAME, &self.hostname)?;
        map_serializer.serialize_entry(PID, &self.pid)?;
        map_serializer.serialize_entry(
            TIME,
            &DateTime::now_or_zero().to_rfc3339_opts(SecondsFormat::Millis, true),
        )?;
        Ok(())
    }

    /// Given a span, it serialised it to a in-memory buffer (vector of bytes).
    fn serialize_span<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        let mut map_serializer = serializer.serialize_map(None)?;
        let message = format_span_context(span, ty);
        self.serialize_bunyan_core_fields(&mut map_serializer, &message, span.metadata().level())?;
        // Additional metadata useful for debugging
        // They should be nested under `src` (see https://github.com/trentm/node-bunyan#src )
        // but `tracing` does not support nested values yet
        map_serializer.serialize_entry("target", span.metadata().target())?;
        map_serializer.serialize_entry("line", &span.metadata().line())?;
        map_serializer.serialize_entry("file", &span.metadata().file())?;

        // Add all default fields
        for (key, value) in self.default_fields.iter() {
            if !BUNYAN_RESERVED_FIELDS.contains(&key.as_str()) {
                map_serializer.serialize_entry(key, &serde_json::Value::from(value))?;
            } else {
                tracing::info!(
                    "{} is a reserved field in the bunyan log format. Skipping it.",
                    key
                );
            }
        }

        let extensions = span.extensions();
        if let Some(visitor) = extensions.get::<Storage>() {
            for (key, value) in visitor.values() {
                if !BUNYAN_RESERVED_FIELDS.contains(key) {
                    map_serializer.serialize_entry(key, &serde_json::Value::from(value))?;
                } else {
                    tracing::info!(
                        "{} is a reserved field in the bunyan log format. Skipping it.",
                        key
                    );
                }
            }
        }
        map_serializer.end()?;
        Ok(buffer)
    }
}

impl Default for Bunyan {
    fn default() -> Self {
        Self {
            pid: 0,
            hostname: "".to_string(),
            bunyan_version: 0,
            name: "".to_string(),
            default_fields: Default::default(),
        }
    }
}

/// Ensure consistent formatting of the span context.
///
/// Example: "[AN_INTERESTING_SPAN - START]"
fn format_span_context<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
    span: &SpanRef<S>,
    ty: Type,
) -> String {
    format!("[{} - {}]", span.metadata().name().to_uppercase(), ty)
}

/// Ensure consistent formatting of event message.
///
/// Examples:
/// - "[AN_INTERESTING_SPAN - EVENT] My event message" (for an event with a parent span)
/// - "My event message" (for an event without a parent span)
fn format_event_message<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
    current_span: &Option<SpanRef<S>>,
    event: &Event,
    event_visitor: &Storage<'_>,
) -> String {
    // Extract the "message" field, if provided. Fallback to the target, if missing.
    let mut message = event_visitor
        .values()
        .get("message")
        .map(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .flatten()
        .unwrap_or_else(|| event.metadata().target())
        .to_owned();

    // If the event is in the context of a span, prepend the span name to the message.
    if let Some(span) = &current_span {
        message = format!("{} {}", format_span_context(span, Type::Event), message);
    }

    message
}

impl LogFormatter for Bunyan {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        &self,
        layer: &LogLayer<W, F>,
    ) -> Self {
        Self {
            pid: layer.proc_id().clone().unwrap_or(0),
            hostname: layer.hostname().clone().unwrap_or("localhost".to_string()),
            bunyan_version: 0,
            name: layer.application().clone().unwrap_or("app".to_string()),
            default_fields: Default::default(),
        }
    }

    fn format_event<S: Subscriber + for<'a> LookupSpan<'a>>(
        &mut self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String {
        let mut event_visitor = Storage::default();
        event.record(&mut event_visitor);

        // Opting for a closure to use the ? operator and get more linear code.
        let format = || {
            let mut buffer = Vec::new();

            let mut serializer = serde_json::Serializer::new(&mut buffer);
            let mut map_serializer = serializer.serialize_map(None)?;

            let message = format_event_message(&current_span, event, &event_visitor);
            self.serialize_bunyan_core_fields(
                &mut map_serializer,
                &message,
                event.metadata().level(),
            )?;
            // Additional metadata useful for debugging
            // They should be nested under `src` (see https://github.com/trentm/node-bunyan#src )
            // but `tracing` does not support nested values yet
            map_serializer.serialize_entry("target", event.metadata().target())?;
            map_serializer.serialize_entry("line", &event.metadata().line())?;
            map_serializer.serialize_entry("file", &event.metadata().file())?;

            // Add all default fields
            for (key, value) in self.default_fields.iter().filter(|(key, _)| {
                key.as_str() != "message" && !BUNYAN_RESERVED_FIELDS.contains(&key.as_str())
            }) {
                map_serializer.serialize_entry(key, &serde_json::Value::from(value))?;
            }

            // Add all the other fields associated with the event, expect the message we already used.
            for (key, value) in event_visitor
                .values()
                .iter()
                .filter(|(&key, _)| key != "message" && !BUNYAN_RESERVED_FIELDS.contains(&key))
            {
                map_serializer.serialize_entry(key, &serde_json::Value::from(value))?;
            }

            // Add all the fields from the current span, if we have one.
            if let Some(span) = &current_span {
                let extensions = span.extensions();
                if let Some(visitor) = extensions.get::<Storage>() {
                    for (key, value) in visitor.values() {
                        if !BUNYAN_RESERVED_FIELDS.contains(key) {
                            map_serializer.serialize_entry(key, &serde_json::Value::from(value))?;
                        } else {
                            tracing::info!(
                                "{} is a reserved field in the bunyan log format. Skipping it.",
                                key
                            );
                        }
                    }
                }
            }
            map_serializer.end()?;
            Ok(buffer)
        };
        format()
            .map_err(|e: std::io::Error| e.to_string())
            .and_then(|t| String::from_utf8(t).map_err(|e| e.to_string()))
            .unwrap_or_else(|e| e)
    }

    fn format_span<S: Subscriber + for<'a> LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String {
        self.serialize_span(&span, ty)
            .map_err(|e: std::io::Error| e.to_string())
            .and_then(|t| String::from_utf8(t).map_err(|e| e.to_string()))
            .unwrap_or_else(|e| e)
    }
}
