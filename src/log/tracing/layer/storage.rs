use super::super::Value;
use crate::time::DateTime;
use alloc::{borrow::ToOwned, format};
use core::fmt;
use core::ops::{Deref, DerefMut};
use hashbrown::HashMap;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Record};
use tracing::{Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

/// This layer is only concerned with information storage, it does not do any formatting or provide any output.
///
/// It's purpose is to store the fields associated to spans in an easy-to-consume format
/// for downstream layers concerned with emitting a formatted representation of
/// spans or events.
#[derive(Clone, Debug)]
pub struct StorageLayer;

#[derive(Clone, Debug)]
pub struct Storage<'a> {
    values: HashMap<&'a str, Value>,
}

impl<'a> Storage<'a> {
    pub fn values(&self) -> &HashMap<&'a str, Value> {
        &self.values
    }

    pub fn set(&mut self, key: &'a str, value: Value) {
        self.values.insert(key, value);
    }

    pub fn has(&self, key: &'a str) -> bool {
        self.values.contains_key(key)
    }
}

/// Get a new visitor, with an empty bag of key-value pairs.
impl Default for Storage<'_> {
    fn default() -> Self {
        let mut values = HashMap::new();
        values.insert("timestamp", Value::TimeStamp(DateTime::now()));
        Self { values }
    }
}

impl<'a> Deref for Storage<'a> {
    type Target = HashMap<&'a str, Value>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> DerefMut for Storage<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

/// Taken verbatim from tracing-subscriber
impl Visit for Storage<'_> {
    /// Visit a signed 64-bit integer value.
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.values.insert(field.name(), Value::from(value));
    }

    /// Visit an unsigned 64-bit integer value.
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.values.insert(field.name(), Value::from(value));
    }

    /// Visit a 64-bit floating point value.
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.values.insert(field.name(), Value::from(value));
    }

    /// Visit a boolean value.
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.values.insert(field.name(), Value::from(value));
    }

    /// Visit a string value.
    fn record_str(&mut self, field: &Field, value: &str) {
        self.values.insert(field.name(), Value::from(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        match field.name() {
            // Skip fields that are actually log metadata that have already been handled
            name if name.starts_with("log.") => (),
            name if name.starts_with("r#") => {
                self.values
                    .insert(&name[2..], Value::from(format!("{:?}", value)));
            }
            name => {
                self.values
                    .insert(name, Value::from(format!("{:?}", value)));
            }
        };
    }
}

impl<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>> Layer<S>
    for StorageLayer
{
    /// Span creation.
    /// This is the only occasion we have to store the fields attached to the span
    /// given that they might have been borrowed from the surrounding context.
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");

        // We want to inherit the fields from the parent span, if there is one.
        let mut visitor = if let Some(parent_span) = span.parent() {
            // Extensions can be used to associate arbitrary data to a span.
            // We'll use it to store our representation of its fields.
            // We create a copy of the parent visitor!
            let mut extensions = parent_span.extensions_mut();
            extensions
                .get_mut::<Storage>()
                .map(|v| v.to_owned())
                .unwrap_or_default()
        } else {
            Storage::default()
        };

        let mut extensions = span.extensions_mut();

        // Register all fields.
        // Fields on the new span should override fields on the parent span if there is a conflict.
        attrs.record(&mut visitor);
        // Associate the visitor with the Span for future usage via the Span's extensions
        extensions.insert(visitor);
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found, this is a bug");

        // Before you can associate a record to an existing Span, well, that Span has to be created!
        // We can thus rely on the invariant that we always associate a JsonVisitor with a Span
        // on creation (`new_span` method), hence it's safe to unwrap the Option.
        let mut extensions = span.extensions_mut();
        let visitor = extensions
            .get_mut::<Storage>()
            .expect("Visitor not found on 'record', this is a bug");
        // Register all new fields
        values.record(visitor);
    }

    /// When we enter a span **for the first time** save the timestamp in its extensions.
    fn on_enter(&self, span: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found, this is a bug");

        let mut extensions = span.extensions_mut();
        if extensions.get_mut::<DateTime>().is_none() {
            extensions.insert(DateTime::now());
        }
    }

    /// When we close a span, register how long it took in milliseconds.
    fn on_close(&self, span: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&span).expect("Span not found, this is a bug");

        // Using a block to drop the immutable reference to extensions
        // given that we want to borrow it mutably just below
        let elapsed_milliseconds = {
            let extensions = span.extensions();
            extensions
                .get::<DateTime>()
                .map(|i| i.elapsed().as_millis_since_epoch())
                // If `Instant` is not in the span extensions it means that the span was never
                // entered into.
                .unwrap_or(0)
        };

        let elapsed_milliseconds: u64 = { elapsed_milliseconds.try_into().unwrap_or_default() };

        let mut extensions_mut = span.extensions_mut();
        let visitor = extensions_mut
            .get_mut::<Storage>()
            .expect("Visitor not found on 'record', this is a bug");

        visitor
            .values
            .insert("elapsed_milliseconds", Value::from(elapsed_milliseconds));
    }
}
