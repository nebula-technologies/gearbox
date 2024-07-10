use super::{Header, Name, Value};
use crate::net::http::request::header::values::Values;
use alloc::{string::ToString, vec::Vec};
use core::fmt;
use core::slice::Iter;
use crate_serde::de::{MapAccess, Visitor};
use crate_serde::ser::SerializeMap;
use crate_serde::{de, ser};
use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HeaderMap {
    inner: HashMap<Name, Values>,
}

impl HeaderMap {
    pub fn get<K: Into<Name>>(&self, key: K) -> Option<Header> {
        let key = key.into();
        self.inner.get(&key).map(|t| Header(key.into(), t.clone()))
    }

    pub fn get_mut<K: Into<Name>>(&mut self, key: K) -> Option<&mut Values> {
        let key = key.into();
        self.inner.get_mut(&key)
    }

    pub fn insert(&mut self, header: Header) -> &mut Self {
        self.inner.insert(header.0, header.1);
        self
    }

    pub fn extend(&mut self, headers: HeaderMap) -> &mut Self {
        headers.inner.iter().for_each(|(key, value)| {
            if let Some(t) = self.inner.get_mut(key) {
                t.extend(value.clone());
            } else {
                self.inner.insert(key.clone(), value.clone());
            }
        });
        self
    }
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, Name, Values> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, Name, Values> {
        self.inner.iter_mut()
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        HeaderMap {
            inner: HashMap::new(),
        }
    }
}

impl ser::Serialize for HeaderMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.inner.len()))?;
        for (k, v) in &self.inner {
            let key = &k.0;
            let values: Vec<String> =
                v.0.iter()
                    .map(|val| String::from_utf8(val.0.clone()).unwrap())
                    .collect();
            map.serialize_entry(key, &values)?;
        }
        map.end()
    }
}

impl<'de> de::Deserialize<'de> for HeaderMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct HeaderMapVisitor;

        impl<'de> Visitor<'de> for HeaderMapVisitor {
            type Value = HeaderMap;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of header names to values")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((key, values)) = access.next_entry::<String, Vec<String>>()? {
                    let name = Name(key);
                    let vals = values.into_iter().map(|v| Value(v.into_bytes())).collect();
                    map.insert(name, Values(vals));
                }
                Ok(HeaderMap { inner: map })
            }
        }

        deserializer.deserialize_map(HeaderMapVisitor)
    }
}

impl From<HashMap<String, Vec<String>>> for HeaderMap {
    fn from(map: HashMap<String, Vec<String>>) -> Self {
        let mut headers = HeaderMap::default();
        map.iter().for_each(|(key, values)| {
            let values: Values = values
                .iter()
                .map(Value::from)
                .collect::<Vec<Value>>()
                .into();
            headers.insert(Header(key.as_str().into(), values));
        });
        headers
    }
}

impl From<Vec<(String, String)>> for HeaderMap {
    fn from(vec: Vec<(String, String)>) -> Self {
        let mut headers = HeaderMap::default();
        vec.iter().for_each(|(key, value)| {
            headers.insert((key.to_string(), value.to_string()).into());
        });
        headers
    }
}

impl From<HeaderMap> for HashMap<String, Vec<String>> {
    fn from(map: HeaderMap) -> Self {
        map.inner
            .iter()
            .map(|(key, values)| {
                let values: Vec<String> = values.iter().map(ToString::to_string).collect();
                (key.to_string(), values)
            })
            .collect()
    }
}

impl TryFrom<HeaderMap> for reqwest::header::HeaderMap {
    type Error = reqwest::header::InvalidHeaderValue;
    fn try_from(map: HeaderMap) -> Result<Self, Self::Error> {
        Self::try_from(&map)
    }
}
impl TryFrom<&HeaderMap> for reqwest::header::HeaderMap {
    type Error = reqwest::header::InvalidHeaderValue;
    fn try_from(map: &HeaderMap) -> Result<Self, Self::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        for (key, values) in &map.inner {
            let key: reqwest::header::HeaderName = key.into();
            for value in values.iter() {
                headers.insert(key.clone(), reqwest::header::HeaderValue::try_from(value)?);
            }
        }

        Ok(headers)
    }
}

impl From<reqwest::header::HeaderMap> for HeaderMap {
    fn from(map: reqwest::header::HeaderMap) -> Self {
        Self::from(&map)
    }
}

impl From<&reqwest::header::HeaderMap> for HeaderMap {
    fn from(map: &reqwest::header::HeaderMap) -> Self {
        let mut headers = HeaderMap::default();
        map.iter().for_each(|(name, _value)| {
            let values: Values = map
                .get_all(&name.to_string())
                .iter()
                .map(Value::from)
                .collect::<Vec<Value>>()
                .into();
            headers.insert(Header(name.into(), values));
        });
        headers
    }
}

impl From<&HeaderMap> for HeaderMap {
    fn from(map: &HeaderMap) -> Self {
        map.clone()
    }
}
