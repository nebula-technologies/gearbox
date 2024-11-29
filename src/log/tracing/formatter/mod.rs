#[cfg(feature = "log-tracing-bunyan")]
pub mod bunyan;
#[cfg(feature = "log-tracing-deeplog")]
pub mod deeplog;
#[cfg(feature = "log-tracing-syslog")]
pub mod syslog;

use crate::log::tracing::layer::log_layer::LogEmitter;
use crate::log::tracing::layer::{LogLayer, Storage, Type};
use crate::prelude::sync::Arc;
use alloc::string::String;
use core::fmt::Debug;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::SpanRef;

#[cfg(feature = "log-tracing-bunyan")]
pub use bunyan::Bunyan;
#[cfg(feature = "log-tracing-deeplog")]
pub use deeplog::DeepLogFormatter;
#[cfg(feature = "log-tracing-syslog")]
pub use syslog::Syslog;

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
