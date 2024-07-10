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
