use crate::log::fmt::log_layer::LogLayer;
use crate::log::fmt::log_layer::Type;
use crate::log::fmt::storage::Storage;
use alloc::string::String;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::SpanRef;

pub trait LogFormatter {
    fn from_log_layer<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        layer: &LogLayer<W, F>,
    ) -> Self;
    fn format_event<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        current_span: &Option<SpanRef<S>>,
        event: &Event,
        event_visitor: &Storage<'_>,
    ) -> String;

    fn format_span<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &mut self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> String;
}
