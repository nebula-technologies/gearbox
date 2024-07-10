use super::error::Error;
use super::LocalStorage;
use crate::common::TryDefault;
use crate::rails::ext::blocking::Merge;
use crate::rails::ext::blocking::RailsMapErrInto;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_storage_get_set() {
        let mut storage = LocalStorage::new();
        let key = "test_key";
        let value = "test_value".to_string();

        storage.set(key, &value).unwrap();
        let retrieved: String = storage.get(key).unwrap();
        assert_eq!(retrieved, value);
    }

    #[test]
    fn test_local_storage_error_handling() {
        let mut storage = LocalStorage::new();
        let key = "test_key";

        let result: Result<String, Error> = storage.get(key);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "No data with key: test_key"
        );
    }

    #[test]
    fn test_local_storage_delete() {
        let mut storage = LocalStorage::new();
        let key = "test_key";
        let value = "test_value".to_string();

        storage.set(key, &value).unwrap();
        let deleted: String = storage.del(key).unwrap();
        assert_eq!(deleted, value);

        let result: Result<String, Error> = storage.get(key);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "No data with key: test_key"
        );
    }

    #[test]
    fn test_local_storage_serialization_error() {
        let mut storage = LocalStorage::new();
        let key = "test_key";
        let value = vec![1, 2, 3];

        let result = storage.set(key, &value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Failed to serialize data: "
        );
    }
}
