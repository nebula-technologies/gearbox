use crate::log::tracing::subscriber::layer::{LogLayer, Storage, Type};
use alloc::string::String;
use tracing::log::Log;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::SpanRef;

pub trait LogFormatter {
    fn log_layer_defaults<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default>(
        &self,
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
