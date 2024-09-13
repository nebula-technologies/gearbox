use crate::log::tracing::entity::syslog::Facility;
use crate::log::tracing::entity::syslog::Severity;
use crate::log::tracing::layer::LogLayer;
use crate::log::tracing::{
    layer::{Storage, Type},
    LogFormatter, Value,
};
use crate::time::DateTime;
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt;
use core::ops::{Deref, DerefMut};
use hashbrown::HashMap;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::{LookupSpan, SpanRef};

#[derive(Debug, Default)]
pub struct Syslog {
    facility: Option<Facility>,
    severity: Option<Severity>,
    version: Option<i32>,
    timestamp: Option<DateTime>,
    hostname: Option<String>,
    application: Option<String>,
    proc_id: Option<u32>,
    message_id: Option<Vec<String>>,
    message: Option<String>,
    data: Option<StructuredData>,
    file: Option<String>,
    line: Option<u32>,
}

impl Syslog {
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

impl fmt::Display for Syslog {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        //<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"] BOMAn application event log entry...
        let default_string = "-".to_string();
        write!(f, "<{priority_value}>{version} {timestamp} {hostname} {app_name} {proc_id} {msgid} {structured_data} {msg}",
               priority_value = self.severity.as_ref().map(|t| t.to_level(self.facility.as_ref())).unwrap_or(0),
               version = self.version.map(|v| v.to_string()).as_ref().unwrap_or(&default_string),
               timestamp = self.timestamp.as_ref().map(|t| t.to_rfc3339()).as_ref().unwrap_or(&default_string),
               hostname = self.hostname.as_ref().unwrap_or(&default_string),
               app_name = self.application.as_ref().unwrap_or(&default_string),
               proc_id = self.proc_id.map(|t| t.to_string()).as_ref().unwrap_or(&default_string),
               msgid = self.message_id.as_ref().map(|t| t.join("-")).as_ref().unwrap_or(&default_string),
               structured_data = self.data.as_ref().unwrap_or(&StructuredData::default()).to_string(),
               msg = self.message.as_ref().unwrap_or(&default_string))
    }
}

impl LogFormatter for Syslog {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        &self,
        layer: &LogLayer<W, F>,
    ) -> Self {
        Self {
            facility: None,
            severity: None,
            version: Some(1),
            timestamp: Some(DateTime::now()),
            hostname: layer.hostname().clone(),
            application: layer.application().clone(),
            proc_id: layer.proc_id().clone(),
            message_id: None,
            message: None,
            data: None,
            file: None,
            line: None,
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
        self.message_id = current_span.as_ref().map(|t| {
            vec![
                t.id().into_non_zero_u64().to_string(),
                Type::Event.to_string(),
            ]
        });

        self.line = event.metadata().line();
        self.file = event.metadata().file().map(|t| t.to_string());
        self.severity = event_visitor
            .get("severity")
            .and_then(|t| t.try_into().ok())
            .or_else(|| event_visitor.get("level").and_then(|t| t.try_into().ok()))
            .or_else(|| Option::from(Severity::from(event.metadata().level())));

        self.to_string()
    }

    fn format_span<S: Subscriber + for<'a> LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String {
        self.message = Option::from(self.format_span_context(span, &ty));
        self.line = span.metadata().line();
        self.file = span.metadata().file().map(|t| t.to_string());
        self.severity = Option::from(Severity::from(span.metadata().level()));
        self.message_id = Option::from(vec![
            span.id().into_non_zero_u64().to_string(),
            ty.to_string(),
        ]);
        self.to_string()
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
