use crate::template::pipeline::{Pipeline, PipelineValue};
use crate::time::DateTime;
use core::any::Any;

/// A pipeline for formatting dates.
#[derive(Debug, Clone)]
pub struct PrefixPipeline {
    prefix: String,
}

unsafe impl Send for PrefixPipeline {}
unsafe impl Sync for PrefixPipeline {}

impl PrefixPipeline {
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
        PrefixPipeline {
            prefix: String::from(""),
        }
    }
}

impl Default for PrefixPipeline {
    fn default() -> Self {
        PrefixPipeline::new()
    }
}

impl Pipeline for PrefixPipeline {
    fn format(
        &self,
        pipe_object: &Box<dyn PipelineValue + Send + Sync>,
    ) -> Box<dyn PipelineValue + Send + Sync + 'static> {
        if let Some(t) = pipe_object.as_any().downcast_ref::<String>() {
            return Box::new(format!("{}{}", self.prefix, t));
        }
        Box::new("Invalid Date".to_string())
    }

    fn options(&self, options: &str) -> Box<dyn Pipeline + Send + Sync> {
        Box::new(PrefixPipeline {
            prefix: options.to_string(),
        })
    }

    fn boxed_clone(&self) -> Box<dyn Pipeline + Send + Sync> {
        Box::new(self.clone())
    }
}
