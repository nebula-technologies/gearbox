use crate::error::tracer::{Error, ErrorTracer, ErrorTracerExtInfo};

pub trait RailsMapErrTracer<T, E> {
    fn map_err_tracer(self, error: ErrorTracerExtInfo) -> Result<T, ErrorTracer>
    where
        Self: Sized,
        E: Error;
}

impl<T, E: 'static> RailsMapErrTracer<T, E> for Result<T, E> {
    fn map_err_tracer(self, err: ErrorTracerExtInfo) -> Result<T, ErrorTracer>
    where
        Self: Sized,
        E: Error,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(err.with_error(e)),
        }
    }
}
