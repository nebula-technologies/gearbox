use crate::log::fmt::facility::Facility;
use crate::log::fmt::log_value::LogValue;
use crate::log::fmt::util::UtilTryInto;
use core::fmt::{Display, Formatter};
use tracing::Level as TracingLevel;

#[derive(Debug)]
pub enum SeverityError {
    ConversionError(ConversionError),
}

#[derive(Debug)]
pub enum ConversionError {
    IntegerOutOfBounds,
    FloatOutOfBounds,
    UnableToConvertBool,
    StringDoesNotMatchValidSeverity,
    UnableToConvertTimestamp,
    UnableToConvertSeverity,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Severity {
    Trace = 8,
    Debug = 7,
    Informational = 6,
    Notice = 5,
    Warning = 4,
    Error = 3,
    Critical = 2,
    Alert = 1,
    Emergency = 0,
}
impl Severity {
    pub fn to_level(&self, facility: Option<&Facility>) -> u32 {
        let facility_u32 = u32::from(facility.unwrap_or(&Facility::LocalUse7));
        let severity_u32 = u32::from(self);
        facility_u32 | severity_u32
    }
}
impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Severity::Trace => write!(f, "Trace"),
            Severity::Debug => write!(f, "Debug"),
            Severity::Informational => write!(f, "Informational"),
            Severity::Notice => write!(f, "Notice"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
            Severity::Critical => write!(f, "Critical"),
            Severity::Alert => write!(f, "Alert"),
            Severity::Emergency => write!(f, "Emergency"),
        }
    }
}
impl From<Severity> for u32 {
    fn from(s: Severity) -> Self {
        Self::from(&s)
    }
}
impl From<&Severity> for u32 {
    fn from(s: &Severity) -> Self {
        match s {
            Severity::Trace => 8,
            Severity::Debug => 7,
            Severity::Informational => 6,
            Severity::Notice => 5,
            Severity::Warning => 4,
            Severity::Error => 3,
            Severity::Critical => 2,
            Severity::Alert => 1,
            Severity::Emergency => 0,
        }
    }
}
impl From<Severity> for TracingLevel {
    fn from(t: Severity) -> Self {
        match t {
            Severity::Trace => TracingLevel::TRACE,
            Severity::Debug => TracingLevel::DEBUG,
            Severity::Informational => TracingLevel::INFO,
            Severity::Notice => TracingLevel::WARN,
            Severity::Warning => TracingLevel::WARN,
            Severity::Error => TracingLevel::ERROR,
            Severity::Critical => TracingLevel::ERROR,
            Severity::Alert => TracingLevel::ERROR,
            Severity::Emergency => TracingLevel::ERROR,
        }
    }
}
impl From<TracingLevel> for Severity {
    fn from(t: TracingLevel) -> Self {
        match t.as_str() {
            "TRACE" => Severity::Trace,
            "DEBUG" => Severity::Debug,
            "INFO" => Severity::Informational,
            "WARN" => Severity::Warning,
            "ERROR" => Severity::Error,
            _ => Severity::Error,
        }
    }
}
impl TryFrom<&str> for Severity {
    type Error = SeverityError;
    fn try_from(u: &str) -> Result<Self, <Severity as TryFrom<&str>>::Error> {
        match u {
            "debug" => Ok(Severity::Debug),
            "informational" => Ok(Severity::Informational),
            "notice" => Ok(Severity::Notice),
            "warning" => Ok(Severity::Warning),
            "error" => Ok(Severity::Error),
            "critical" => Ok(Severity::Critical),
            "alert" => Ok(Severity::Alert),
            "emergency" => Ok(Severity::Emergency),
            _ => Err(SeverityError::ConversionError(
                ConversionError::StringDoesNotMatchValidSeverity,
            )),
        }
    }
}
impl TryFrom<i64> for Severity {
    type Error = SeverityError;
    fn try_from(i: i64) -> Result<Self, <Severity as TryFrom<LogValue>>::Error> {
        match i {
            8 => Ok(Severity::Trace),
            7 => Ok(Severity::Debug),
            6 => Ok(Severity::Informational),
            5 => Ok(Severity::Notice),
            4 => Ok(Severity::Warning),
            3 => Ok(Severity::Error),
            2 => Ok(Severity::Critical),
            1 => Ok(Severity::Alert),
            0 => Ok(Severity::Emergency),
            _ => Err(SeverityError::ConversionError(
                ConversionError::IntegerOutOfBounds,
            )),
        }
    }
}
impl TryFrom<u64> for Severity {
    type Error = SeverityError;
    fn try_from(u: u64) -> Result<Self, <Severity as TryFrom<LogValue>>::Error> {
        u.util_try_into()
            .map_err(|_| SeverityError::ConversionError(ConversionError::FloatOutOfBounds))
            .and_then(|i: i64| Severity::try_from(i))
    }
}
impl TryFrom<f64> for Severity {
    type Error = SeverityError;
    fn try_from(f: f64) -> Result<Self, <Severity as TryFrom<LogValue>>::Error> {
        f.util_try_into()
            .map_err(|_| SeverityError::ConversionError(ConversionError::FloatOutOfBounds))
            .and_then(|i: i64| Severity::try_from(i))
    }
}
impl TryFrom<LogValue> for Severity {
    type Error = SeverityError;
    fn try_from(value: LogValue) -> Result<Self, <Severity as TryFrom<LogValue>>::Error> {
        Self::try_from(&value)
    }
}
impl TryFrom<&LogValue> for Severity {
    type Error = SeverityError;
    fn try_from(value: &LogValue) -> Result<Self, <Severity as TryFrom<LogValue>>::Error> {
        match value {
            LogValue::I64(i) => Severity::try_from(*i),
            LogValue::U64(u) => Severity::try_from(*u),
            LogValue::F64(f) => Severity::try_from(*f),
            LogValue::Bool(_) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertBool,
            )),
            LogValue::String(s) => Severity::try_from(s.as_str()),
            LogValue::Debug(s) => Severity::try_from(s.as_str()),
            LogValue::TimeStamp(_) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertTimestamp,
            )),
            LogValue::Severity(_t) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertSeverity,
            )),
        }
    }
}
impl From<&tracing::Level> for Severity {
    fn from(t: &tracing::Level) -> Self {
        match t {
            &tracing::Level::TRACE => Severity::Trace,
            &tracing::Level::ERROR => Severity::Error,
            &tracing::Level::DEBUG => Severity::Debug,
            &tracing::Level::WARN => Severity::Warning,
            &tracing::Level::INFO => Severity::Informational,
        }
    }
}
