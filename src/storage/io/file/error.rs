use alloc::{boxed::Box, string::String};
use core::fmt::{Display, Formatter};
use serde_json::Error as JsonError;
use serde_yaml::Error as YamlError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    NoFileConfigured(String),
    ShouldHaveBeenInfallible,
    JsonError(JsonError),
    YamlError(YamlError),
    FileDoesNotExist,
    ExtensionError(Box<dyn crate::error::tracer::ErrorDebug>),
    NoPath,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::NoFileConfigured(s) => write!(f, "No file configured: {}", s),
            Error::ShouldHaveBeenInfallible => write!(f, "Should have been infallible"),
            Error::JsonError(e) => write!(f, "JSON error: {}", e),
            Error::YamlError(e) => write!(f, "YAML error: {}", e),
            Error::FileDoesNotExist => write!(f, "File does not exist"),
            Error::ExtensionError(e) => write!(f, "Extension error: {:?}", e),
            Error::NoPath => write!(f, "No path provided for file"),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<JsonError> for Error {
    fn from(value: JsonError) -> Self {
        Self::JsonError(value)
    }
}

impl From<YamlError> for Error {
    fn from(value: YamlError) -> Self {
        Self::YamlError(value)
    }
}
