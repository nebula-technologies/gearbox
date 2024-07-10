use alloc::string::{String, ToString};
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

#[cfg(test)]
mod tests {
    use super::*;
    use web_sys::wasm_bindgen::JsValue;

    #[test]
    fn test_error_display() {
        let error = Error::FailedToGetLocalStorage("test".to_string());
        assert_eq!(error.to_string(), "Failed to get local storage: test");

        let error = Error::SerializationError("test".to_string());
        assert_eq!(error.to_string(), "Failed to serialize data: test");

        let error = Error::NoLocalStorageSystemInitialized;
        assert_eq!(error.to_string(), "No local storage system initialized");

        let error = Error::NoDataWithKey("test".to_string());
        assert_eq!(error.to_string(), "No data with key: test");
    }

    #[test]
    fn test_error_from_jsvalue() {
        let js_value = JsValue::from("test");
        let error: Error = js_value.into();
        assert_eq!(error.to_string(), "Failed to get local storage: test");
    }

    #[test]
    fn test_error_from_serde_error() {
        let serde_error: serde_json::Error = serde_json::from_str::<i32>("invalid").unwrap_err();
        let error: Error = serde_error.into();
        assert_eq!(
            error.to_string(),
            "Failed to get local storage: expected i32 at line 1 column 1"
        );
    }
}
