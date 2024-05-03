use super::{Header, Name, Value};
use crate::common::TryDefault;
use crate::net::http::request::header::values::Values;
use crate::rails::ext::RailsMapErrInto;
use hashbrown::HashMap;

pub struct HeaderMap {
    inner: HashMap<Name, Values>,
}

impl HeaderMap {
    pub fn get<K: Into<Name>>(&self, key: K) -> Option<Header> {
        let key = key.into();
        self.inner.get(&key).map(|t| Header(key.into(), t.clone()))
    }

    pub fn insert(&mut self, header: Header) -> &mut Self {
        self.inner.insert(header.0, header.1);
        self
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        HeaderMap {
            inner: HashMap::new(),
        }
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
        map.iter().for_each(|(name, value)| {
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
