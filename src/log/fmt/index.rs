use crate::log::fmt::value::Value;
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
};
use core::fmt;
use core::fmt::Display;
use core::ops;

pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;

    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;

    /// Panic if array index out of bounds. If key is not already in the object,
    /// insert it with a value of null. Panic if Value is a type that cannot be
    /// indexed into, except if Value is null then it can be treated as an empty
    /// object.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        match v {
            Value::Array(vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of JSON array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of JSON {}", self, Type(v)),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Map(map) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Map(map) => map.get_mut(self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = v {
            *v = Value::Map(BTreeMap::new());
        }
        match v {
            Value::Map(map) => map.entry(self.to_string()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in JSON {}", self, Type(v)),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    use alloc::string::String;
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
}

/// Used in panic messages.
struct Type<'a>(&'a Value);

impl<'a> Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Value::Null => formatter.write_str("null"),
            Value::Bool(_) => formatter.write_str("boolean"),
            Value::Int(_) => formatter.write_str("integer"),
            Value::UInt(_) => formatter.write_str("uinteger"),
            Value::Float(_) => formatter.write_str("float"),
            Value::String(_) => formatter.write_str("string"),
            Value::Array(_) => formatter.write_str("array"),
            Value::Map(_) => formatter.write_str("object"),
        }
    }
}

impl<I> ops::Index<I> for Value
where
    I: Index,
{
    type Output = Value;

    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Value
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::fmt::value::Value;
    use alloc::collections::BTreeMap;

    #[test]
    fn test_index_usize() {
        let array = Value::Array(vec![Value::Int(10), Value::Int(20)]);
        assert_eq!(Index::index_into(&0, &array), Some(&Value::Int(10)));
        assert_eq!(Index::index_into(&1, &array), Some(&Value::Int(20)));
        assert_eq!(Index::index_into(&2, &array), None);

        let mut array = Value::Array(vec![Value::Int(10), Value::Int(20)]);
        assert_eq!(
            Index::index_into_mut(&1, &mut array),
            Some(&mut Value::Int(20))
        );
        assert_eq!(Index::index_into_mut(&2, &mut array), None);

        let mut array = Value::Array(vec![Value::Int(10), Value::Int(20)]);
        assert_eq!(Index::index_or_insert(&1, &mut array), &mut Value::Int(20));
    }

    #[test]
    fn test_index_str() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), Value::String("value1".to_string()));
        let map = Value::Map(map);
        assert_eq!(
            Index::index_into(&"key1", &map),
            Some(&Value::String("value1".to_string()))
        );
        assert_eq!(Index::index_into(&"key2", &map), None);

        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), Value::String("value1".to_string()));
        let mut map = Value::Map(map);
        assert_eq!(
            Index::index_into_mut(&"key1", &mut map),
            Some(&mut Value::String("value1".to_string()))
        );
        assert_eq!(Index::index_into_mut(&"key2", &mut map), None);

        let mut map = Value::Null;
        assert_eq!(Index::index_or_insert(&"key1", &mut map), &mut Value::Null);
    }

    #[test]
    fn test_index_string() {
        let key = "key1".to_string();
        let mut map = BTreeMap::new();
        map.insert(key.clone(), Value::String("value1".to_string()));
        let map = Value::Map(map);
        assert_eq!(
            Index::index_into(&key, &map),
            Some(&Value::String("value1".to_string()))
        );
        assert_eq!(Index::index_into(&"key2".to_string(), &map), None);

        let mut map = BTreeMap::new();
        map.insert(key.clone(), Value::String("value1".to_string()));
        let mut map = Value::Map(map);
        assert_eq!(
            Index::index_into_mut(&key, &mut map),
            Some(&mut Value::String("value1".to_string()))
        );
        assert_eq!(Index::index_into_mut(&"key2".to_string(), &mut map), None);

        let mut map = Value::Null;
        assert_eq!(Index::index_or_insert(&key, &mut map), &mut Value::Null);
    }

    #[test]
    fn test_index_reference() {
        let array = Value::Array(vec![Value::Int(10), Value::Int(20)]);
        let index = &1;
        assert_eq!(Index::index_into(index, &array), Some(&Value::Int(20)));

        let key = "key1";
        let mut map = BTreeMap::new();
        map.insert(key.to_string(), Value::String("value1".to_string()));
        let map = Value::Map(map);
        assert_eq!(
            Index::index_into(key, &map),
            Some(&Value::String("value1".to_string()))
        );
    }
}
