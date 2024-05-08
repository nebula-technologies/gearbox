use alloc::string::{String, ToString};

pub enum Severity {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Emergency => "emergency",
            Severity::Alert => "alert",
            Severity::Critical => "critical",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Notice => "notice",
            Severity::Info => "info",
            Severity::Debug => "debug",
        }
    }

    pub fn as_int(&self) -> u8 {
        match self {
            Severity::Emergency => 0,
            Severity::Alert => 1,
            Severity::Critical => 2,
            Severity::Error => 3,
            Severity::Warning => 4,
            Severity::Notice => 5,
            Severity::Info => 6,
            Severity::Debug => 7,
        }
    }
}

impl ToString for Severity {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
