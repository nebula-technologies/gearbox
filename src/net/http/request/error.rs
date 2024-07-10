use crate::error::{DynTracerError, TracerError};
use alloc::string::String;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    UrlParser(ParseError),
    Request(reqwest::Error),
    NoUrl,
    HeaderValue(reqwest::header::InvalidHeaderValue),
    DeserializeContentType(String),
    DeserializeJson(serde_json::Error),
    BodyError(DynTracerError),
    NoMethod,
    NoHost,
    NoPath,
    NoHeaders,
    NoBody,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Request(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DeserializeJson(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::HeaderValue(e)
    }
}

impl From<DynTracerError> for Error {
    fn from(e: DynTracerError) -> Self {
        Error::BodyError(e)
    }
}
