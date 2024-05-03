mod cache;
mod constants;
mod constants_utils;
mod date_time;
mod duration;
mod error;
mod utils;

pub use constants::{
    EPOCH, NANOS_PER_DAY, NANOS_PER_HOUR, NANOS_PER_LEAP_MONTH, NANOS_PER_LEAP_YEAR,
    NANOS_PER_MICRO, NANOS_PER_MILLI, NANOS_PER_MINUTE, NANOS_PER_MONTH, NANOS_PER_SEC,
    NANOS_PER_YEAR, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_LEAP_YEAR, SECS_PER_MINUTE,
    SECS_PER_MONTH, SECS_PER_YEAR,
};

use crate::rails::ext::{RailsMapErrInto, RailsMapErrTracer};
use crate::Error;
pub use date_time::DateTime;
pub use duration::Duration;
pub use error::Error;

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

/// This is a trait for implementing a timesystem on top of anohter timesystem,
/// The main purpose is to allow alternate timesystems from the normal std time-system as this might
/// not be available on embedded systems.
///
/// the function `time_now` will return a i32 of a time since epoch, 1970-01-01 00:00:00 +0000
pub trait TimeNow {
    type Error;
    fn time_now() -> Result<Duration, Self::Error>;
}

#[cfg(feature = "std")]
impl TimeNow for SystemTime {
    type Error = crate::error::tracer::ErrorTracer;
    fn time_now() -> Result<Duration, Self::Error> {
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|t| Duration::from_secs(t.as_secs() as i64))
            .map_err_tracer(Error!())
    }
}
