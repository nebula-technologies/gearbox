use crate::log::fmt::formatter::LogFormatter;
use crate::log::fmt::get_exec_name;
use crate::log::fmt::log_value::LogValue;
use crate::log::fmt::storage::Storage;
use alloc::{string::String, vec::Vec};
use core::fmt;
use core::marker::PhantomData;
use core::result::Result;
use hashbrown::HashMap;
use std::io::Write;
use tracing::span::Attributes;
use tracing::{Event, Id, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;
#[allow(unused)]
const TRACING_COMMON: &'static str = "tracing-log.";
#[allow(unused)]
const TRACING_OVERWRITES: &'static str = "tracing-log.overwrites.";

pub enum Error {}

pub struct LogLayer<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default> {
    make_writer: W,
    version: Option<i32>,
    hostname: Option<String>,
    application: Option<String>,
    proc_id: Option<u32>,
    phantom: PhantomData<F>,
}

impl<W: for<'a> MakeWriter<'a> + 'static, F: LogFormatter + Default> LogLayer<W, F> {
    pub fn new(name: Option<String>, make_writer: W, _: F) -> Self {
        Self::with_default_fields(name, make_writer, HashMap::new())
    }

    pub fn with_default_fields(
        name: Option<String>,
        make_writer: W,
        _default_fields: HashMap<String, LogValue>,
    ) -> Self {
        #[cfg(all(any(unix, windows), feature = "std"))]
        let hostname = Option::from(
            crate::net::hostname::gethostname()
                .to_string_lossy()
                .into_owned(),
        );

        #[cfg(all(not(any(unix, windows)), feature = "std"))]
        let hostname = None;

        Self {
            make_writer,
            version: Option::from(1),
            proc_id: Option::from(crate::common::process::id()),
            hostname,
            application: name.or_else(|| get_exec_name()),
            phantom: Default::default(),
        }
    }

    pub fn emit(&self, mut buffer: Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_all(b"\n")?;
        self.make_writer.make_writer().write_all(&buffer)
    }

    pub fn version(&self) -> &Option<i32> {
        &self.version
    }

    pub fn proc_id(&self) -> &Option<u32> {
        &self.proc_id
    }

    pub fn hostname(&self) -> &Option<String> {
        &self.hostname
    }

    pub fn application(&self) -> &Option<String> {
        &self.application
    }
}

impl<S, W, F> Layer<S> for LogLayer<W, F>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    W: for<'a> MakeWriter<'a> + 'static,
    F: LogFormatter + Default + 'static,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Creating the SPAN from context.
        let current_span = ctx.lookup_current();

        // Event Visitor and storage initialization.
        let mut event_visitor = Storage::default();
        event.record(&mut event_visitor);

        let mut entry = F::from_log_layer(self);

        let _ = self.emit(
            entry
                .format_event(&current_span, event, &event_visitor)
                .into_bytes(),
        );
    }

    fn on_new_span(&self, _attrs: &Attributes, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut entry = F::from_log_layer(self);
        let _ = self.emit(entry.format_span(&span, Type::EnterSpan).into_bytes());
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let mut entry = F::from_log_layer(self);
        let _ = self.emit(entry.format_span(&span, Type::ExitSpan).into_bytes());
    }
}

/// The type of record we are dealing with: entering a span, exiting a span, an event.
#[derive(Clone, Debug)]
pub enum Type {
    EnterSpan,
    ExitSpan,
    Event,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Type::EnterSpan => "START",
            Type::ExitSpan => "ENDED",
            Type::Event => "EVENT",
        };
        write!(f, "{}", repr)
    }
}
