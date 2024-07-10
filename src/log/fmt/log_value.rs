use crate::log::fmt::severity::Severity;
use crate::log::fmt::util::{IntegerConversionError, UtilTryInto};
use crate::time::DateTime;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt::{Debug, Display, Formatter};

pub enum LogValueConvertionError {
    FailedToConvertToDateTimeUtc(String),
    FailedToConvertToU32(String),
    FailedToConvertToI32(String),
    UnableToCreateTimestampFrom(i64),
    IntegerConversionError(IntegerConversionError),
}

impl From<IntegerConversionError> for LogValueConvertionError {
    fn from(e: IntegerConversionError) -> Self {
        Self::IntegerConversionError(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogValue {
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
    String(String),
    TimeStamp(DateTime),
    Severity(Severity),
    Debug(String),
}

impl From<i64> for LogValue {
    fn from(v: i64) -> Self {
        LogValue::I64(v)
    }
}

impl From<u64> for LogValue {
    fn from(v: u64) -> Self {
        LogValue::U64(v)
    }
}

impl From<f64> for LogValue {
    fn from(v: f64) -> Self {
        LogValue::F64(v)
    }
}

impl From<bool> for LogValue {
    fn from(v: bool) -> Self {
        LogValue::Bool(v)
    }
}

impl From<String> for LogValue {
    fn from(v: String) -> Self {
        LogValue::String(v)
    }
}

impl From<&str> for LogValue {
    fn from(v: &str) -> Self {
        LogValue::String(v.to_string())
    }
}

impl Display for LogValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            LogValue::I64(t) => write!(f, "{}", t),
            LogValue::U64(t) => write!(f, "{}", t),
            LogValue::F64(t) => write!(f, "{}", t),
            LogValue::Bool(t) => write!(f, "{}", t),
            LogValue::String(t) => write!(f, "{}", t),
            LogValue::Debug(t) => write!(f, "{}", t),
            LogValue::TimeStamp(t) => write!(f, "{}", t),
            LogValue::Severity(t) => write!(f, "{}", t),
        }
    }
}

impl TryFrom<LogValue> for DateTime {
    type Error = LogValueConvertionError;

    fn try_from(value: LogValue) -> Result<Self, Self::Error> {
        match value {
            LogValue::I64(i) => Ok(DateTime::from_secs(i).into()),
            LogValue::U64(u) => Ok(DateTime::from_secs(u as i64).into()),
            LogValue::F64(f) => Ok(DateTime::from_secs(f as i64).into()),

            LogValue::Bool(_) => Err(LogValueConvertionError::FailedToConvertToDateTimeUtc(
                "LogValue is a boolean, unable to convert".to_string(),
            )),
            LogValue::String(s) => DateTime::from_str(s.as_str())
                .map_err(|e| LogValueConvertionError::FailedToConvertToDateTimeUtc(e.to_string())),
            LogValue::Debug(s) => DateTime::from_str(s.as_str())
                .map_err(|e| LogValueConvertionError::FailedToConvertToDateTimeUtc(e.to_string())),
            LogValue::TimeStamp(t) => Ok(t),
            LogValue::Severity(_t) => Err(LogValueConvertionError::FailedToConvertToDateTimeUtc(
                "LogValue is a Severity and cannot be convert".to_string(),
            )),
        }
    }
}

impl From<LogValue> for String {
    fn from(value: LogValue) -> Self {
        value.to_string()
    }
}

impl TryFrom<LogValue> for u32 {
    type Error = LogValueConvertionError;

    fn try_from(value: LogValue) -> Result<Self, Self::Error> {
        match value {
            LogValue::I64(i) => i.util_try_into().map_err(LogValueConvertionError::from),
            LogValue::U64(u) => u.util_try_into().map_err(LogValueConvertionError::from),
            LogValue::F64(f) => f.util_try_into().map_err(LogValueConvertionError::from),
            LogValue::Bool(_) => Err(LogValueConvertionError::FailedToConvertToU32(
                "LogValue is a Bool, unable to convert".to_string(),
            )),
            LogValue::String(_s) => Err(LogValueConvertionError::FailedToConvertToU32(
                "LogValue is a String, unable to convert".to_string(),
            )),
            LogValue::Debug(_s) => Err(LogValueConvertionError::FailedToConvertToU32(
                "LogValue is a String, unable to convert".to_string(),
            )),
            LogValue::TimeStamp(t) => Ok(t.to_unix() as u32),
            LogValue::Severity(_t) => Err(LogValueConvertionError::FailedToConvertToU32(
                "LogValue is a Severity and cannot be convert".to_string(),
            )),
        }
    }
}

impl TryFrom<LogValue> for i32 {
    type Error = LogValueConvertionError;

    fn try_from(value: LogValue) -> Result<Self, Self::Error> {
        match value {
            LogValue::I64(i) => i.util_try_into().map_err(LogValueConvertionError::from),
            LogValue::U64(u) => u.util_try_into().map_err(LogValueConvertionError::from),
            LogValue::F64(f) => f.util_try_into().map_err(LogValueConvertionError::from),
            LogValue::Bool(_) => Err(LogValueConvertionError::FailedToConvertToI32(
                "LogValue is a Bool, unable to convert".to_string(),
            )),
            LogValue::String(_s) => Err(LogValueConvertionError::FailedToConvertToI32(
                "LogValue is a String, unable to convert".to_string(),
            )),
            LogValue::Debug(_s) => Err(LogValueConvertionError::FailedToConvertToI32(
                "LogValue is a String, unable to convert".to_string(),
            )),
            LogValue::TimeStamp(t) => Ok(t.to_unix() as i32),
            LogValue::Severity(_t) => Err(LogValueConvertionError::FailedToConvertToI32(
                "LogValue is a Severity and cannot be convert".to_string(),
            )),
        }
    }
}

impl From<LogValue> for Vec<String> {
    fn from(value: LogValue) -> Self {
        vec![value.to_string()]
    }
}

impl From<u32> for LogValue {
    fn from(u: u32) -> Self {
        LogValue::U64(u as u64)
    }
}
impl From<Option<u32>> for LogValue {
    fn from(opt: Option<u32>) -> Self {
        match opt {
            None => LogValue::Debug("".to_string()),
            Some(u) => LogValue::U64(u as u64),
        }
    }
}
impl From<Option<&str>> for LogValue {
    fn from(opt: Option<&str>) -> Self {
        match opt {
            None => LogValue::Debug("".to_string()),
            Some(s) => LogValue::String(s.to_string()),
        }
    }
}

#[cfg(feature = "bunyan")]
impl From<&LogValue> for serde_json::Value {
    fn from(v: &LogValue) -> Self {
        match v {
            LogValue::I64(t) => t.clone().into(),
            LogValue::U64(t) => t.clone().into(),
            LogValue::F64(t) => t.clone().into(),
            LogValue::Bool(t) => t.clone().into(),
            LogValue::String(t) => t.clone().into(),
            LogValue::TimeStamp(t) => t.to_string().into(),
            LogValue::Severity(t) => t.to_string().into(),
            LogValue::Debug(t) => t.clone().into(),
        }
    }
}
