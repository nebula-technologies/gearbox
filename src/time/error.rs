use crate::time::utils::CompareError;
use alloc::string::{String, ToString};
use core::fmt::Display;
use core::num::ParseIntError;
#[cfg(feature = "std")]
use std::time::SystemTimeError;

#[derive(Debug, Clone)]
pub enum Error {
    #[cfg(feature = "std")]
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "std")]
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
