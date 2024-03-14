use crate::common::TryDefault;
use crate::rails::ext::map_into::RailsMapErrIntoBox;
#[cfg(target_arch = "x86_64")]
use crate::storage::io::file::error::Error as FileError;
use crate::storage::KeyStoreExt;
use serde_json::Error as JsonError;
use std::fmt::{Debug, Display, Formatter};

pub trait SelectiveStorage
where
    Self: Sized,
{
    type Error;
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error>;
    fn set<T: serde::Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::Error>;
    fn del<T: serde::Serialize>(&mut self, key: &str) -> Result<(), Self::Error>;
    fn create() -> Result<Self, Self::Error>;
}

impl<S, E> SelectiveStorage for S
where
    E: std::error::Error + 'static,
    S: KeyStoreExt<Error = E> + TryDefault<Error = E>,
{
    type Error = Error;
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error> {
        S::create().and_then(|t: S| t.get(key)).map_err_box_into()
    }

    fn set<T: serde::Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::Error> {
        S::create()
            .and_then(|mut t: S| t.set(key, value))
            .map_err_box_into()
    }

    fn del<T: serde::Serialize>(&mut self, key: &str) -> Result<(), Self::Error> {
        S::create()
            .and_then(|mut t: S| t.del(key))
            .map_err_box_into()
    }
    fn create() -> Result<S, Self::Error> {
        S::try_default().map_err_box_into()
    }
}

pub enum Error {
    NoFileConfigured(String),
    SerializationError(JsonError),
    FailedToGetMutableInstance,
    UnderlyingLayerError(Box<dyn std::error::Error>),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoFileConfigured(s) => write!(f, "No file configured: {}", s),
            Error::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Error::FailedToGetMutableInstance => write!(f, "Failed to get mutable instance"),
            Error::UnderlyingLayerError(e) => write!(f, "Underlying layer error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::NoFileConfigured(e.to_string())
    }
}

#[cfg(target_arch = "x86_64")]
impl From<Error> for FileError {
    fn from(e: Error) -> Self {
        FileError::ExtensionError(Box::new(e))
    }
}

impl<E> From<Box<E>> for Error
where
    E: std::error::Error + 'static,
{
    fn from(e: Box<E>) -> Self {
        Error::UnderlyingLayerError(e)
    }
}
