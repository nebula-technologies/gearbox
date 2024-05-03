// Mods
pub mod error_macro;
pub mod extended_info;
// Local uses
use core::any::TypeId;
use core::fmt::Display;
use core::fmt::{Debug, Formatter};
use std::any::Any;
#[cfg(feature = "std")]
use std::time::SystemTimeError;

// Exported Uses
pub use extended_info::ErrorTracerExtInfo;

pub trait Error: Debug {}

pub trait AnyBoxError: Any {
    fn as_any(&self) -> &dyn Any;
}

impl AnyBoxError for Box<dyn Error> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(feature = "std")]
impl Error for SystemTimeError {}

pub struct ErrorTracer {
    error: Box<dyn Error>,
    type_id: TypeId,
    info: ErrorTracerExtInfo,
    cause: Option<Box<ErrorTracer>>,
}

impl ErrorTracer {
    pub fn kind(&self) -> &dyn Error {
        self.error.as_ref()
    }

    pub fn kind_mut(&self) -> &dyn Error {
        self.error.as_ref()
    }

    pub fn kind_by_type<T: 'static + Error>(&self) -> Option<&Box<T>> {
        self.error.as_any().downcast_ref::<Box<T>>()
    }
}

impl<T> From<T> for ErrorTracer
where
    T: 'static + Error,
{
    fn from(err: T) -> Self {
        let type_id = err.type_id();
        Self {
            error: Box::new(err),
            type_id,
            info: ErrorTracerExtInfo::default(),
            cause: None,
        }
    }
}

impl<T> From<(T, ErrorTracerExtInfo)> for ErrorTracer
where
    T: 'static + Error,
{
    fn from((err, info): (T, ErrorTracerExtInfo)) -> Self {
        let type_id = err.type_id();
        ErrorTracer {
            error: Box::new(err),
            type_id,
            info,
            cause: None,
        }
    }
}

impl Display for ErrorTracer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let file = self.info.file().cloned().unwrap_or("<Unknown>".to_string());
        let line = self
            .info
            .line()
            .map(|t| t.to_string())
            .unwrap_or("-1".to_string());
        let error = format!("{:?}", self.error);
        let code = self.info.code().unwrap_or(&0);
        let module = self.info.subsystem().cloned().unwrap_or("<>".to_string());
        write!(f, "{}::{}:{} {}:Error: {}", file, module, line, code, error)
    }
}

impl Debug for ErrorTracer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let file = self.info.file().cloned().unwrap_or("<Unknown>".to_string());
        let line = self
            .info
            .line()
            .map(|t| t.to_string())
            .unwrap_or("-1".to_string());
        let error = format!("{:?}", self.error);
        let code = self.info.code().unwrap_or(&0);
        let module = self.info.subsystem().cloned().unwrap_or("<>".to_string());
        write!(f, "{}::{}:{} {}:Error: {}", file, module, line, code, error)
    }
}
