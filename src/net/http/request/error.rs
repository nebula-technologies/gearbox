use alloc::string::String;
use url::ParseError;

pub enum Error {
    UrlParser(ParseError),
    Request(reqwest::Error),
    NoUrl,
    HeaderValue(reqwest::header::InvalidHeaderValue),
    DeserializeContentType(String),
    DeserializeJson(serde_json::Error),
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
