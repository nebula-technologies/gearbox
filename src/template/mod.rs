//! ## Description:
//! The `template` module provides a robust and flexible mechanism for rendering templates in Rust, allowing dynamic content insertion and transformation through pipelines. This module is designed to provide high performance and safety in template rendering and data transformation. It includes various structures such as `TemplateEngine`, `TemplateContext`, `PipelineManager`, `DatePipeline`, and `PrefixPipeline`, each tailored for specific templating patterns. It combines the functionality of a template parser and a pipeline manager, offering versatility and simplicity in template rendering.
//!
//! ## Features
//!
//! - **TemplateEngine**: Responsible for rendering templates using context data and applying pipelines for data transformations.
//! - **TemplateContext**: Manages the context data for templates, allowing dynamic insertion and retrieval of values.
//! - **PipelineManager**: Manages the available pipelines for data transformation, supporting default and custom pipelines.
//! - **DatePipeline**: A pipeline for formatting dates.
//! - **PrefixPipeline**: A pipeline for prefixing strings.
//!
//! ## Benefits and Risks
//!
//! - **Versatility**: The module's flexibility enhances development by allowing dynamic template rendering and easy extension with custom pipelines.
//! - **Danger**: Potential risks include improper pipeline usage and performance overhead from locking mechanisms in multithreaded environments.
//!
//! ## Thread Safety
//!
//! The `TemplateEngine` and its associated components use atomic operations and locking mechanisms for thread safety. The `Mutex` used for `PIPELINES` ensures safe concurrent access, though it may introduce performance overhead.
//!
//! ## Breaking Cycles with Weak References
//!
//! The module does not directly use weak references but employs `Mutex` for safe concurrent access to shared resources.
//!
//! ## Cloning References
//!
//! Creating a new reference from an existing template engine or context is done using the `Clone` trait implemented for `TemplateEngine` and `TemplateContext`.
//!
//! ```rust
//! use gearbox::template::{TemplateEngine, TemplateContext};
//!
//! let engine = TemplateEngine::new();
//! let engine_clone = engine.clone();
//! let context = TemplateContext::new();
//! let context_clone = context.clone();
//! ```
//!
//! ## Deref Behavior
//!
//! `TemplateContext` provides direct access to its internal `HashMap` for managing context variables, allowing easy insertion and retrieval of values.
//!
//! ## Usage Examples
//!
//! **Sharing Some Immutable Data Between Threads**
//!
//! ```rust
//! use gearbox::template::{TemplateEngine, TemplateContext};
//! use std::thread;
//!
//! let engine = TemplateEngine::new();
//! let mut context = TemplateContext::new();
//! context.insert("name", Box::new("World".to_string()));
//!
//! for _ in 0..10 {
//!     let engine = engine.clone();
//!     let context = context.clone();
//!
//!     thread::spawn(move || {
//!         let result = engine.render("Hello, {{ name }}!", &context).unwrap();
//!         println!("{}", result);
//!     });
//! }
//! ```
//!
//! ### Sharing a Mutable Atomic Value
//!
//! ```rust
//! use gearbox::template::{TemplateEngine, TemplateContext};
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::sync::Arc;
//! use std::thread;
//!
//! let engine = TemplateEngine::new();
//! let val = Arc::new(AtomicUsize::new(5));
//!
//! for _ in 0..10 {
//!     let val = Arc::clone(&val);
//!
//!     thread::spawn(move || {
//!         let v = val.fetch_add(1, Ordering::Relaxed);
//!         println!("{}", v);
//!     });
//! }
//! ```
//!
//! ## Detailed Descriptions
//!
//! ### TemplateEngine
//!
//! The `TemplateEngine` structure is the core of the `template` module. It is responsible for rendering templates by applying context data and utilizing pipelines for data transformation. The `TemplateEngine` maintains a `PipelineManager` which holds all available pipelines.
//!
//! - **Fields**:
//!   - `pipelines`: A `PipelineManager` instance that manages all registered pipelines.
//!
//! - **Methods**:
//!   - `new() -> Self`: Creates a new `TemplateEngine` instance with default pipelines.
//!   - `get_pipelines_default() -> PipelineManager`: Returns the default set of pipelines.
//!   - `update_pipeline<P: Pipeline + Send + Sync + 'static>(name: &str, pipeline: P)`: Updates or adds a pipeline with the specified name.
//!   - `get_pipeline(name: &str) -> Option<Box<dyn Pipeline + Send + Sync>>`: Retrieves a specific pipeline by name.
//!   - `get_pipelines() -> PipelineManager`: Retrieves all available pipelines.
//!   - `render(&self, template: &str, context: &TemplateContext) -> Result<String, DynTracerError>`: Renders a template using the provided context.
//!
//! ```rust
//! use gearbox::template::{TemplateEngine, TemplateContext};
//!
//! let engine = TemplateEngine::new();
//! let mut context = TemplateContext::new();
//! context.insert("name", Box::new("World".to_string()));
//!
//! let result = engine.render("Hello, {{ name }}!", &context).unwrap();
//! assert_eq!(result, "Hello, World!");
//! ```
//!
//! ### TemplateContext
//!
//! The `TemplateContext` structure manages the context data used in templates. It allows for dynamic insertion and retrieval of values.
//!
//! - **Fields**:
//!   - `variables`: A `HashMap` that stores the context variables.
//!
//! - **Methods**:
//!   - `new() -> Self`: Creates a new `TemplateContext` instance.
//!   - `insert(&mut self, key: &str, value: Box<dyn PipelineValue + Send + Sync>)`: Inserts a new context variable.
//!   - `get(&self, key: &str) -> Option<&Box<dyn PipelineValue + Send + Sync>>`: Retrieves a context variable by key.
//!
//! ```rust
//! use gearbox::template::TemplateContext;
//!
//! let mut context = TemplateContext::new();
//! context.insert("key", Box::new("value".to_string()));
//! println!("TemplateContext value: {}", context.get("key").unwrap());
//! ```
//!
//! ### PipelineManager
//!
//! The `PipelineManager` structure manages the available pipelines for data transformation.
//!
//! - **Fields**:
//!   - `0`: A `HashMap` that stores the pipelines.
//!
//! - **Methods**:
//!   - Implements `Deref` and `DerefMut` traits to provide access to the internal `HashMap`.
//!   - Implements `Clone` trait to allow cloning of the `PipelineManager` along with its pipelines.
//!
//! ```rust,ignore
//! use gearbox::template::{PipelineManager, pipelines::DatePipeline, Pipeline};
//!
//! let mut manager = PipelineManager::default();
//! manager.insert("date".to_string(), DatePipeline::new().boxed_clone());
//! println!("PipelineManager contains date pipeline: {}", manager.contains_key("date"));
//! ```
//!
//! ### DatePipeline
//!
//! The `DatePipeline` structure is a pipeline for formatting dates. It implements the `Pipeline` trait.
//!
//! - **Fields**:
//!   - `format`: A `String` that stores the date format.
//!
//! - **Methods**:
//!   - `new() -> Self`: Creates a new `DatePipeline` with a default format.
//!   - `format(&self, pipe_object: &Box<dyn PipelineValue + Send + Sync>) -> Box<dyn PipelineValue + Send + Sync>`: Formats the input value using the pipeline's transformation.
//!   - `options(&self, options: &str) -> Box<dyn Pipeline + Send + Sync>`: Creates a new instance of the pipeline with the specified options.
//!   - `boxed_clone(&self) -> Box<dyn Pipeline + Send + Sync>`: Clones the pipeline into a boxed instance.
//!
//! ```rust
//! use gearbox::template::{Pipeline, pipelines::DatePipeline, PipelineValue};
//! use gearbox::time::DateTime;
//!
//! let date_pipeline = DatePipeline::new();
//! let date = DateTime::from_date(2024, 7, 1);
//! let value: Box<(dyn PipelineValue + Send + Sync + 'static)> = Box::new(date);
//! let formatted = date_pipeline.format(&value);
//! println!("Formatted date: {}", formatted);
//! ```
//!
//! ### PrefixPipeline
//!
//! The `PrefixPipeline` structure is a pipeline for prefixing strings. It implements the `Pipeline` trait.
//!
//! - **Fields**:
//!   - `prefix`: A `String` that stores the prefix.
//!
//! - **Methods**:
//!   - `new() -> Self`: Creates a new `PrefixPipeline` with a default prefix.
//!   - `format(&self, pipe_object: &Box<dyn PipelineValue + Send + Sync>) -> Box<dyn PipelineValue + Send + Sync>`: Formats the input value using the pipeline's transformation.
//!   - `options(&self, options: &str) -> Box<dyn Pipeline + Send + Sync>`: Creates a new instance of the pipeline with the specified options.
//!   - `boxed_clone(&self) -> Box<dyn Pipeline + Send + Sync>`: Clones the pipeline into a boxed instance.
//!
//! ```rust
//! use gearbox::template::{Pipeline, pipelines::PrefixPipeline, PipelineValue};
//!
//! let prefix_pipeline = PrefixPipeline::new();
//! let value: Box<(dyn PipelineValue + Send + Sync + 'static)> = Box::new("value".to_string());
//! let prefixed = prefix_pipeline.format(&value);
//! println!("Prefixed value: {}", prefixed);
//! ```
//!
//! ## Architectural Diagram
//!
//! ```mermaid
//! graph TD;
//!     TemplateEngine -->|Uses| PipelineManager;
//!     PipelineManager -->|Contains| DatePipeline;
//!     PipelineManager -->|Contains| PrefixPipeline;
//!     TemplateEngine -->|Uses| TemplateContext;
//!     TemplateContext -->|Contains| Variables;
//!     TemplateEngine -->|Uses| Pipelines;
//! ```

