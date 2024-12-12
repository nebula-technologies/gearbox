#[derive(Debug, Clone)]
pub enum LogOutput {
    Full,
    Minimal,
    Default,
    Human,
}

impl LogOutput {
    pub fn minimal(self) -> Self {
        Self::Minimal
    }
    pub fn full(self) -> Self {
        Self::Full
    }
    pub fn default(self) -> Self {
        Self::Default
    }
    pub fn human(self) -> Self {
        Self::Human
    }
}

impl Default for LogOutput {
    fn default() -> Self {
        Self::Default
    }
}
