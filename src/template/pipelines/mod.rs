pub mod date_time;
pub mod prefix;

use crate::template::Pipeline;
use core::ops::{Deref, DerefMut};
pub use date_time::DatePipeline;
use hashbrown::HashMap;
pub use prefix::PrefixPipeline;

#[derive(Debug, Default)]
pub struct PipelineManager(HashMap<String, Box<dyn Pipeline + Send + Sync>>);

impl Clone for PipelineManager {
    fn clone(&self) -> Self {
        let mut map = HashMap::new();
        for (key, value) in self.iter() {
            map.insert(key.clone(), value.boxed_clone());
        }
        Self(map)
    }
}

impl Deref for PipelineManager {
    type Target = HashMap<String, Box<dyn Pipeline + Send + Sync>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PipelineManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
