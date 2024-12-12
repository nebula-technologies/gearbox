use crate::service::framework::axum::FrameworkConfig;
use crate::sync::CommonContainerTrait;

#[derive(Debug, Clone)]
pub struct FrameworkManager<S>
where
    S: CommonContainerTrait,
{
    config: FrameworkConfig,
    state: S,
}

impl<S> FrameworkManager<S>
where
    S: CommonContainerTrait,
{
    pub fn config(&self) -> &FrameworkConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut FrameworkConfig {
        &mut self.config
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn set_state(&mut self, state: S) {
        self.state = state;
    }
}

impl<S> Default for FrameworkManager<S>
where
    S: CommonContainerTrait,
{
    fn default() -> Self {
        FrameworkManager {
            config: FrameworkConfig::default(),
            state: S::default(),
        }
    }
}
