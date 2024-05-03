use core::fmt::{Display, Formatter};
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    FailedToGetLocalStorage(String),
    SerializationError(String),
    NoLocalStorageSystemInitialized,
    NoDataWithKey(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::FailedToGetLocalStorage(s) => {
                write!(f, "Failed to get local storage: {}", s)
            }
            Error::SerializationError(s) => {
                write!(f, "Failed to serialize data: {}", s)
            }
            Error::NoLocalStorageSystemInitialized => {
                write!(f, "No local storage system initialized")
            }
            Error::NoDataWithKey(s) => {
                write!(f, "No data with key: {}", s)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::FailedToGetLocalStorage(value.as_string().unwrap_or_default())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::FailedToGetLocalStorage(value.to_string())
    }
}
