pub enum LogOutput {
    Full,
    Minimal,
    Default,
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
}
