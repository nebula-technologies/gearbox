use crate::log::tracing::formatter::{Bunyan, DeepLogFormatter, Syslog};
use crate::log::tracing::layer::{LogLayer, Storage, Type};
use crate::log::tracing::LogFormatter as TracingLogFormatter;
use crate::prelude::tracing::{Event, Subscriber};
use crate::service::framework::axumv1::logger::DEFAULT_LOG_BACKEND;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::SpanRef;

#[derive(Clone, Debug)]
pub enum LogFormatter {
    DeepLog(Option<DeepLogFormatter>),
    Bunyan(Option<Bunyan>),
    Syslog(Option<Syslog>),
}

impl Default for LogFormatter {
    fn default() -> Self {
        match DEFAULT_LOG_BACKEND.read().as_ref() {
            Some(LogFormatter::Bunyan(_)) => LogFormatter::Bunyan(Some(Bunyan::default())),
            Some(LogFormatter::DeepLog(_)) => {
                LogFormatter::DeepLog(Some(DeepLogFormatter::default()))
            }
            Some(LogFormatter::Syslog(_)) => LogFormatter::Syslog(Some(Syslog::default())),
            None => LogFormatter::DeepLog(Some(DeepLogFormatter::default())),
        }
    }
}

impl From<DeepLogFormatter> for LogFormatter {
    fn from(f: DeepLogFormatter) -> Self {
        LogFormatter::DeepLog(Some(f))
    }
}

impl TracingLogFormatter for LogFormatter {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: TracingLogFormatter + Default>(
        &self,
        layer: &LogLayer<W, F>,
    ) -> Self {
        match self {
            LogFormatter::DeepLog(Some(formatter)) => {
                formatter.log_layer_defaults(layer);
                LogFormatter::DeepLog(Some(formatter.clone()))
            }
            LogFormatter::Bunyan(Some(formatter)) => {
                formatter.log_layer_defaults(layer);
                LogFormatter::Bunyan(Some(formatter.clone()))
            }
            LogFormatter::Syslog(Some(formatter)) => {
                formatter.log_layer_defaults(layer);
                LogFormatter::Syslog(Some(formatter.clone()))
            }
            _ => self.clone(),
        }
    }

    fn format_event<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String {
        match self {
            LogFormatter::DeepLog(Some(formatter)) => {
                formatter.format_event(current_span, event, event_visitor)
            }
            LogFormatter::Bunyan(Some(formatter)) => {
                formatter.format_event(current_span, event, event_visitor)
            }
            LogFormatter::Syslog(Some(formatter)) => {
                formatter.format_event(current_span, event, event_visitor)
            }
            _ => String::new(),
        }
    }

    fn format_span<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String {
        match self {
            LogFormatter::DeepLog(Some(formatter)) => formatter.format_span(span, ty),
            LogFormatter::Bunyan(Some(formatter)) => formatter.format_span(span, ty),
            LogFormatter::Syslog(Some(formatter)) => formatter.format_span(span, ty),
            _ => String::new(),
        }
    }
}
