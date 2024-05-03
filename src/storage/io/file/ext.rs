use super::File;
use crate::common::TryDefault;
use crate::path;
use crate::storage::io::file::error::Error;
use crate::storage::{json::JsonExt, yaml::YamlExt, KeyStoreExt};
use serde::de::DeserializeOwned;

impl JsonExt for File {
    type Error = Error;
    fn get_json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Self::Error> {
        self.get_contents()
            .and_then(|t| serde_json::from_str::<T>(&t).map_err(Error::from))
    }

    fn set_json<T: serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        serde_json::to_string(value)
            .map_err(Error::from)
            .and_then(|t| self.write_to_file(t))
    }
}

impl YamlExt for File {
    type Error = Error;
    fn get_yaml<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Self::Error> {
        self.get_contents()
            .and_then(|t| serde_yaml::from_str::<T>(&t).map_err(Error::from))
    }

    fn set_yaml<T: serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        serde_yaml::to_string(value)
            .map_err(Error::from)
            .and_then(|t| self.write_to_file(t))
    }
}

impl KeyStoreExt for File
where
    Self: JsonExt,
{
    type Error = Error;
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error> {
        self.get_json()
            .and_then(|t: serde_json::Value| {
                t.get(key)
                    .cloned()
                    .ok_or(Error::NoFileConfigured(key.to_string()))
            })
            .and_then(|t| serde_json::from_value::<T>(t.clone()).map_err(Error::from))
    }

    fn set<T: serde::Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::Error> {
        self.get_json()
            .and_then(|mut json: serde_json::Value| {
                serde_json::to_value(value)
                    .map_err(Error::from)
                    .and_then(|t| {
                        json.as_object_mut()
                            .ok_or(Error::NoFileConfigured(key.to_string()))
                            .and_then(|map| {
                                map.insert(key.to_string(), t);
                                Ok(())
                            })
                    })
            })
            .and_then(|t| serde_json::to_string(&t).map_err(Error::from))
            .and_then(|t| self.write_to_file(t))
    }

    fn del<T: DeserializeOwned>(&mut self, key: &str) -> Result<T, Self::Error> {
        self.get_json()
            .and_then(|mut json: serde_json::Value| {
                json.as_object_mut()
                    .ok_or(Error::NoFileConfigured(key.to_string()))
                    .and_then(|map| {
                        map.remove(key)
                            .ok_or(Error::NoFileConfigured(key.to_string()))
                            .and_then(|t| serde_json::from_value::<T>(t).map_err(Error::from))
                    })
                    .map(|t: T| (t, json))
            })
            .and_then(|(t, json)| {
                serde_json::to_string(&json)
                    .map_err(Error::from)
                    .and_then(|t| self.write_to_file(t))
                    .map(|_| t)
            })
    }
}

impl TryDefault for File {
    type Error = Error;
    fn try_default() -> Result<Self, Self::Error> {
        path::config_dir().ok_or(Error::NoPath).map(Self::from_path)
    }
}
