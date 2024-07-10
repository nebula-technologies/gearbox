use crate::net::http::request::header::Value;
use alloc::{
    slice::Iter,
    string::String,
    vec,
    vec::{IntoIter, Vec},
};
use core::ops::{Deref, DerefMut};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Values(pub Vec<Value>);

impl Values {
    // Returns an iterator that yields references to the elements.
    pub fn iter(&self) -> Iter<'_, Value> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }
    pub fn as_vec(&self) -> &Vec<Value> {
        &self.0
    }

    pub fn to_header_string(&self) -> String {
        self.as_vec()
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl Deref for Values {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = IntoIter<Value>;

    fn into_iter(self) -> IntoIter<Value> {
        self.0.into_iter()
    }
}

impl From<&Value> for Values {
    fn from(value: &Value) -> Self {
        Values(vec![value.clone()])
    }
}

impl From<Vec<String>> for Values {
    fn from(v: Vec<String>) -> Self {
        Values(v.into_iter().map(|t| Value::from(t)).collect())
    }
}

impl From<Vec<&str>> for Values {
    fn from(v: Vec<&str>) -> Self {
        Values(v.into_iter().map(|t| Value::from(t)).collect())
    }
}

impl From<&[String]> for Values {
    fn from(v: &[String]) -> Self {
        Values(v.iter().map(|t| Value::from(t)).collect())
    }
}

impl From<&[&str]> for Values {
    fn from(v: &[&str]) -> Self {
        Values(v.iter().map(|t| Value::from(*t)).collect())
    }
}

impl From<String> for Values {
    fn from(v: String) -> Self {
        Values(vec![Value::from(v)])
    }
}

impl From<&str> for Values {
    fn from(value: &str) -> Self {
        Values(vec![Value::from(value)])
    }
}

impl From<Value> for Values {
    fn from(value: Value) -> Self {
        Values(vec![value])
    }
}

impl From<Vec<Value>> for Values {
    fn from(v: Vec<Value>) -> Self {
        Values(v)
    }
}

impl From<Values> for Vec<String> {
    fn from(values: Values) -> Self {
        values
            .into_iter()
            .map(|t| t.try_into().unwrap_or("".to_string()))
            .collect()
    }
}
