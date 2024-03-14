use crate::time::utils::CompareError;
use std::fmt::Display;
use std::num::ParseIntError;
use std::time::SystemTimeError;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidSystemTime(SystemTimeError),
    StringParser(String),
    InvalidFormat(String),
    Comparison(CompareError),
    InvalidTimezone,
    InvalidTimestamp,
    InvalidPattern,
    CalculationOverflow(String),
    InvalidZoneState,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSystemTime(e) => write!(f, "Invalid system time: {}", e),
            Error::StringParser(e) => write!(f, "String parser error: {}", e),
            Error::InvalidFormat(e) => write!(f, "Invalid format: {}", e),
            Error::Comparison(e) => write!(f, "Comparison error: {}", e),
            Error::InvalidTimezone => write!(f, "Invalid timezone"),
            Error::InvalidTimestamp => write!(f, "Invalid String Timestamp"),
            Error::InvalidPattern => write!(f, "Invalid Pattern"),
            Error::CalculationOverflow(e) => write!(f, "Calculation overflow: {}", e),
            Error::InvalidZoneState => write!(f, "Invalid Zone State"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::StringParser(e.to_string())
    }
}

impl From<CompareError> for Error {
    fn from(e: CompareError) -> Self {
        Error::Comparison(e)
    }
}
