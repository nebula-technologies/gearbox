#[derive(Debug, Clone)]
pub enum LogOutput {
    Full,
    Minimal,
    Default,
    Human,
}

impl LogOutput {
    pub fn minimal(mut self) -> Self {
        Self::Minimal
    }
    pub fn full(mut self) -> Self {
        Self::Full
    }
    pub fn default(mut self) -> Self {
        Self::Default
    }
    pub fn human(mut self) -> Self {
        Self::Human
    }
}

impl Default for LogOutput {
    fn default() -> Self {
        Self::Default
    }
}
