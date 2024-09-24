use crate::error::tracer::{DynTracerError, ErrorDebug, TracerError};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
};
use core::any::Any;
use core::fmt::Display;
#[cfg(feature = "dep_serde")]
use serde_derive::{Deserialize, Serialize};
use spin::rwlock::RwLock;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "dep_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ErrorTracerExtInfo {
    line: Option<u32>,
    file: Option<String>,
    subsystem: Option<String>,
    code: Option<u16>,
}

impl ErrorTracerExtInfo {
    pub const fn new(
        line: Option<u32>,
        file: Option<String>,
        subsystem: Option<String>,
        code: Option<u16>,
    ) -> Self {
        Self {
            line,
            file,
            subsystem,
            code,
        }
    }

    pub fn with_file(mut self, file: &str) -> Self {
        self.file = Option::from(file.to_string());
        self
    }

    pub fn with_line(mut self, line: u32) -> Self {
        self.line = Option::from(line);
        self
    }

    pub fn with_subsystem(mut self, subsystem: &str) -> Self {
        self.subsystem = Option::from(subsystem.to_owned());
        self
    }

    pub fn with_code(mut self, code: u16) -> Self {
        self.code = Option::from(code);
        self
    }

    pub fn file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    pub fn line(&self) -> Option<&u32> {
        self.line.as_ref()
    }

    pub fn subsystem(&self) -> Option<&String> {
        self.subsystem.as_ref()
    }

    pub fn code(&self) -> Option<&u16> {
        self.code.as_ref()
    }

    pub fn with_error<T: 'static + ErrorDebug>(self, error: T) -> TracerError<T> {
        TracerError::new(Box::new(error), self, None)
    }

    pub fn with_dyn_error<T: 'static + ErrorDebug>(self, error: T) -> DynTracerError {
        DynTracerError::new(Box::new(error), self, None)
    }
}

impl Default for ErrorTracerExtInfo {
    fn default() -> Self {
        Self {
            line: None,
            file: None,
            subsystem: None,
            code: None,
        }
    }
}
