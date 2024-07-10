use crate::error::tracer::{DynTracerError, ErrorDebug, ErrorTracerExtInfo, TracerError};

pub trait RailsMapErrTracer<T, E> {
    fn map_tracer_err(self, error: ErrorTracerExtInfo) -> Result<T, TracerError<E>>
    where
        Self: Sized,
        E: ErrorDebug;
    fn map_dyn_tracer_err(self, error: ErrorTracerExtInfo) -> Result<T, DynTracerError>
    where
        Self: Sized,
        E: ErrorDebug;
}

impl<T, E: 'static> RailsMapErrTracer<T, E> for Result<T, E> {
    fn map_tracer_err(self, err: ErrorTracerExtInfo) -> Result<T, TracerError<E>>
    where
        Self: Sized,
        E: ErrorDebug,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(err.with_error(e)),
        }
    }
    fn map_dyn_tracer_err(self, err: ErrorTracerExtInfo) -> Result<T, DynTracerError>
    where
        Self: Sized,
        E: ErrorDebug,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(err.with_dyn_error(e)),
        }
    }
}
