use super::File;
use crate::common::TryDefault;
use crate::path;
use crate::storage::io::file::error::Error;
use crate::storage::{json::JsonExt, yaml::YamlExt, KeyStoreExt};
use serde::de::DeserializeOwned;

impl JsonExt for File {
    type Error = Error;
    fn get_json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Self::Error> {
        self.contents_string()
            .and_then(|t| serde_json::from_str::<T>(&t).map_err(Error::from))
    }

    fn set_json<T: serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        serde_json::to_string(value)
            .map_err(Error::from)
            .and_then(|t| self.write_str_to_file(&t))
    }
}

impl YamlExt for File {
    type Error = Error;
    fn get_yaml<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Self::Error> {
        self.contents_string()
            .and_then(|t| serde_yaml::from_str::<T>(&t).map_err(Error::from))
    }

    fn set_yaml<T: serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        serde_yaml::to_string(value)
            .map_err(Error::from)
            .and_then(|t| self.write_str_to_file(&t))
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
            .and_then(|t| self.write_str_to_file(&t))
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
                    .and_then(|t| self.write_str_to_file(&t))
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
#[cfg(test)]
mod tests {
    use super::{File, KeyStoreExt};
    use crate::storage::json::JsonExt;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_file_create_and_read() {
        let path = PathBuf::from(
            "/tmp/rust-test/storage/io/file/ext/tests/test_file_create_and_read.json",
        );
        let mut file = File::new(path.clone(), None);
        let content = r#"{"key": "value"}"#.to_string();

        let t = file.write_str_to_file(&content);
        let read_content = file.contents_string().unwrap();
        assert_eq!(read_content, content);

        fs::remove_file(path).unwrap();
    }
    #[test]
    fn test_file_get_json() {
        let path =
            PathBuf::from("/tmp/rust-test/storage/io/file/ext/tests/test_file_get_json.json");
        let mut file = File::new(path.clone(), None);
        let content = r#"{"key": "value"}"#;

        file.write_str_to_file(content).unwrap();
        let json: serde_json::Value = file.get_json().unwrap();
        assert_eq!(json["key"], "value");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_file_set_json() {
        let path =
            PathBuf::from("/tmp/rust-test/storage/io/file/ext/tests/test_file_set_json.json");
        let mut file = File::new(path.clone(), None);
        let data = serde_json::json!({ "key": "value" });

        file.set_json(&data).unwrap();
        let read_content = file.contents_string().unwrap();
        assert_eq!(read_content, data.to_string());

        fs::remove_file(path).unwrap();
    }

    #[test]
    #[ignore]
    fn test_file_error_handling() {
        let path = PathBuf::from("/tmp/C:  \n  (*#&$/invalid_path/test_file.json");
        let mut file = File::new(path.clone(), None);

        let result = file.write_str_to_file("test");
        println!("{:?}", result);
        assert!(result.is_err());
    }
}
