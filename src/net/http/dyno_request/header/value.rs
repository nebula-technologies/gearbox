use alloc::{string::String, vec::Vec};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value(pub Vec<u8>);

impl Value {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value(v.as_bytes().to_vec())
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value(v.as_bytes().to_vec())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value(v.as_bytes().to_vec())
    }
}

impl TryFrom<Value> for String {
    type Error = alloc::string::FromUtf8Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        String::from_utf8(v.0)
    }
}

impl TryFrom<Value> for reqwest::header::HeaderValue {
    type Error = reqwest::header::InvalidHeaderValue;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        reqwest::header::HeaderValue::try_from(&value)
    }
}
impl TryFrom<&Value> for reqwest::header::HeaderValue {
    type Error = reqwest::header::InvalidHeaderValue;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        reqwest::header::HeaderValue::from_bytes(&value.0)
    }
}

impl From<reqwest::header::HeaderValue> for Value {
    fn from(value: reqwest::header::HeaderValue) -> Self {
        Self::from(&value)
    }
}
impl From<&reqwest::header::HeaderValue> for Value {
    fn from(value: &reqwest::header::HeaderValue) -> Self {
        Value(value.as_bytes().to_vec())
    }
}
