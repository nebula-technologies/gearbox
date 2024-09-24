use crate::common::TryDefault;
use crate::externs::serde;
use crate::rails::ext::syn::RailsMapErrIntoBox;
#[cfg(all(target_arch = "x86_64", feature = "std", feature = "storage-io"))]
use crate::storage::io::file::error::Error as FileError;
use crate::storage::KeyStoreExt;
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::fmt::{Debug, Display, Formatter};
#[cfg(feature = "with_json")]
use serde_json::Error as JsonError;

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
    E: crate::error::tracer::ErrorDebug + 'static,
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
    #[cfg(all(feature = "with_serde", feature = "with_json"))]
    SerializationError(JsonError),
    FailedToGetMutableInstance,
    UnderlyingLayerError(Box<dyn crate::error::tracer::ErrorDebug>),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::NoFileConfigured(s) => write!(f, "No file configured: {}", s),
            #[cfg(all(feature = "with_serde", feature = "with_json"))]
            Error::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Error::FailedToGetMutableInstance => write!(f, "Failed to get mutable instance"),
            Error::UnderlyingLayerError(e) => write!(f, "Underlying layer error: {:?}", e),
        }
    }
}

#[cfg(all(feature = "with_serde", feature = "with_json"))]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::NoFileConfigured(e.to_string())
    }
}

#[cfg(all(target_arch = "x86_64", feature = "std", feature = "storage-io"))]
impl From<Error> for FileError {
    fn from(e: Error) -> Self {
        FileError::ExtensionError(Box::new(e))
    }
}

impl<E> From<Box<E>> for Error
where
    E: crate::error::tracer::ErrorDebug + 'static,
{
    fn from(e: Box<E>) -> Self {
        Error::UnderlyingLayerError(e)
    }
}
