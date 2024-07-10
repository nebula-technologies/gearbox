use crate::log::fmt::index::Index;
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::{self, Debug};
use core::mem;

use core::str;

#[derive(Clone)]
pub enum Value {
    Null,

    Bool(bool),

    Int(i64),
    UInt(u64),
    Float(f64),

    String(String),

    Array(Vec<Value>),

    Map(BTreeMap<String, Value>),
}
impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(f1), Self::Bool(f2)) => f1 == f2,
            (Self::Int(f1), Self::Int(f2)) => f1 == f2,
            (Self::UInt(f1), Self::UInt(f2)) => f1 == f2,
            (Self::Float(f1), Self::Float(f2)) => f1.to_string() == f2.to_string(),
            (Self::String(f1), Self::String(f2)) => f1 == f2,
            (Self::Array(f1), Self::Array(f2)) => f1 == f2,
            (Self::Map(f1), Self::Map(f2)) => f1 == f2,
            _ => false,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                Debug::fmt(vec, formatter)
            }
            Value::Map(map) => {
                formatter.write_str("Map ")?;
                Debug::fmt(map, formatter)
            }
            Value::Int(i) => Debug::fmt(i, formatter),
            Value::UInt(u) => Debug::fmt(u, formatter),
            Value::Float(f) => Debug::fmt(f, formatter),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}
impl From<BTreeMap<String, Value>> for Value {
    fn from(t: BTreeMap<String, Value>) -> Self {
        Value::Map(t)
    }
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl Value {
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(list) => Some(list),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Value::Int(_) => true,
            Value::UInt(_) => true,
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Value::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_u64(&self) -> bool {
        match self {
            Value::UInt(_) => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            Value::UInt(n) => {
                if n <= &(i64::MAX as u64) {
                    Some(*n as i64)
                } else {
                    None
                }
            }
            Value::Float(n) => Some(n.round() as i64),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Int(n) => {
                if n.is_negative() {
                    None
                } else {
                    Some(*n as u64)
                }
            }
            Value::UInt(n) => Some(*n),
            Value::Float(_n) => None,
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(n) => Some(*n as f64),
            Value::UInt(n) => Some(*n as f64),
            Value::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    /// If the `Value` is a Null, returns (). Returns None otherwise.
    ///
    /// ```
    /// # use std::collections::BTreeMap;
    /// # use gearbox::log::fmt::value::Value;
    /// #
    /// let mut t = BTreeMap::new();
    /// t.insert("a".into(), Value::Null);
    /// t.insert("b".into(), false.into());
    /// let v: Value = t.into();
    ///
    /// assert_eq!(v["a"].as_null(), Some(()));
    ///
    /// // The boolean `false` is not null.
    /// assert_eq!(v["b"].as_null(), None);
    /// ```
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }
    /// Looks up a value by Pointer.
    ///
    /// Pointer defines a string syntax for identifying a specific value
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use gearbox::log::fmt::value::Value;
    ///
    /// let mut tree2 = BTreeMap::new();
    /// tree2.insert("y".to_string(),Value::Array(vec!["z".into(), "zz".into()]));
    ///
    /// let mut tree1 = BTreeMap::new();
    /// tree1.insert("x".to_string(),Value::Map(tree2));
    /// let data = Value::Map(tree1);
    ///
    ///
    /// assert_eq!(data.pointer("/x/y/1").unwrap(), &Value::from("zz"));
    /// assert_eq!(data.pointer("/a/b/c"), None);
    /// ```
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                Value::Map(map) => map.get(&token),
                Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
    /// Looks up a value by a Pointer and returns a mutable reference to
    /// that value.
    ///
    /// the Pointer defines a string syntax for identifying a specific value
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// # Example of Use
    ///
    /// ```
    ///
    /// use std::collections::BTreeMap;
    /// use gearbox::log::fmt::value::Value;
    ///
    /// let mut tree = BTreeMap::new();
    /// tree.insert("x".to_string(), Value::Float(1.0));
    /// tree.insert("y".to_string(), Value::Float(2.0));
    /// let mut value: Value = Value::Map(tree);
    ///
    /// // Check value using read-only pointer
    /// assert_eq!(value.pointer("/x"), Some(&Value::Float(1.0)));
    /// // Change value with direct assignment
    ///  *value.pointer_mut("/x").unwrap() = Value::Float(1.5);
    /// // Check that new value was written
    /// assert_eq!(value.pointer("/x"), Some(&Value::Float(1.5)));
    /// // Or change the value only if it exists
    /// value.pointer_mut("/x").map(|v| *v = Value::Float(1.5));
    ///
    /// // "Steal" ownership of a value. Can replace with any valid Value.
    /// let old_x = value.pointer_mut("/x").map(Value::take).unwrap();
    /// assert_eq!(old_x, Value::Float(1.5));
    /// assert_eq!(value.pointer("/x").unwrap(), &Value::Null);
    /// ```
    pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                Value::Map(map) => map.get_mut(&token),
                Value::Array(list) => parse_index(&token).and_then(move |x| list.get_mut(x)),
                _ => None,
            })
    }

    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

#[cfg(test)]
pub mod tests {
    use crate::log::fmt::value::Value;
    use std::collections::BTreeMap;

    #[test]
    fn confirm_values() {
        let v = Value::Int(9);
        assert!(v.is_i64());
        assert!(!v.is_f64());
        assert!(!v.is_u64());
        let v = Value::UInt(9);
        assert!(v.is_u64());
        assert!(!v.is_i64());
        assert!(!v.is_f64());
        let mut v = Value::Float(9.0);
        assert!(!v.is_u64());
        assert!(!v.is_i64());
        assert!(v.is_f64());
        v = Value::Array(Vec::new());
        assert!(v.is_array());
        v = Value::Bool(false);
        assert!(v.is_boolean());
        v = Value::Null;
        assert!(v.is_null());
        v = Value::String("".to_string());
        assert!(v.is_string());
    }

    #[test]
    fn pointer_mut_test() {
        let mut tree = BTreeMap::new();
        tree.insert("x".to_string(), Value::Float(1.0));
        tree.insert("y".to_string(), Value::Float(2.0));
        let mut value: Value = Value::Map(tree);

        // Check value using read-only pointer
        assert_eq!(value.pointer("/x"), Some(&Value::Float(1.0)));
        // Change value with direct assignment
        *value.pointer_mut("/x").unwrap() = Value::Float(1.5);
        // Check that new value was written
        assert_eq!(value.pointer("/x"), Some(&Value::Float(1.5)));
        // Or change the value only if it exists
        value.pointer_mut("/x").map(|v| *v = Value::Float(1.5));

        // "Steal" ownership of a value. Can replace with any valid Value.
        let old_x = value.pointer_mut("/x").map(Value::take).unwrap();
        assert_eq!(old_x, Value::Float(1.5));
        assert_eq!(value.pointer("/x").unwrap(), &Value::Null);
    }
}
