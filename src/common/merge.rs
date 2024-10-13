#[cfg(feature = "collections-hash-map")]
use crate::collections::HashMap;
use core::hash::Hash;

pub trait DataMerge<A> {
    fn data_merge(&mut self, a: A) -> &mut Self;
}

// Implementing DataMerge for Vec<T>
impl<T: Clone> DataMerge<Vec<T>> for Vec<T> {
    fn data_merge(&mut self, other: Vec<T>) -> &mut Self {
        self.extend(other); // Append the elements of `other` to `self`
        self
    }
}

#[cfg(feature = "collections-hash-map")]
// Implementing DataMerge for HashMap<K, V>
impl<K: Eq + Hash + Clone, V: Clone> DataMerge<HashMap<K, V>> for HashMap<K, V> {
    fn data_merge(&mut self, other: HashMap<K, V>) -> &mut Self {
        for (k, v) in other {
            self.insert(k, v); // Insert/overwrite each key-value pair from `other`
        }
        self
    }
}

// Implementing DataMerge for Option<T>
impl<T: DataMerge<T>> DataMerge<Option<T>> for Option<T> {
    fn data_merge(&mut self, other: Option<T>) -> &mut Self {
        match (self.as_mut(), other) {
            (Some(s), Some(o)) => {
                s.data_merge(o);
            }
            (None, Some(o)) => {
                *self = Some(o);
            }
            _ => {}
        }
        self
    }
}

// Implementing DataMerge for Option<T>
impl<T: DataMerge<T>> DataMerge<T> for Option<T> {
    fn data_merge(&mut self, other: T) -> &mut Self {
        if let Some(s) = self.as_mut() {
            s.data_merge(other);
        } else {
            *self = Some(other);
        }
        self
    }
}
// Implementing DataMerge for Result<T, E>
impl<T: Clone, E: Clone> DataMerge<Result<T, E>> for Result<T, E> {
    fn data_merge(&mut self, other: Result<T, E>) -> &mut Self {
        match other {
            Ok(v) => *self = Ok(v),   // Replace self with other if it's Ok
            Err(e) => *self = Err(e), // Replace self with other if it's Err
        }
        self
    }
}
