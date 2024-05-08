use crate::net::http::request::header::Value;
use alloc::{
    slice::Iter,
    string::String,
    vec,
    vec::{IntoIter, Vec},
};

#[derive(Clone)]
pub struct Values(pub Vec<Value>);

impl Values {
    // Returns an iterator that yields references to the elements.
    pub fn iter(&self) -> Iter<'_, Value> {
        self.0.iter()
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
