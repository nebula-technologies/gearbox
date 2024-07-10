use crate::template::pipeline::{Pipeline, PipelineValue};
use crate::time::DateTime;
use core::any::Any;

/// A pipeline for formatting dates.
#[derive(Debug, Clone)]
pub struct DatePipeline {
    format: String,
}

unsafe impl Send for DatePipeline {}
unsafe impl Sync for DatePipeline {}

impl DatePipeline {
    /// Creates a new DatePipeline with a default format.
    ///
    /// # Example
    ///
    /// ```
    /// use gearbox::template::pipelines::date_time::DatePipeline;
    ///
    /// let pipeline = DatePipeline::new();
    /// ```
    pub fn new() -> Self {
        DatePipeline {
            format: String::from("%Y-%m-%d"),
        }
    }
}

impl Default for DatePipeline {
    fn default() -> Self {
        DatePipeline::new()
    }
}

impl Pipeline for DatePipeline {
    fn format(
        &self,
        pipe_object: &Box<dyn PipelineValue + Send + Sync>,
    ) -> Box<dyn PipelineValue + Send + Sync + 'static> {
        fn to_any(a: &Box<dyn Any + Send + Sync>) -> &Box<dyn Any + Send + Sync> {
            a
        }

        if let Some(date) = pipe_object.as_any().downcast_ref::<DateTime>() {
            return Box::new(date.format_to_str(&self.format));
        }
        Box::new("Invalid Date".to_string())
    }

    fn options(&self, options: &str) -> Box<dyn Pipeline + Send + Sync> {
        Box::new(DatePipeline {
            format: options.to_string(),
        })
    }

    fn boxed_clone(&self) -> Box<dyn Pipeline + Send + Sync> {
        Box::new(self.clone())
    }
}