pub mod engine;
pub mod pipeline;
pub mod pipelines;

pub use engine::{TemplateContext, TemplateEngine};
pub use pipeline::{Pipeline, PipelineValue};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::engine::TemplateContext;
    use crate::template::pipelines::DatePipeline;
    use crate::time::DateTime;
    use core::{any::Any, fmt};

    #[derive(Debug, Clone)]
    struct TestValue(i32);

    impl PipelineValue for TestValue {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
            Box::new(Clone::clone(self))
        }
    }

    impl fmt::Display for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_date_pipeline_format() {
        let date_pipeline = DatePipeline::new();
        let date = DateTime::from_date(2024, 7, 1);
        let boxed_date: Box<dyn PipelineValue + Send + Sync> = Box::new(date);
        let formatted = date_pipeline.format(&boxed_date);
        assert_eq!(formatted.to_string(), "2024-07-01");

        let date_pipeline_custom = date_pipeline.options("%Y");
        let formatted_custom = date_pipeline_custom.format(&boxed_date);
        assert_eq!(formatted_custom.to_string(), "2024");
    }

    #[test]
    fn test_template_engine_render_single_pipeline() {
        let mut context = TemplateContext::new();
        context.insert("birthday", Box::new(DateTime::from_date(2024, 7, 1)));

        let engine = TemplateEngine::new();
        let template = "{{ birthday | date }}";
        let rendered = engine.render(template, &context);
        assert_eq!(rendered.unwrap(), "2024-07-01");
    }

    #[test]
    fn test_template_engine_render_with_options() {
        let mut context = TemplateContext::new();
        context.insert("birthday", Box::new(DateTime::from_date(2024, 7, 1)));

        let engine = TemplateEngine::new();
        let template_with_options = "{{ birthday | date:%Y }}";
        let rendered_with_options = engine.render(template_with_options, &context);
        assert_eq!(rendered_with_options.unwrap(), "2024");
    }

    #[test]
    fn test_template_engine_render_multiple_pipes() {
        let mut context = TemplateContext::new();
        context.insert("birthday", Box::new(DateTime::from_date(2024, 7, 1)));

        let engine = TemplateEngine::new();
        let multiple_pipes_template = "{{ birthday | date:%Y | prefix: 'Date: '  }}";
        let rendered_multiple_pipes = engine.render(multiple_pipes_template, &context);
        assert_eq!(rendered_multiple_pipes.unwrap(), "Date: 2024"); // assuming second pipe overrides the format
    }

    #[test]
    fn test_template_engine_with_custom_pipeline() {
        let mut context = TemplateContext::new();
        context.insert("value", Box::new(TestValue(42)));

        let mut engine = TemplateEngine::new();
        TemplateEngine::update_pipeline("double", TestValuePipeline::new());
        engine.reload_pipelines();

        let template = "{{ value | double }}";
        let rendered = engine.render(template, &context);
        assert_eq!(rendered.unwrap(), "84");
    }

    #[derive(Debug, Clone)]
    struct TestValuePipeline;

    impl TestValuePipeline {
        pub fn new() -> Self {
            TestValuePipeline
        }
    }

    impl Pipeline for TestValuePipeline {
        fn format(
            &self,
            pipe_object: &Box<dyn PipelineValue + Send + Sync>,
        ) -> Box<(dyn PipelineValue + Send + Sync + 'static)> {
            if let Some(value) = pipe_object.as_any().downcast_ref::<TestValue>() {
                return Box::new(TestValue(value.0 * 2));
            }
            Box::new("Invalid Value".to_string())
        }

        fn options(&self, _options: &str) -> Box<dyn Pipeline + Send + Sync> {
            Box::new(self.clone())
        }

        fn boxed_clone(&self) -> Box<dyn Pipeline + Send + Sync> {
            Box::new(self.clone())
        }
    }

    impl fmt::Display for TestValuePipeline {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestValuePipeline")
        }
    }

    #[test]
    fn test_render_no_placeholders() {
        let engine = TemplateEngine::new();
        let context = TemplateContext::new();
        let template = "Hello, World!";
        let result = engine.render(template, &context).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_render_with_missing_context() {
        let engine = TemplateEngine::new();
        let mut context = TemplateContext::new();
        context.insert("name", Box::new("John".to_string()));
        let template = "Hello, {{ name }} and {{ unknown }}!";
        let result = engine.render(template, &context).unwrap();
        assert_eq!(result, "Hello, John and {{ unknown }}!");
    }

    #[test]
    fn test_add_new_pipeline() {
        let mut engine = TemplateEngine::new();
        let custom_pipeline = TestValuePipeline::new();
        TemplateEngine::update_pipeline("custom", custom_pipeline);

        engine.reload_pipelines();
        let mut context = TemplateContext::new();
        context.insert("value", Box::new(TestValue(42)));

        let template = "{{ value | custom }}";
        let rendered = engine.render(template, &context).unwrap();
        assert_eq!(rendered, "84");
    }

    #[test]
    fn test_template_context_insert_and_get() {
        let mut context = TemplateContext::new();
        context.insert("name", Box::new("John".to_string()));

        let value = context.get("name");
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_string(), "John");
    }

    #[test]
    fn test_template_context_clone() {
        let mut context = TemplateContext::new();
        context.insert("name", Box::new("John".to_string()));
        let cloned_context = context.clone();

        let value = cloned_context.get("name");
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_string(), "John");
    }

    #[test]
    fn test_invalid_pipeline_in_template() {
        let engine = TemplateEngine::new();
        let mut context = TemplateContext::new();
        context.insert("name", Box::new("John".to_string()));

        let template = "Hello, {{ name | unknown }}!";
        let result = engine.render(template, &context);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().err_to_string(),
            "\"Pipeline 'unknown' not found\""
        );
    }

    #[test]
    fn test_concurrent_rendering() {
        use std::thread;

        let engine = TemplateEngine::new();
        let mut context = TemplateContext::new();
        context.insert("name", Box::new("World".to_string()));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let engine = engine.clone();
                let context = context.clone();
                thread::spawn(move || {
                    let template = "Hello, {{ name }}!";
                    let result = engine.render(template, &context).unwrap();
                    assert_eq!(result, "Hello, World!");
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
