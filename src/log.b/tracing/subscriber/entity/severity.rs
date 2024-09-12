use crate::log::tracing::subscriber::{
    entity::{ConversionError, Facility},
    ExtTryInto, Value,
};
use core::fmt::{Display, Formatter};
use tracing::Level as TracingLevel;

#[derive(Debug)]
pub enum SeverityError {
    ConversionError(ConversionError),
}
#[derive(Debug, PartialEq, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Emergency => "emergency",
            Self::Alert => "alert",
            Self::Critical => "critical",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Notice => "notice",
            Self::Informational => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }

    pub fn as_int(&self) -> u8 {
        match self {
            Self::Emergency => 0,
            Self::Alert => 1,
            Self::Critical => 2,
            Self::Error => 3,
            Self::Warning => 4,
            Self::Notice => 5,
            Self::Informational => 6,
            Self::Debug => 7,
            Self::Trace => 8,
        }
    }
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
            Self::Trace => write!(f, "Trace"),
            Self::Debug => write!(f, "Debug"),
            Self::Informational => write!(f, "Informational"),
            Self::Notice => write!(f, "Notice"),
            Self::Warning => write!(f, "Warning"),
            Self::Error => write!(f, "Error"),
            Self::Critical => write!(f, "Critical"),
            Self::Alert => write!(f, "Alert"),
            Self::Emergency => write!(f, "Emergency"),
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
            "TRACE" => Self::Trace,
            "DEBUG" => Self::Debug,
            "INFO" => Self::Informational,
            "WARN" => Self::Warning,
            "ERROR" => Self::Error,
            _ => Self::Error,
        }
    }
}
impl TryFrom<&str> for Severity {
    type Error = SeverityError;
    fn try_from(u: &str) -> Result<Self, <Severity as TryFrom<&str>>::Error> {
        match u {
            "debug" => Ok(Self::Debug),
            "informational" => Ok(Self::Informational),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "critical" => Ok(Self::Critical),
            "alert" => Ok(Self::Alert),
            "emergency" => Ok(Self::Emergency),
            _ => Err(SeverityError::ConversionError(
                ConversionError::StringDoesNotMatchValidValues,
            )),
        }
    }
}

impl TryFrom<&String> for Severity {
    type Error = SeverityError;
    fn try_from(u: &String) -> Result<Self, <Severity as TryFrom<&String>>::Error> {
        Self::try_from(u.as_str())
    }
}

impl TryFrom<String> for Severity {
    type Error = SeverityError;
    fn try_from(u: String) -> Result<Self, <Severity as TryFrom<String>>::Error> {
        Self::try_from(u.as_str())
    }
}

impl TryFrom<i64> for Severity {
    type Error = SeverityError;
    fn try_from(i: i64) -> Result<Self, <Severity as TryFrom<Value>>::Error> {
        match i {
            8 => Ok(Self::Trace),
            7 => Ok(Self::Debug),
            6 => Ok(Self::Informational),
            5 => Ok(Self::Notice),
            4 => Ok(Self::Warning),
            3 => Ok(Self::Error),
            2 => Ok(Self::Critical),
            1 => Ok(Self::Alert),
            0 => Ok(Self::Emergency),
            _ => Err(SeverityError::ConversionError(
                ConversionError::IntegerOutOfBounds,
            )),
        }
    }
}
impl TryFrom<u64> for Severity {
    type Error = SeverityError;
    fn try_from(u: u64) -> Result<Self, <Severity as TryFrom<Value>>::Error> {
        u.ext_try_into()
            .map_err(|_| SeverityError::ConversionError(ConversionError::FloatOutOfBounds))
            .and_then(|i: i64| Self::try_from(i))
    }
}
impl TryFrom<f64> for Severity {
    type Error = SeverityError;
    fn try_from(f: f64) -> Result<Self, <Severity as TryFrom<Value>>::Error> {
        f.ext_try_into()
            .map_err(|_| SeverityError::ConversionError(ConversionError::FloatOutOfBounds))
            .and_then(|i: i64| Self::try_from(i))
    }
}
impl TryFrom<Value> for Severity {
    type Error = SeverityError;
    fn try_from(value: Value) -> Result<Self, <Severity as TryFrom<Value>>::Error> {
        Self::try_from(&value)
    }
}
impl TryFrom<&Value> for Severity {
    type Error = SeverityError;
    fn try_from(value: &Value) -> Result<Self, <Severity as TryFrom<Value>>::Error> {
        match value {
            Value::Int(i) => Self::try_from(*i),
            Value::UInt(u) => Self::try_from(*u),
            Value::Float(f) => Self::try_from(*f),
            Value::Bool(_) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertBool,
            )),
            Value::String(s) => Self::try_from(s.as_str()),
            Value::Debug(s) => Self::try_from(s.as_str()),
            Value::TimeStamp(_) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertTimestamp,
            )),
            Value::Severity(t) => Ok(t.clone()),
            Value::Null => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertNull,
            )),
            Value::Array(_) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertArray,
            )),
            Value::Map(_) => Err(SeverityError::ConversionError(
                ConversionError::UnableToConvertMap,
            )),
        }
    }
}
impl From<&tracing::Level> for Severity {
    fn from(t: &tracing::Level) -> Self {
        match t {
            &tracing::Level::TRACE => Self::Trace,
            &tracing::Level::ERROR => Self::Error,
            &tracing::Level::DEBUG => Self::Debug,
            &tracing::Level::WARN => Self::Warning,
            &tracing::Level::INFO => Self::Informational,
        }
    }
}
