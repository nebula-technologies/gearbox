use crate::common::TryDefault;
use crate::rails::ext::syn::RailsMapErrInto;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(all(not(target_arch = "wasm32"), feature = "storage-io"))]
pub mod io;
pub mod json;
#[cfg(all(target_arch = "wasm32", feature = "storage-web"))]
pub mod web;
pub mod yaml;

pub mod selective_storage;

pub trait KeyStoreExt {
    type Error;
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error>;
    fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::Error>;
    fn del<T: DeserializeOwned>(&mut self, key: &str) -> Result<T, Self::Error>;
    fn create<E: Into<Self::Error>, S: KeyStoreExt + TryDefault<Error = E>>(
    ) -> Result<S, Self::Error> {
        S::try_default().map_err_into::<Self::Error>()
    }
}
