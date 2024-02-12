use super::error::Error;
use super::LocalStorage;
use crate::common::TryDefault;
use crate::rails::ext::map_into::RailsMapErrInto;
use crate::rails::ext::merge::Merge;
use crate::storage::{json::JsonExt, yaml::YamlExt, KeyStoreExt};
use serde::de::DeserializeOwned;
use serde::Serialize;

impl KeyStoreExt for LocalStorage {
    type Error = Error;
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error> {
        Self::create()
            .and_then(|t: Self| t.inner.ok_or(Error::NoLocalStorageSystemInitialized))
            .and_then(|t| t.get_item(key).map_err_into())
            .and_then(|t| {
                t.ok_or(Self::Error::NoDataWithKey(key.to_string()))
                    .and_then(|t| serde_json::from_str::<T>(&t).map_err_into())
            })
    }

    fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::Error> {
        Self::create()
            .and_then(|t: Self| t.inner.ok_or(Error::NoLocalStorageSystemInitialized))
            .and_then(|t| {
                serde_json::to_string(value)
                    .map_err_into()
                    .and_then(|value_str| t.set_item(key, &value_str).map_err_into())
            })
    }

    fn del<T: DeserializeOwned>(&mut self, key: &str) -> Result<T, Self::Error> {
        Self::create()
            .and_then(|t: Self| t.inner.ok_or(Error::NoLocalStorageSystemInitialized))
            .merge(self.get::<T>(key), |r1, r2: T| {
                r1.remove_item(key).map_err_into().map(|_| r2)
            })
    }
}
