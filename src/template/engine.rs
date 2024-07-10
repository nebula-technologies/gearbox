use crate::error::tracer::DynTracerError;
use crate::error::TracerError;
use crate::template::pipeline::{Pipeline, PipelineValue};
use crate::template::pipelines::date_time::DatePipeline;
use crate::template::pipelines::{PipelineManager, PrefixPipeline};
use crate::{tracer_dyn_err, tracer_err};
use alloc::boxed::Box;
use core::any::Any;
use core::fmt::Debug;
use hashbrown::HashMap;
use regex::Regex;
use spin::Mutex;

/// A global static for storing pipelines.
static PIPELINES: Mutex<Option<PipelineManager>> = Mutex::new(None);

/// TemplateEngine is responsible for rendering templates using context data and applying
/// pipelines for data transformations.
///
/// # Example
///
/// ```
/// use gearbox::template::{TemplateContext, TemplateEngine};
/// use gearbox::template::PipelineValue;
/// use hashbrown::HashMap;
///
/// let engine = TemplateEngine::new();
/// let mut context = TemplateContext::new();;
/// context.insert("name", Box::new("World".to_string()));
///
/// let result = engine.render("Hello, {{ name }}!", &context).unwrap();
/// assert_eq!(result, "Hello, World!");
/// ```
#[derive(Debug, Clone)]
pub struct TemplateEngine {
    pipelines: PipelineManager,
}

impl TemplateEngine {
    /// Returns the default set of pipelines.
    ///
    /// # Example
    ///
    /// ```rust,no_run,ignore
    /// let pipelines = TemplateEngine::get_pipelines_default();
    /// assert!(pipelines.contains_key("date"));
    /// ```
    fn get_pipelines_default() -> PipelineManager {
        let mut map = PipelineManager::default();
        map.insert("date".to_string(), DatePipeline::new().boxed_clone());
        map.insert("prefix".to_string(), PrefixPipeline::new().boxed_clone());
        map
    }

    /// Updates the pipeline with a new one.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pipeline.
    /// * `pipeline` - The pipeline to add.
    ///
    /// # Example
    ///
    /// ```
    /// use gearbox::template::TemplateEngine;
    /// use gearbox::template::pipelines::date_time::DatePipeline;
    ///
    /// TemplateEngine::update_pipeline("date", DatePipeline::new());
    /// ```
    pub fn update_pipeline<P: Pipeline + Send + Sync + 'static>(name: &str, pipeline: P) {
        let mut pipelines = PIPELINES.lock();
        pipelines
            .get_or_insert_with(Self::get_pipelines_default)
            .insert(name.to_string(), Box::new(pipeline));
    }

    /// Gets a specific pipeline by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pipeline.
    ///
    /// # Returns
    ///
    /// An optional boxed pipeline.
    ///
    /// # Example
    ///
    /// ```
    /// use gearbox::template::TemplateEngine;
    ///
    /// let pipeline = TemplateEngine::get_pipeline("date");
    /// assert!(pipeline.is_some());
    /// ```
    pub fn get_pipeline(name: &str) -> Option<Box<dyn Pipeline + Send + Sync>> {
        let mut pipelines = PIPELINES.lock();
        pipelines
            .get_or_insert_with(Self::get_pipelines_default)
            .get(name)
            .map(|t| t.boxed_clone())
    }

    /// Gets all available pipelines.
    ///
    /// # Returns
    ///
    /// A hashmap of all pipelines.
    ///
    /// # Example
    ///
    /// ```
    /// use gearbox::template::TemplateEngine;
    ///
    /// let pipelines = TemplateEngine::get_pipelines();
    /// assert!(pipelines.contains_key("date"));
    /// ```
    pub fn get_pipelines() -> PipelineManager {
        let mut pipelines = PIPELINES.lock();
        pipelines
            .get_or_insert_with(Self::get_pipelines_default)
            .clone()
    }

    /// Creates a new TemplateEngine instance.
    ///
    /// # Example
    ///
    /// ```
    /// use gearbox::template::TemplateEngine;
    ///
    /// let engine = TemplateEngine::new();
    /// ```
    pub fn new() -> Self {
        TemplateEngine {
            pipelines: Self::get_pipelines(),
        }
    }

    pub fn reload_pipelines(&mut self) {
        self.pipelines = Self::get_pipelines();
    }

    /// Renders a template using the provided context.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string.
    /// * `context` - A hashmap of context values.
    ///
    /// # Returns
    ///
    /// A result containing the rendered string or an error.
    ///
    /// # Example
    ///
    /// ```
    /// use gearbox::template::*;
    /// use hashbrown::HashMap;
    ///
    /// let engine = TemplateEngine::new();
    /// let mut context = TemplateContext::new();
    /// context.insert("name", Box::new("World".to_string()));
    ///
    /// let result = engine.render("Hello, {{ name }}!", &context).unwrap();
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn render(
        &self,
        template: &str,
        context: &TemplateContext,
    ) -> Result<String, DynTracerError> {
        let mut output = template.to_string();
        let re = Regex::new(r"\{\{\s*(.*?)\s*\}\}")
            .map_err(|_| tracer_dyn_err!("Failed to create regex"))?;

        for cap in re.captures_iter(&output.clone()) {
            let full_match = &cap[0];
            let mut parts = cap[1].split('|').map(str::trim);
            if let Some(key) = parts.next() {
                if let Some(initial) = context.get(key) {
                    let mut current_value = None;

                    for pipe_segment in parts {
                        let pipe_parts: Vec<&str> = pipe_segment.splitn(2, ':').collect();
                        let pipe_name = pipe_parts[0].trim();
                        let pipe_options = if pipe_parts.len() > 1 {
                            let option = pipe_parts[1].trim().replace(r"\'", "'");
                            if option.starts_with('\'') && option.ends_with('\'') {
                                option[1..option.len() - 1].to_string()
                            } else {
                                option
                            }
                        } else {
                            "".to_string()
                        };

                        if let Some(pipe) = self.pipelines.get(pipe_name) {
                            let pipeline = if pipe_options.is_empty() {
                                pipe.boxed_clone()
                            } else {
                                pipe.options(&pipe_options).boxed_clone()
                            };
                            if let Some(t) = current_value {
                                current_value = Some(pipeline.format(&t));
                            } else {
                                current_value = Some(pipeline.format(initial));
                            }
                        } else {
                            return Err(tracer_dyn_err!(format!(
                                "Pipeline '{}' not found",
                                pipe_name
                            )));
                        }
                    }

                    if let Some(current_value) = current_value {
                        output = output.replace(full_match, &current_value.to_string());
                    } else {
                        output = output.replace(full_match, &initial.to_string());
                    }
                }
            }
        }

        Ok(output)
    }
}

pub struct TemplateContext {
    variables: HashMap<String, Box<dyn PipelineValue + Send + Sync>>,
}

impl TemplateContext {
    pub fn new() -> Self {
        TemplateContext {
            variables: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: Box<dyn PipelineValue + Send + Sync>) {
        self.variables.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn PipelineValue + Send + Sync>> {
        self.variables.get(key)
    }
}

impl Clone for TemplateContext {
    fn clone(&self) -> Self {
        let mut variables = HashMap::new();
        for (k, v) in self.variables.iter() {
            variables.insert(k.clone(), v.boxed_clone());
        }
        TemplateContext { variables }
    }
}
