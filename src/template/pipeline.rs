use alloc::boxed::Box;
use alloc::sync::Arc;
use core::any::Any;
use core::fmt::Debug;
use core::fmt::Display;

/// A trait for pipeline transformations.
pub trait Pipeline: Debug {
    /// Formats the input value using the pipeline's transformation.
    ///
    /// # Arguments
    ///
    /// * `pipe_object` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// A boxed transformed value.
    fn format(
        &self,
        pipe_object: &Box<dyn PipelineValue + Send + Sync>,
    ) -> Box<dyn PipelineValue + Send + Sync>;

    /// Creates a new instance of the pipeline with the specified options.
    ///
    /// # Arguments
    ///
    /// * `options` - The options for the pipeline.
    ///
    /// # Returns
    ///
    /// A boxed pipeline instance with the specified options.
    fn options(&self, options: &str) -> Box<dyn Pipeline + Send + Sync>;

    /// Clones the pipeline into a boxed instance.
    ///
    /// # Returns
    ///
    /// A boxed clone of the pipeline.
    fn boxed_clone(&self) -> Box<dyn Pipeline + Send + Sync>;
}

/// A trait for values that can be used in pipelines.
pub trait PipelineValue: Any + Display {
    /// Returns the value as a reference to `Any`.
    ///
    /// # Returns
    ///
    /// A reference to the value as `Any`.
    fn as_any(&self) -> &dyn Any;

    /// Clones the pipeline value into a boxed instance.
    ///
    /// # Returns
    ///
    /// A boxed clone of the pipeline.
    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync>;

    /// Clones the pipeline value into a boxed instance.
    ///
    /// # Returns
    ///
    /// A boxed clone of the pipeline.
    fn cloned(&self) -> Self
    where
        Self: Clone,
    {
        Clone::clone(self)
    }
}

impl PipelineValue for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
        Box::new(Clone::clone(self))
    }
}
impl PipelineValue for f32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
        Box::new(Clone::clone(self))
    }
}
impl PipelineValue for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
        Box::new(Clone::clone(self))
    }
}
impl PipelineValue for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
        Box::new(Clone::clone(self))
    }
}
impl PipelineValue for Arc<String> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
        Box::new(Clone::clone(self))
    }
}
