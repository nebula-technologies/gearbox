use crate::prelude::sync::Arc;
use crate::service::framework::axumv1::{FrameworkConfig, StateController};

#[derive(Debug, Clone)]
pub struct FrameworkManager {
    config: FrameworkConfig,
    state: Arc<StateController>,
}

impl FrameworkManager {
    pub fn config(&self) -> &FrameworkConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut FrameworkConfig {
        &mut self.config
    }

    pub fn state(&self) -> &StateController {
        &self.state
    }

    pub fn set_state(&mut self, state: StateController) {
        self.state = Arc::new(state);
    }
}

impl Default for FrameworkManager {
    fn default() -> Self {
        FrameworkManager {
            config: FrameworkConfig::default(),
            state: Arc::new(StateController::default()),
        }
    }
}
