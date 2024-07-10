pub mod error;
pub mod ext;

use crate::common::TryDefault;
use crate::rails::ext::blocking::RailsMapErrInto;
use crate::rails::tracing::common::RailsLog;
pub use error::Error;
use web_sys::{window, Storage};

pub struct LocalStorage {
    inner: Option<Storage>,
}

impl LocalStorage {
    pub fn new() -> Self {
        Self {
            inner: window()
                .ok_or(Error::FailedToGetLocalStorage(
                    "window was not found for the web platform".to_string(),
                ))
                .and_then(|t| t.local_storage().map_err_into())
                .and_then(|t| {
                    t.ok_or(Error::FailedToGetLocalStorage(
                        "local_storage was not found for the web platform".to_string(),
                    ))
                })
                .log(crate::error!(Err))
                .ok(),
        }
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl TryDefault for LocalStorage {
    type Error = Error;
    fn try_default() -> Result<Self, Self::Error> {
        Ok(Self::new())
    }
}
