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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_as_str() {
        assert_eq!(Severity::Emergency.as_str(), "emergency");
        assert_eq!(Severity::Alert.as_str(), "alert");
        assert_eq!(Severity::Critical.as_str(), "critical");
        assert_eq!(Severity::Error.as_str(), "error");
        assert_eq!(Severity::Warning.as_str(), "warning");
        assert_eq!(Severity::Notice.as_str(), "notice");
        assert_eq!(Severity::Info.as_str(), "info");
        assert_eq!(Severity::Debug.as_str(), "debug");
    }

    #[test]
    fn test_severity_as_int() {
        assert_eq!(Severity::Emergency.as_int(), 0);
        assert_eq!(Severity::Alert.as_int(), 1);
        assert_eq!(Severity::Critical.as_int(), 2);
        assert_eq!(Severity::Error.as_int(), 3);
        assert_eq!(Severity::Warning.as_int(), 4);
        assert_eq!(Severity::Notice.as_int(), 5);
        assert_eq!(Severity::Info.as_int(), 6);
        assert_eq!(Severity::Debug.as_int(), 7);
    }

    #[test]
    fn test_severity_to_string() {
        assert_eq!(Severity::Emergency.to_string(), "emergency".to_string());
        assert_eq!(Severity::Alert.to_string(), "alert".to_string());
        assert_eq!(Severity::Critical.to_string(), "critical".to_string());
        assert_eq!(Severity::Error.to_string(), "error".to_string());
        assert_eq!(Severity::Warning.to_string(), "warning".to_string());
        assert_eq!(Severity::Notice.to_string(), "notice".to_string());
        assert_eq!(Severity::Info.to_string(), "info".to_string());
        assert_eq!(Severity::Debug.to_string(), "debug".to_string());
    }
}
