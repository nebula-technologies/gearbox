use crate::error::tracer::{Error, ErrorTracer};
use core::fmt::{Debug, Display};
use std::any::Any;

pub struct ErrorTracerExtInfo {
    line: Option<u32>,
    file: Option<String>,
    subsystem: Option<String>,
    code: Option<u16>,
}

impl ErrorTracerExtInfo {
    pub fn new(
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
        self.file = Option::from(file.to_owned());
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

    pub fn with_error<T: 'static + Error>(self, error: T) -> ErrorTracer {
        let type_id = error.type_id();
        ErrorTracer {
            error: Box::new(error),
            type_id,
            info: self,
            cause: None,
        }
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
