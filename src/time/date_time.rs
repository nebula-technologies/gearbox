use crate::time::cache::{CacheKey, CacheKey::*, CacheWrapper};
use crate::time::constants::{
    COMMON_TIMESTAMP_FORMATS, EPOCH, NANOS_PER_MICRO, NANOS_PER_MILLI, NANOS_PER_SEC, SECS_PER_DAY,
    SECS_PER_HOUR, SECS_PER_LEAP_YEAR, SECS_PER_MINUTE, SECS_PER_YEAR,
};
use crate::time::duration::Duration;
use crate::time::error::Error;
use crate::time::utils::{days_in_month, is_leap_year};

use crate::template::PipelineValue;
use crate::time::constants_utils::{Mdf, YearFlags};
use crate::time::{utils, SecondsFormat, TimeNow};
use alloc::{
    format,
    string::{String, ToString},
};
use core::any::Any;
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Add, Sub};
#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a point in time with associated timezone information.
///
/// `DateTime` is a struct that combines a `Duration` since the UNIX epoch
/// to represent the specific moment, along with a `Duration` to manage timezone
/// offsets. It also utilizes a `CacheWrapper` to optimize repeated time calculations.
///
/// # Examples
///
/// ```
/// use gearbox::time::{DateTime, Duration};
///
/// let mut datetime = DateTime::now();
/// println!("Current time: {:?}", datetime);
/// ```
#[derive(Debug)]
pub struct DateTime {
    time: Duration,
    zone: Duration,
    cache: CacheWrapper,
}

impl DateTime {
    /// Clears the internal cache. This method is useful when there are significant
    /// changes to the DateTime's state that invalidate cached values.
    #[allow(unused)]
    fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Retrieves a cached value based on a specified key.
    ///
    /// If the value is not cached, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `key` - A `CacheKey` that specifies which time component to retrieve from the cache.
    fn cache(&self, key: CacheKey) -> Option<(i32, Duration)> {
        self.cache.get(key)
    }

    /// Updates the cache with a new value for a specified key, associating it with a duration.
    ///
    /// This method can be used to manually set cache values for specific time components.
    /// This is useful in scenarios where calculated time components are known and do not need to be recomputed.
    ///
    /// # Arguments
    ///
    /// * `key` - The `CacheKey` specifying which component to update.
    /// * `value` - The integer value of the component.
    /// * `duration` - The `Duration` that specifies how long this cache entry is valid.
    fn cache_update<D>(&self, key: CacheKey, value: i32, duration: D) -> (i32, Duration)
    where
        D: Into<Duration>,
    {
        self.cache.set(key, value, duration)
    }
}

/// DateTime - Creation Functions
impl DateTime {
    /// Constructs a new `DateTime` object representing the current time.
    ///
    /// This function retrieves the system time, calculates the duration since the UNIX epoch,
    /// and initializes the `time` field with this duration. The `zone` is set to zero, and the cache is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let now = DateTime::now();
    /// println!("Current DateTime: {:?}", now);
    /// ```
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        let system_time = SystemTime::now();
        let duration = system_time.duration_since(UNIX_EPOCH).unwrap();
        Self {
            time: duration.into(),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Constructs a new `DateTime` object representing the current time if std feature else zero.
    ///
    /// This function retrieves the system time, calculates the duration since the UNIX epoch,
    /// and initializes the `time` field with this duration. The `zone` is set to zero, and the cache is empty.
    /// If std feature is not available it will return 0 time, this is for allowing a unification
    /// when either std or no-std is used. In short this remove possible gating of code in other places
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let now = DateTime::now();
    /// println!("Current DateTime: {:?}", now);
    /// ```
    pub fn now_or_zero() -> Self {
        #[cfg(not(feature = "std"))]
        {
            Self::default()
        }
        #[cfg(feature = "std")]
        {
            let system_time = SystemTime::now();
            let duration = system_time.duration_since(UNIX_EPOCH).unwrap();
            Self {
                time: duration.into(),
                zone: Duration::zero(),
                cache: Default::default(),
            }
        }
    }

    /// Constructs a new `DateTime` object representing from the timesystem.
    ///
    /// This function retrieves the system time from a timesystem that implements the trait `TimeSystemNow`,
    /// calculates the duration since the UNIX epoch, and initializes the `time` field with this duration.
    /// The `zone` is set to zero, and the cache is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    /// use gearbox::time::{DateTime, TimeNow};
    ///
    /// let now = DateTime::now_with_timesystem::<SystemTime>();
    /// println!("Current DateTime: {:?}", now);
    /// ```

    pub fn now_with_timesystem<T: TimeNow>() -> Self {
        let system_time = T::time_now();
        // Please note that TimeNow requires the time to be in seconds since epoch
        let duration = system_time
            .map(|t| t.duration_since(0))
            .unwrap_or(Duration::zero());
        Self {
            time: duration.into(),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Returns a timestamp that is set to 'zero' 0 which is the same as epoch in UNIX time.
    /// This is useful for creating a DateTime object that will need time added or adjusted later
    /// # Examples
    /// ```rust
    ///
    /// use gearbox::time::DateTime;
    /// let datetime = DateTime::new();
    /// println!("DateTime: {:?}", datetime);
    ///
    /// assert_eq!(datetime.year(), 1970);
    /// ```
    pub fn new() -> Self {
        Self {
            time: Duration::zero(),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Calculates the time elapsed from this `DateTime` instance to now.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::now();
    /// let elapsed = datetime.elapsed();
    /// println!("Elapsed time: {:?}", elapsed);
    /// ```
    #[cfg(feature = "std")]
    pub fn elapsed(&self) -> Self {
        let now = Self::now();
        self.clone() - now
    }

    /// Creates a `DateTime` from seconds since the UNIX epoch.
    ///
    /// # Arguments
    ///
    /// * `sec` - The number of seconds since the UNIX epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::{DateTime, Duration};
    ///
    /// let datetime = DateTime::from_secs(1609459200); // 2021-01-01T00:00:00Z
    /// println!("DateTime: {:?}", datetime);
    /// ```
    pub fn from_secs(sec: i64) -> Self {
        Self {
            time: Duration::from_secs(sec),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Creates a `DateTime` from nanoseconds since the UNIX epoch.
    ///
    /// # Arguments
    ///
    /// * `nanos` - The number of nanoseconds since the UNIX epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::{DateTime, Duration};
    ///
    /// let datetime = DateTime::from_nanos(1609459200000000000);
    /// println!("DateTime: {:?}", datetime);
    /// ```
    pub fn from_nanos(nanos: i128) -> Self {
        Self {
            time: Duration::from_nanos(nanos),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Creates a `DateTime` from seconds and additional nanoseconds since the UNIX epoch.
    ///
    /// # Arguments
    ///
    /// * `sec` - The number of seconds since the UNIX epoch.
    /// * `nanos` - Additional nanoseconds to add to the seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::{DateTime, Duration};
    ///
    /// let datetime = DateTime::from_secs_nanos(1609459200, 500);
    /// println!("DateTime: {:?}", datetime);
    /// ```
    pub fn from_secs_nanos(sec: i64, nanos: u32) -> Self {
        Self {
            time: Duration::from_secs_nanos(&sec, &nanos),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Creates a `DateTime` from a UNIX timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The UNIX timestamp as seconds since the epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::from_timestamp(1609459200);
    /// println!("DateTime from timestamp: {:?}", datetime);
    /// ```
    pub fn from_timestamp(timestamp: i64) -> Self {
        Self {
            time: Duration::from_secs(timestamp),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    /// Creates a `DateTime` from a `SystemTime`.
    ///
    /// If `SystemTime` is before the UNIX epoch, adjusts the `DateTime` to represent the correct negative duration.
    ///
    /// # Arguments
    ///
    /// * `system_time` - A `std::time::SystemTime` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{SystemTime, UNIX_EPOCH};
    /// use gearbox::time::DateTime;
    ///
    /// let system_time = SystemTime::now();
    /// let datetime = DateTime::from_system_time(system_time);
    /// println!("DateTime from SystemTime: {:?}", datetime);
    /// ```
    #[cfg(feature = "std")]
    pub fn from_system_time(system_time: SystemTime) -> Self {
        system_time
            .duration_since(UNIX_EPOCH)
            .map(|t| Self {
                time: t.into(),
                zone: Duration::zero(),
                cache: Default::default(),
            })
            .unwrap_or_else(|e| {
                Self::from_secs_nanos(
                    e.duration().as_secs() as i64 * -1,
                    e.duration().subsec_nanos(),
                )
            })
    }
    /// Constructs a `DateTime` from a specified date.
    ///
    /// This function accounts for leap years when calculating the time elapsed from the epoch to the specified date.
    ///
    /// # Arguments
    ///
    /// * `year` - Year component of the date.
    /// * `month` - Month component of the date.
    /// * `day` - Day component of the date.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::from_date(2021, 1, 1); // Represents 2021-01-01
    /// println!("DateTime from date: {:?}", datetime);
    /// ```
    pub fn from_date(year: i32, month: u8, day: u8) -> Self {
        let mut duration = Duration::zero();
        if year < EPOCH {
            for i in year + 1..EPOCH {
                if is_leap_year(&i) {
                    duration.remove_secs(SECS_PER_LEAP_YEAR as u64);
                } else {
                    duration.remove_secs(SECS_PER_YEAR as u64);
                }
            }
            for i in month + 1..13 {
                let days = days_in_month(&year, &i);
                duration.remove_secs(days as u64 * SECS_PER_DAY as u64);
            }
            duration.remove_secs(
                days_in_month(&year, &month) as u64 - day as u64 * SECS_PER_DAY as u64,
            );
        } else {
            for i in EPOCH..year {
                if is_leap_year(&i) {
                    duration.add_secs(SECS_PER_LEAP_YEAR as u64);
                } else {
                    duration.add_secs(SECS_PER_YEAR as u64);
                }
            }
            for i in 1..month {
                let days = days_in_month(&year, &i);
                duration.add_secs(days as u64 * SECS_PER_DAY as u64);
            }
            duration.add_secs(day as u64 * SECS_PER_DAY as u64);
        }

        Self {
            time: duration,
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }
    /// Constructs a `DateTime` with a detailed date and time specification including timezone.
    ///
    /// # Arguments
    ///
    /// * `year` - Year component.
    /// * `month` - Month component.
    /// * `day` - Day component.
    /// * `hour` - Hour component.
    /// * `minute` - Minute component.
    /// * `second` - Second component.
    /// * `nanos` - Nanosecond component.
    /// * `zone` - Tuple representing the timezone offset as hours and minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::from_date_long(2021, 1, 1, 12, 0, 0, 0, (0, 0));
    /// println!("Detailed DateTime: {:?}", datetime);
    /// ```
    pub fn from_date_long(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u64,
        zone: (i8, u8),
    ) -> Self {
        let mut date_time = Self::from_date(year, month, day);
        date_time.adjust_hours(hour as i64);
        date_time.adjust_minutes(minute as i64);
        date_time.adjust_seconds(second as i64);
        date_time.add_nanos(nanos);
        date_time.adjust_zone((zone.0 as i32 * SECS_PER_HOUR as i32) + zone.1 as i32);
        date_time
    }

    /// Parses a `DateTime` from a formatted string.
    ///
    /// This method attempts to parse a string into a `DateTime` using predefined common timestamp formats.
    ///
    /// # Arguments
    ///
    /// * `string` - A string slice representing the formatted date and time.
    ///
    /// # Errors
    ///
    /// Returns `Err(Error::InvalidFormat)` if the string does not match any known formats.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::from_str("2021-01-01T12:00:00Z");
    /// assert!(datetime.is_ok());
    /// println!("Parsed DateTime: {:?}", datetime.unwrap());
    /// ```
    pub fn from_str(string: &str) -> Result<Self, Error> {
        for i in COMMON_TIMESTAMP_FORMATS.iter() {
            let date_time_res = utils::str_to_timestamp(string, i).map(|t| Self::from(t));
            if date_time_res.is_ok() {
                return date_time_res;
            }
        }
        Err(Error::InvalidFormat(string.to_string()))
    }
}

/// # DateTime - TimeZone Control
impl DateTime {
    /// Returns the current time zone offset in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::now();
    /// println!("Timezone offset in seconds: {}", datetime.timezone_offset_seconds());
    /// ```
    pub fn timezone_offset_seconds(&self) -> i64 {
        self.timezone().as_secs()
    }

    /// Returns the current time zone offset in hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::now();
    /// println!("Timezone offset in hours: {}", datetime.timezone_offset_hours());
    /// ```
    pub fn timezone_offset_hours(&self) -> i64 {
        self.timezone().as_secs() / SECS_PER_HOUR as i64
    }

    /// Generates a timezone abbreviation based on the current timezone offset.
    ///
    /// This method computes the offset in a human-readable format (`±HH:MM`).
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::now();
    /// println!("Timezone abbreviation: {}", datetime.timezone_abbreviation());
    /// ```
    pub fn timezone_abbreviation(&self) -> String {
        let zone = self.timezone();
        let sign = if zone.is_negative() { "-" } else { "+" };
        let hours = zone.as_secs().abs() / SECS_PER_HOUR as i64;
        let minutes = (zone.as_secs() % SECS_PER_HOUR as i64) / SECS_PER_MINUTE as i64;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }
    /// Converts the timezone to a string in the format `±HH:MM`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::now();
    /// println!("Timezone as string: {}", datetime.zone_to_str());
    /// ```
    pub fn zone_to_str(&self) -> String {
        let zone = self.timezone();
        let sign = if zone.is_negative() { "-" } else { "+" };
        let hours = zone.as_secs() / SECS_PER_HOUR as i64;
        let minutes = (zone.as_secs() % SECS_PER_HOUR as i64) / SECS_PER_MINUTE as i64;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }

    /// Provides a reference to the current timezone `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let datetime = DateTime::now();
    /// let timezone_duration = datetime.timezone();
    /// println!("Timezone duration: {:?}", timezone_duration);
    /// ```
    pub fn timezone(&self) -> &Duration {
        &self.zone
    }

    /// Provides a mutable reference to the current timezone `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::{DateTime, Duration};
    ///
    /// let mut datetime = DateTime::now();
    /// *datetime.timezone_mut() = Duration::from_secs(3600);  // Adjusting timezone to +1 hour
    /// ```
    pub fn timezone_mut(&mut self) -> &mut Duration {
        &mut self.zone
    }
    /// Adjusts the `DateTime` object's timezone and shifts the time accordingly.
    ///
    /// This method calculates the difference between the current timezone and the new one, adjusting the
    /// internal time to reflect the change in timezone without altering the absolute point in time.
    ///
    /// # Arguments
    ///
    /// * `zone` - The new timezone `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::time::{DateTime, Duration};
    ///
    /// let mut datetime = DateTime::now();
    /// datetime.shift_timezone(Duration::from_secs(3600)); // Shifting timezone to UTC+1
    /// println!("New DateTime: {:?}", datetime);
    /// ```
    pub fn shift_timezone(&mut self, zone: Duration) {
        let difference = self.zone.as_secs() - zone.as_secs();
        if difference.is_positive() {
            self.time.add_secs(difference as u64);
        } else if difference.is_negative() {
            self.time.remove_secs(difference.abs() as u64);
        }
        self.zone = zone;
    }
}

impl DateTime {
    pub fn adjust_days(&mut self, days: i64) {
        if days.is_negative() {
            self.time
                .remove_secs(days.abs() as u64 * SECS_PER_DAY as u64);
        } else {
            self.time.add_secs(days as u64 * SECS_PER_DAY as u64);
        }
    }
    pub fn adjust_hours(&mut self, hours: i64) {
        if hours.is_negative() {
            self.time
                .remove_secs(hours.abs() as u64 * SECS_PER_HOUR as u64);
        } else {
            self.time.add_secs(hours as u64 * SECS_PER_HOUR as u64);
        }
    }

    pub fn adjust_minutes(&mut self, minutes: i64) {
        if minutes.is_negative() {
            self.time
                .remove_secs(minutes.abs() as u64 * SECS_PER_MINUTE as u64);
        } else {
            self.time.add_secs(minutes as u64 * SECS_PER_MINUTE as u64);
        }
    }

    pub fn adjust_seconds(&mut self, seconds: i64) {
        self.time.add_secs(seconds as u64);
    }

    pub fn add_nanos(&mut self, nanos: u64) {
        self.time.add_nanos(nanos as u128);
    }

    pub fn add_millis(&mut self, millis: u64) {
        self.time
            .add_nanos(millis as u128 * NANOS_PER_MILLI as u128);
    }

    pub fn adjust_zone(&mut self, zone: i32) {
        if zone.is_negative() {
            self.timezone_mut()
                .remove_secs(zone.abs() as u64 * SECS_PER_MINUTE as u64);
        } else {
            self.timezone_mut()
                .add_secs(zone as u64 * SECS_PER_MINUTE as u64);
        }
    }
}

/// Represents a date and time, providing utility functions for detailed manipulation and retrieval of individual time components.
///
/// This struct assumes a supporting `Time` implementation that allows precise control and querying of time data.
///
/// # Examples
///
/// ```
/// use gearbox::time::DateTime;
///
/// let mut dt = DateTime::new();
/// dt.adjust_days(1);
/// dt.adjust_hours(-3);
/// dt.adjust_minutes(30);
/// dt.adjust_seconds(45);
/// ```
impl DateTime {
    /// Returns the year component of the current date.
    ///
    /// # Returns
    /// * `i32`: The current year as a four-digit number.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.year(), 1970);
    /// ```
    pub fn year(&self) -> i32 {
        self.year_with_overflow().0
    }

    /// Returns the month component of the current date.
    ///
    /// # Returns
    /// * `u8`: The month of the year, where 1 represents January and 12 represents December.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.month(), 1); // May
    /// ```
    pub fn month(&self) -> u8 {
        self.month_with_overflow().0 as u8
    }

    /// Returns the ISO week number of the current date.
    ///
    /// # Returns
    /// * `u8`: The week number according to the ISO week date system.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    /// let dt = DateTime::new();
    /// assert_eq!(dt.week(), 1);
    /// ```
    pub fn week(&self) -> u8 {
        self.week_of_year_with_overflow().1
    }

    /// Returns the ISO week date year.
    ///
    /// # Returns
    /// * `i32`: The year of the current ISO week.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.iso_week(), 1);
    /// ```
    pub fn iso_week(&self) -> i32 {
        self.week_of_year_with_overflow().1 as i32
    }

    /// Returns the day of the year.
    ///
    /// # Returns
    /// * `u32`: The day number within the current year.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.day_of_year(), 1); // 150th day of the year
    /// ```
    pub fn day_of_year(&self) -> u32 {
        self.day_of_year_with_overflow().0 as u32
    }

    /// Returns the day of the month.
    ///
    /// # Returns
    /// * `u8`: The day of the month.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.day_of_month(), 1); // EPOCH starts at 1st january
    /// ```
    pub fn day_of_month(&self) -> u8 {
        self.day_of_month_with_overflow().0 as u8
    }

    /// Returns the day of the week.
    ///
    /// # Returns
    /// * `u8`: The day of the week (1 for Monday, 7 for Sunday).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.day_of_week(), 4); // Thursday
    /// ```
    pub fn day_of_week(&self) -> u8 {
        self.day_of_week_with_overflow().0 as u8
    }
    /// Returns the number of days in the current month, considering leap years.
    ///
    /// # Returns
    /// * `u8`: The number of days in the month.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.days_in_current_month(), 31); // January has 31 days
    /// ```
    pub fn days_in_current_month(&self) -> u8 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!("Invalid month"),
        }
    }

    /// Returns the ISO day of the week.
    ///
    /// # Returns
    /// * `u32`: The ISO day of the week, where 1 is Monday and 7 is Sunday.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.iso_day_of_week(), 4); // Monday
    /// ```
    pub fn iso_day_of_week(&self) -> u32 {
        self.day_of_week_with_overflow().0 as u32
    }

    /// Returns the hour component of the time.
    ///
    /// # Returns
    /// * `u8`: The hour of the day (0-23).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.hour(), 0); // 1 PM
    /// ```
    pub fn hour(&self) -> u8 {
        self.hour_with_overflow().0 as u8
    }

    /// Returns the minute component of the time.
    ///
    /// # Returns
    /// * `u8`: The minute of the hour (0-59).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.minute(), 0);
    /// ```
    pub fn minute(&self) -> u8 {
        self.minute_with_overflow().0 as u8
    }

    /// Returns the second component of the time.
    ///
    /// # Returns
    /// * `u8`: The second of the minute (0-59).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.second(), 0);
    /// ```
    pub fn second(&self) -> u8 {
        self.second_with_overflow().0 as u8
    }

    /// Returns the millisecond component of the current second.
    ///
    /// # Returns
    /// * `u16`: The millisecond of the current second (0-999).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.millisecond(), 0);
    /// ```
    pub fn millisecond(&self) -> u16 {
        self.millisecond_with_overflow().0 as u16
    }

    /// Returns the microsecond component of the current millisecond.
    ///
    /// # Returns
    /// * `u16`: The microsecond of the current millisecond (0-999).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.microsecond(), 0);
    /// ```
    pub fn microsecond(&self) -> u16 {
        self.microsecond_with_overflow().0 as u16
    }

    /// Calculates the remaining nanoseconds within the current microsecond.
    ///
    /// # Returns
    /// * `u64`: The nanoseconds remaining within the current microsecond (0-999).
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.nanosecond(), 0);
    /// ```
    pub fn nanosecond(&self) -> u64 {
        self.second_with_overflow().1.as_sub_nanos() as u64
    }

    /// Returns the number of seconds since the Unix epoch.
    ///
    /// # Returns
    /// * `i64`: The number of seconds since January 1, 1970, 00:00:00 UTC.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.as_seconds_since_epoch(), 0);
    /// ```
    pub fn as_seconds_since_epoch(&self) -> i64 {
        self.time.as_secs()
    }

    /// Returns the number of milliseconds since the Unix epoch.
    ///
    /// # Returns
    /// * `i64`: The number of milliseconds since January 1, 1970, 00:00:00 UTC.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.as_millis_since_epoch(), 0);
    /// ```
    pub fn as_millis_since_epoch(&self) -> i64 {
        self.time.as_millis()
    }

    /// Returns the number of nanoseconds since the Unix epoch.
    ///
    /// # Returns
    /// * `i128`: The number of nanoseconds since January 1, 1970, 00:00:00 UTC.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::DateTime;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.as_nanos_since_epoch(), 0);
    /// ```
    pub fn as_nanos_since_epoch(&self) -> i128 {
        self.time.as_nanos()
    }

    /// Calculates the duration since a specified duration ago until this DateTime instance.
    ///
    /// # Arguments
    /// * `duration`: A `std::time::Duration` to measure back from now.
    ///
    /// # Returns
    /// * `Duration`: The amount of time from the duration specified until now.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::{DateTime, Duration};
    ///
    /// let dt = DateTime::new();
    /// let duration_since = dt.duration_since(Duration::from_secs(300));
    /// assert_eq!(duration_since, Duration::from_secs(-300));
    /// ```
    pub fn duration_since(self, duration: Duration) -> Duration {
        self.time - duration
    }
}

/// Provides methods to format date components into human-readable strings.
///
/// This implementation includes methods for retrieving the month and day of the week
/// in both full-name and abbreviated formats.
///
/// # Examples
///
/// ```
/// use gearbox::time::*;
///
/// let dt = DateTime::new();
/// assert_eq!(dt.month_long(), "January");
/// assert_eq!(dt.month_short(), "Jan");
/// assert_eq!(dt.day_of_week_long(), "Thursday");
/// assert_eq!(dt.day_of_week_short(), "Thu");
/// ```
impl DateTime {
    /// Returns the full name of the month.
    ///
    /// # Returns
    /// * `&str`: The full name of the month corresponding to the current date.
    ///
    /// # Examples
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.month_long(), "January");
    /// ```
    pub fn month_long(&self) -> &str {
        let month = self.month();
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Invalid Month",
        }
    }

    /// Returns the abbreviated name of the month.
    ///
    /// # Returns
    /// * `&str`: The three-letter abbreviation of the month.
    ///
    /// # Examples
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.month_short(), "Jan");
    /// ```
    pub fn month_short(&self) -> &str {
        let month = self.month();
        match month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Invalid Month",
        }
    }

    /// Returns the full name of the day of the week.
    ///
    /// # Returns
    /// * `&str`: The full name of the day of the week.
    ///
    /// # Examples
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.day_of_week_long(), "Thursday");
    /// ```
    pub fn day_of_week_long(&self) -> &str {
        let day = self.day_of_week();
        match day {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => "Invalid Day",
        }
    }

    /// Returns the abbreviated name of the day of the week.
    ///
    /// # Returns
    /// * `&str`: The three-letter abbreviation of the day of the week.
    ///
    /// # Examples
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.day_of_week_short(), "Thu");
    /// ```
    pub fn day_of_week_short(&self) -> &str {
        let day = self.day_of_week();
        match day {
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            7 => "Sun",
            _ => "Invalid Day",
        }
    }
}

/// # DateTime - Internals (private)
///
impl DateTime {
    /// This is an internal function to calculate the year of the datetime.
    /// If the year is after 1970 then we will forward the overflow, the same goes for years before.
    /// We first set if the year is earlier or later with `after_epoch` using
    /// `self.time.is_positive()`.
    /// then we set the time we are operating from by using `self.time.as_sub_nanos() as u64`.
    /// now while time > 0 we will continue to subtract SECS_PER_YEAR/SECS_PER_LEAP_YEAR, until
    /// we reach near 0, but if its before epoch (`after_epoch` is false)) we will overshoot the
    /// subtraction. This means that we will calculate our overflow from year start,
    /// we will subtract the `time` from 1 year, this will simplify further calculations as we
    /// will not need to care about negative overflows.
    pub(super) fn year_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(Year) {
            return t;
        }

        // this will help us know if we should stop or overshoot the loop
        let after_epoch = self.time.is_positive();
        // just getting the nano sub part of the time.
        let mut nanos_overflow = self.time.as_sub_nanos() as u64;
        // setting the time to the absolute value of the time in seconds
        let mut time = self.time.as_secs().abs();
        let mut year = EPOCH;

        while time > 0 {
            // figure out how many seconds we should reduce from the time;
            let time_reduc = if is_leap_year(&year) {
                SECS_PER_LEAP_YEAR
            } else {
                SECS_PER_YEAR
            } as i64;

            // calculate the next usable timestamp, so we do not need to calculate it more than once
            let tmp_time = time - time_reduc;

            // if the next time would be negative, and we are expected to be after epoch then we
            // want to break, and not update the `time` so that we can return the current year and
            // time as the overflow.
            if tmp_time < 0 && after_epoch {
                break;
            } else {
                time = tmp_time;
            }

            if after_epoch {
                year += 1;
            } else {
                year -= 1;
            }
        }

        // if we are calculating a time before epoch.
        if !after_epoch {
            // here we confirm if them is actually in the negative
            if time.is_negative() {
                // if we are before the epoch and time is negative we need to calculate the overflow.
                // Since we previously send time into the negative we now need to remove the time
                // from a full year of seconds to get the overflow from start of the year.
                if is_leap_year(&year) {
                    time = SECS_PER_LEAP_YEAR as i64 + time;
                } else {
                    time = SECS_PER_YEAR as i64 + time;
                }
            }
            // if there is some nanoseconds in overflow, we will need to remove 1 sec, and add the
            // missing nanoseconds by `NANOS_PER_SEC - nanos_overflow`.
            if nanos_overflow > 0 {
                nanos_overflow = NANOS_PER_SEC as u64 - nanos_overflow;
                time -= 1;
            }
        }

        // cache the result for future use
        self.cache_update(Year, year as i32, (time, nanos_overflow))
    }

    /// This is an internal function to calculate the month of the datetime.
    /// If the month is already cached we will return the cached value.
    pub(super) fn month_with_overflow(&self) -> (i32, Duration) {
        // check the cache if this already exists
        if let Some(t) = self.cache(Month) {
            return t;
        }

        let (year, overflow_dur) = self.year_with_overflow();
        let overflow_sec = overflow_dur.as_secs();
        // Day starts from 1 and not 0 so we add 1 to the days elapsed in the year.
        let days = (overflow_sec / (SECS_PER_DAY as i64)) + 1;
        let overflow = overflow_sec % SECS_PER_DAY as i64;
        let month = Mdf::from_ol(days as i32, YearFlags::from_year(year)).month();

        self.cache_update(Month, month as i32, (overflow, overflow_dur.as_sub_nanos()))
    }

    pub(super) fn week_of_year_with_overflow(&self) -> (i32, u8) {
        let year = self.year();
        let year_flag = YearFlags::from_year(year);
        let week_num = (self.day_of_year() + year_flag.isoweek_delta()) / 7;
        if week_num < 1 {
            // previous year
            let prevlastweek = YearFlags::from_year(self.year() - 1).nisoweeks();
            (year - 1, prevlastweek as u8)
        } else {
            if week_num > year_flag.nisoweeks() {
                // next year
                (year + 1, 1u8)
            } else {
                (year, week_num as u8)
            }
        }
    }

    pub(super) fn day_of_year_with_overflow(&self) -> (u8, Duration) {
        let (_current_month, overflow_dur) = self.month_with_overflow();

        let overflow_sec = overflow_dur.as_secs();
        // Calculate the total days elapsed in the current month based on the overflow nanoseconds
        let days_elapsed_in_month = overflow_sec / SECS_PER_DAY as i64;

        // Calculate the excess nanoseconds that don't complete a full day
        let excess = overflow_sec % SECS_PER_DAY as i64;

        // The day of the month is days_elapsed_in_month + 1 (since days are 1-indexed)
        let day_of_month = (days_elapsed_in_month + 1) as u8;

        (
            day_of_month,
            Duration::from_secs_nanos(&excess, &overflow_dur.as_sub_nanos()),
        )
    }

    /// Calculates the day of the month and overflow nanoseconds in the current day,
    /// leveraging the existing `month_with_overflow` method.
    pub(super) fn day_of_month_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(DayOfMonth) {
            return t;
        }

        let (_current_month, overflow_dur) = self.month_with_overflow();

        let overflow_sec = overflow_dur.as_secs();
        // Calculate the total days elapsed in the current month based on the overflow nanoseconds
        let days_elapsed_in_month = overflow_sec / SECS_PER_DAY as i64;

        // Calculate the excess nanoseconds that don't complete a full day
        let excess = overflow_sec % SECS_PER_DAY as i64;

        // The day of the month is days_elapsed_in_month + 1 (since days are 1-indexed)
        let day_of_month = (days_elapsed_in_month + 1) as u8;

        self.cache_update(
            DayOfMonth,
            day_of_month as i32,
            (excess, overflow_dur.as_sub_nanos()),
        )
    }

    /// Calculates the day of the week and overflow nanoseconds within the current day.
    pub(super) fn day_of_week_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(DayOfWeek) {
            return t;
        }
        let (year, overflow_dur) = self.year_with_overflow();
        let start_day = YearFlags::from_year(year).first_day_of_year();
        let overflow_sec = overflow_dur.as_secs();
        let overflow = overflow_sec % SECS_PER_DAY as i64;
        let day = ((overflow_sec / SECS_PER_DAY as i64) + start_day as i64) % 7 + 1;
        let day_of_week = if day == 0 { 7 } else { day };

        self.cache_update(
            DayOfWeek,
            day_of_week as i32,
            (overflow, overflow_dur.as_sub_nanos()),
        )
    }

    /// Calculates the current hour and overflow nanoseconds within the current hour.
    pub(super) fn hour_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(Hour) {
            return t;
        }
        let (_day, overflow_dur) = self.day_of_month_with_overflow();
        let overflow_sec = overflow_dur.as_secs().abs() as u64;
        let hour = (overflow_sec / SECS_PER_HOUR as u64) as u8;
        let hour_overflow = overflow_sec % SECS_PER_HOUR as u64;

        self.cache_update(
            Hour,
            hour as i32,
            (hour_overflow, overflow_dur.as_sub_nanos()),
        )
    }

    /// Calculates the current minute and overflow nanoseconds within the current minute.
    pub(super) fn minute_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(Minute) {
            return t;
        }
        let (_hour, overflow_dur) = self.hour_with_overflow();
        let overflow_sec = overflow_dur.as_secs().abs() as u32;
        let minute = (overflow_sec / SECS_PER_MINUTE) as u8;
        let minute_overflow = overflow_sec % SECS_PER_MINUTE;

        self.cache_update(
            Minute,
            minute as i32,
            (minute_overflow, overflow_dur.as_sub_nanos()),
        )
    }

    /// Calculates the current second and overflow nanoseconds within the current second.
    pub(super) fn second_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(Second) {
            return t;
        }
        let (_min, overflow_dur) = self.minute_with_overflow();
        let seconds = overflow_dur.as_secs().abs();

        self.cache_update(Second, seconds as i32, (0, overflow_dur.as_sub_nanos()))
    }

    /// Calculates the current millisecond and overflow nanoseconds within the current millisecond.
    pub(super) fn millisecond_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(Millisecond) {
            return t;
        }
        let (_min, overflow_dur) = self.second_with_overflow();
        let milliseconds = (overflow_dur.as_sub_nanos() / NANOS_PER_MILLI) as u32;

        self.cache_update(Millisecond, milliseconds as i32, (0, 0))
    }

    /// Calculates the current microsecond and overflow nanoseconds within the current microsecond.
    pub(super) fn microsecond_with_overflow(&self) -> (i32, Duration) {
        if let Some(t) = self.cache(Microseconds) {
            return t;
        }
        let (_min, overflow_dur) = self.second_with_overflow();
        let milliseconds = (overflow_dur.as_sub_nanos() / NANOS_PER_MICRO) as u32;

        self.cache_update(Microseconds, milliseconds as i32, (0, 0))
    }

    /// Helper function for ordinal Suffix
    pub(super) fn ordinal_suffix(&self) -> &str {
        let day = self.day_of_month();
        match day {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        }
    }

    pub(super) fn is_leap_year(&self) -> bool {
        utils::is_leap_year(&self.year())
    }
}

/// Implementation for to_* conversions
impl DateTime {
    /// Formats the date and time to the RFC 2822 standard, commonly used in email headers.
    ///
    /// # Returns
    /// * `String`: The formatted date and time as a string in RFC 2822 format.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.to_rfc2822(), "Thu, 01 Jan 1970 00:00:00 +0000");
    /// ```
    pub fn to_rfc2822(&self) -> String {
        let day = self.day_of_week_short();
        let month = self.month_short();
        let day_of_month = self.day_of_month();
        let year = self.year();
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();
        let zone_sign = if self.timezone().is_negative() {
            "-"
        } else {
            "+"
        };
        let zone = format!("{}{:02}{:02}", zone_sign, self.timezone_offset_hours(), 00);

        format!(
            "{}, {:02} {} {} {:02}:{:02}:{:02} {}",
            day, day_of_month, month, year, hour, minute, second, zone
        )
    }

    /// Formats the date and time to the RFC 3339 standard, commonly used in internet protocols.
    ///
    /// # Returns
    /// * `String`: The formatted date and time as a string in RFC 3339 format.
    ///
    /// # Example
    /// ```rust
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.to_rfc3339(), "1970-01-01T00:00:00Z");
    /// ```
    pub fn to_rfc3339(&self) -> String {
        let year = self.year();
        let month = format!("{:02}", self.month());
        let day = format!("{:02}", self.day_of_month());

        let (hour, _hour_overflow) = self.hour_with_overflow();
        let hour = format!("{:02}", hour);

        let (minute, _minute_overflow) = self.minute_with_overflow();
        let minute = format!("{:02}", minute);

        let (second, _) = self.second_with_overflow();
        let second = format!("{:02}", second);

        // Assuming the `zone` is in seconds and needs to be converted to hours and minutes.
        let zone_total_seconds = self.timezone().as_secs().abs();
        let zone_hours = zone_total_seconds / 3600;
        let zone_minutes = (zone_total_seconds % 3600) / 60;

        let zone_sign = if self.timezone().is_negative() {
            "-"
        } else {
            "+"
        };
        let zone = if self.timezone().as_secs() == 0 {
            "Z".to_string()
        } else {
            format!("{}{:02}:{:02}", zone_sign, zone_hours, zone_minutes)
        };

        format!(
            "{}-{}-{}T{}:{}:{}{}",
            year, month, day, hour, minute, second, zone
        )
    }

    /// Formats the date and time to the RFC 3339 standard, commonly used in internet protocols.
    /// Return an RFC 3339 and ISO 8601 date and time string with subseconds formatted as per SecondsFormat.
    ///
    /// If use_z is true and the timezone is UTC (offset 0), uses Z else it will use the full timezone
    ///
    /// # Returns
    /// * `String`: The formatted date and time as a string in RFC 3339 format.
    ///
    /// # Example
    /// ```rust
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.to_rfc3339(), "1970-01-01T00:00:00Z");
    /// ```
    pub fn to_rfc3339_opts(&self, secform: SecondsFormat, use_z: bool) -> String {
        let year = self.year();
        let month = format!("{:02}", self.month());
        let day = format!("{:02}", self.day_of_month());
        let hour = format!("{:02}", self.hour());
        let minute = format!("{:02}", self.minute());
        let second = format!("{:02}", self.second());

        let subsecond = match secform {
            SecondsFormat::Secs => String::new(),
            SecondsFormat::Millis => format!(".{:03}", self.millisecond()),
            SecondsFormat::Micros => format!(".{:06}", self.microsecond()),
            SecondsFormat::Nanos => format!(".{:09}", self.nanosecond()),
        };

        let zone = if use_z && self.timezone().as_secs() == 0 {
            "Z".to_string()
        } else {
            let zone_total_seconds = self.timezone().as_secs().abs();
            let zone_hours = zone_total_seconds / 3600;
            let zone_minutes = (zone_total_seconds % 3600) / 60;
            let zone_sign = if self.timezone().is_negative() {
                "-"
            } else {
                "+"
            };
            format!("{}{:02}:{:02}", zone_sign, zone_hours, zone_minutes)
        };

        format!(
            "{}-{}-{}T{}:{}:{}{}{}",
            year, month, day, hour, minute, second, subsecond, zone
        )
    }

    /// Converts the date and time to Unix timestamp.
    ///
    /// # Returns
    /// * `i64`: The number of seconds since January 1, 1970, 00:00:00 UTC.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.to_unix(), 0);
    /// ```
    pub fn to_unix(&self) -> i64 {
        self.time.as_secs()
    }

    /// Converts the date and time to Unix timestamp adjusted for the timezone.
    ///
    /// # Returns
    /// * `i64`: The number of seconds since January 1, 1970, 00:00:00 UTC adjusted by the timezone.
    ///
    /// # Example
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.to_unix_in_utc(), 0);
    /// ```
    pub fn to_unix_in_utc(&self) -> i64 {
        self.time.as_secs() + self.zone.as_secs()
    }

    /// Formats the date and time according to the provided format string.
    ///
    /// The format string can contain special format specifiers that start with `%`, which will be replaced by corresponding values from the `DateTime` instance.
    ///
    /// # Format Specifiers
    /// | Specifier | Description                                        | Example               |
    /// |-----------|----------------------------------------------------|-----------------------|
    /// | `%d`      | Day of the month, zero-padded (01-31)              | 01, 31                |
    /// | `%D`      | Abbreviated day of the week (Mon-Sun)              | Mon, Wed, Fri         |
    /// | `%j`      | Day of the month, not zero-padded (1-31)           | 1, 9, 31              |
    /// | `%l`      | Full textual representation of the day of the week | Monday, Wednesday     |
    /// | `%N`      | ISO-8601 numeric representation of the day of week | 1 (Monday) - 7 (Sunday) |
    /// | `%S`      | English ordinal suffix for the day of the month    | st, nd, rd, th        |
    /// | `%w`      | Numeric representation of the day of the week      | 0 (Sunday) - 6 (Saturday) |
    /// | `%z`      | Day of the year (001-366)                          | 001, 365              |
    /// | `%W`      | ISO-8601 week number                               | 42, 52                |
    /// | `%F`      | Full textual representation of a month             | January, December     |
    /// | `%m`      | Month as a zero-padded decimal number              | 01, 12                |
    /// | `%M`      | Abbreviated month name                             | Jan, Dec              |
    /// | `%n`      | Month as a decimal number without zero-padding     | 1, 12                 |
    /// | `%t`      | Number of days in the given month                  | 28, 31                |
    /// | `%Y`      | Full numeric year                                  | 1999, 2023            |
    /// | `%y`      | Two-digit year                                     | 99, 23                |
    /// | `%a`      | Lowercase Ante meridiem and Post meridiem (am/pm)  | am, pm                |
    /// | `%A`      | Uppercase Ante meridiem and Post meridiem (AM/PM)  | AM, PM                |
    /// | `%H`      | Hour in 24-hour format, zero-padded                | 00, 23                |
    /// | `%h`      | Hour in 12-hour format, zero-padded                | 01, 12                |
    /// | `%i`      | Minute, zero-padded                                | 00, 59                |
    /// | `%s`      | Second, zero-padded                                | 00, 59                |
    /// | `%u`      | Microsecond                                        | 000001 - 999999       |
    /// | `%v`      | Millisecond                                        | 001 - 999             |
    /// | `%O`      | GMT/UTC timezone offset in hours                   | -0400, +0300          |
    /// | `%P`      | GMT/UTC timezone offset in hours and minutes       | -04:00, +03:00        |
    /// | `%T`      | Timezone abbreviation                              | EST, UTC              |
    /// | `%Z`      | Timezone offset in seconds                         | -14400, 10800         |
    /// | `%r`      | RFC 2822 formatted date                            | Tue, 20 Jul 2021 03:00:00 +0000 |
    /// | `%c`      | ISO 8601 formatted date and time                   | 2021-07-20T03:00:00+00:00 |
    /// | `%U`      | Seconds since the Unix Epoch                       | 1626762000            |
    ///
    /// # Arguments
    /// * `format` - A string slice that specifies the desired format.
    ///
    /// # Returns
    /// * `String`: A string representing the formatted date and time.
    ///
    /// # Examples
    /// ```
    /// use gearbox::time::*;
    ///
    /// let dt = DateTime::new();
    /// assert_eq!(dt.format_to_str("%Y-%m-%d"), "1970-01-01");
    /// assert_eq!(dt.format_to_str("It's %H:%i on %l"), "It's 00:00 on Thursday");
    /// ```
    pub fn format_to_str(&self, format: &str) -> String {
        let mut result = String::new();
        let mut chars = format.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                if let Some(&next_char) = chars.peek() {
                    match next_char {
                        // Day specifiers
                        'd' => result.push_str(&format!("{:02}", self.day_of_month())), // Day of the month, 2 digits with leading zeros
                        'D' => result.push_str(self.day_of_week_short()), // A textual representation of a day, three letters
                        'j' => result.push_str(&self.day_of_month().to_string()), // Day of the month without leading zeros
                        'l' => result.push_str(self.day_of_week_long()), // A full textual representation of the day of the week
                        'N' => result.push_str(&format!("{}", self.iso_day_of_week())), // ISO-8601 numeric representation of the day of the week
                        'S' => result.push_str(self.ordinal_suffix()), // English ordinal suffix for the day of the month, 2 characters
                        'w' => result.push_str(&format!("{}", self.day_of_week() % 7)), // Numeric representation of the day of the week
                        'z' => result.push_str(&format!("{}", self.day_of_year())), // The day of the year (starting from 0)

                        // Week specifier
                        'W' => result.push_str(&format!("{:02}", self.iso_week())), // ISO-8601 week number of year, weeks starting on Monday

                        // Month specifiers
                        'F' => result.push_str(self.month_long()), // A full textual representation of a month, such as January or March
                        'm' => result.push_str(&format!("{:02}", self.month())), // Numeric representation of a month, with leading zeros
                        'M' => result.push_str(self.month_short()), // A short textual representation of a month, three letters
                        'n' => result.push_str(&self.month().to_string()), // Numeric representation of a month, without leading zeros
                        't' => result.push_str(&format!("{}", self.day_of_month())), // Number of days in the given month

                        // Year specifiers
                        'L' => result.push_str(if self.is_leap_year() { "1" } else { "0" }), // Whether it's a leap year
                        //'o' => result.push_str(&self.iso_week_year().to_string()), // ISO-8601 week-numbering year
                        'Y' => result.push_str(&self.year().to_string()), // A full numeric representation of a year, 4 digits
                        'y' => result.push_str(&format!("{:02}", self.year() % 100)), // A two digit representation of a year

                        // Time specifiers
                        'a' => result.push_str(if self.hour() < 12 { "am" } else { "pm" }), // Lowercase Ante meridiem and Post meridiem
                        'A' => result.push_str(if self.hour() < 12 { "AM" } else { "PM" }), // Uppercase Ante meridiem and Post meridiem
                        'B' => result.push_str(&self.to_swatch_internet_time()), // Swatch Internet time
                        'g' => result.push_str(&format!("{}", self.hour() % 12 + 1)), // 12-hour format of an hour without leading zeros
                        'G' => result.push_str(&self.hour().to_string()), // 24-hour format of an hour without leading zeros
                        'h' => result.push_str(&format!("{:02}", self.hour() % 12 + 1)), // 12-hour format of an hour with leading zeros
                        'H' => result.push_str(&format!("{:02}", self.hour())), // 24-hour format of an hour with leading zeros
                        'i' => result.push_str(&format!("{:02}", self.minute())), // Minutes with leading zeros
                        's' => result.push_str(&format!("{:02}", self.second())), // Seconds with leading zeros
                        'u' => result.push_str(&format!("{:06}", self.microsecond())), // Microseconds
                        'v' => result.push_str(&format!("{:03}", self.millisecond())), // Milliseconds

                        // Timezone specifiers
                        // 'e' => result.push_str(&self.timezone_name()), // Timezone identifier
                        // 'I' => result.push_str(if self.is_dst() { "1" } else { "0" }), // Whether or not the date is in daylight saving time
                        'O' => result.push_str(&format!("{:+03}00", self.timezone_offset_hours())), // Difference to Greenwich time (GMT) without colon between hours and minutes
                        'P' => result.push_str(&self.zone_to_str()), // Difference to Greenwich time (GMT) with colon between hours and minutes
                        'T' => result.push_str(&self.timezone_abbreviation()), // Timezone abbreviation
                        'Z' => result.push_str(&self.timezone_offset_seconds().to_string()), // Timezone offset in seconds

                        // Full Date/Time
                        'c' => result.push_str(&self.to_rfc3339()), // ISO 8601 date
                        'r' => result.push_str(&self.to_rfc2822()), // RFC 2822 formatted date
                        'U' => result.push_str(&self.to_unix().to_string()), // Seconds since the Unix Epoch

                        '%' => {
                            chars.next();
                        }
                        _ => result.push(next_char),
                    }
                    chars.next();
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    pub fn to_swatch_internet_time(&self) -> String {
        let time = self.time.as_secs() % SECS_PER_DAY as i64;
        let beats = (time * 1000 / SECS_PER_DAY as i64) as u16;
        format!("@{:03}", beats)
    }
}

impl PipelineValue for DateTime {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed_clone(&self) -> Box<dyn PipelineValue + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Add for DateTime {
    type Output = Self;

    fn add(self, mut rhs: Self) -> Self::Output {
        rhs.shift_timezone(self.timezone().clone());
        Self {
            time: self.time + rhs.time,
            zone: self.zone,
            cache: Default::default(),
        }
    }
}
impl Add for &DateTime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let _self = self.clone();
        let mut _rhs = rhs.clone();

        self + rhs
    }
}

impl Sub for DateTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut _self = self.clone();
        let mut rhs_clone = rhs.clone();

        let zone = _self.timezone();

        rhs_clone.shift_timezone(zone.clone());

        _self.time.subtract_time(rhs_clone.time);
        _self
    }
}

impl Sub for &mut DateTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut _self = self.clone();
        let mut rhs_clone = rhs.clone();

        let zone = _self.timezone();

        rhs_clone.shift_timezone(zone.clone());

        _self.time.subtract_time(rhs_clone.time);
        *self = _self;
        self
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

impl Clone for DateTime {
    fn clone(&self) -> Self {
        let cache = self.cache.clone();
        Self {
            time: self.time.clone(),
            zone: self.zone.clone(),
            cache,
        }
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.zone == other.zone
    }
}

impl Eq for DateTime {}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.time.partial_cmp(&other.time) {
            Some(Ordering::Equal) => self.zone.partial_cmp(&other.zone),
            other => other,
        }
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap() // Assuming time and zone are always comparable
    }
}

impl Hash for DateTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.zone.hash(state);
    }
}

impl From<(i32, u8, u8, u8, u8, u8, u64, (i8, u8))> for DateTime {
    fn from(
        (year, month, day, hour, minute, second, nanosecond, (zone_hour, zone_minute)): (
            i32,
            u8,
            u8,
            u8,
            u8,
            u8,
            u64,
            (i8, u8),
        ),
    ) -> Self {
        Self::from_date_long(
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
            (zone_hour, zone_minute),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::time::constants::COMMON_TIMESTAMP_FORMATS;
    use crate::time::{utils, DateTime};
    use alloc::{
        string::{String, ToString},
        vec,
        vec::Vec,
    };

    fn get_test_data() -> Vec<(String, DateTime)> {
        vec![
            (
                "2031-03-05 12:03:45".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 0, (0, 0))),
            ),
            (
                "2031-03-05T12:03:45".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 0, (0, 0))),
            ),
            (
                "2031-03-05 12:03:45.123".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (0, 0))),
            ),
            (
                "2031-03-05T12:03:45.123".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (0, 0))),
            ),
            (
                "2031-03-05 12:03:45.123+01:00".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (1, 0))),
            ),
            (
                "2031-03-05T12:03:45.123+01:00".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (1, 0))),
            ),
            (
                "2031-03-05 12:03:45.123-01:00".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123, (-1, 0))),
            ),
            (
                "2031-03-05T12:03:45.123-01:00".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123, (-1, 0))),
            ),
            (
                "2031-03-05 12:03:45.123Z".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123, (0, 0))),
            ),
            (
                "2031-03-05T12:03:45.123456Z".to_string(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123456, (0, 0))),
            ),
        ]
    }

    #[test]
    fn test_from_str_auto_date_0() {
        if let Some((input, expected)) = get_test_data().get(0) {
            let result = DateTime::from_str(input).unwrap();
            assert_eq!(result, *expected);
        }
    }
    #[test]
    fn test_from_str_auto_date_1() {
        if let Some((input, expected)) = get_test_data().get(1) {
            let result = DateTime::from_str(input).unwrap();
            assert_eq!(result, *expected);
        }
    }
    #[test]
    fn test_from_str_auto_date_2() {
        if let Some((input, expected)) = get_test_data().get(2) {
            for i in COMMON_TIMESTAMP_FORMATS.iter() {
                if utils::str_to_timestamp(input, i)
                    .map(|t| {
                        println!("Success: {:?}", t);
                    })
                    .is_ok()
                {
                    break;
                }
            }

            let result = DateTime::from_str(input)
                .map_err(|e| println!("Error: {:?}", e))
                .unwrap();
            assert_eq!(result, *expected);
        }
    }
    #[test]
    fn test_from_str_auto_date_3() {
        if let Some((input, expected)) = get_test_data().get(3) {
            let result = DateTime::from_str(input).unwrap();
            assert_eq!(result, *expected);
        }
    }
    #[test]
    fn test_from_str_auto_date_4() {
        if let Some((input, expected)) = get_test_data().get(4) {
            let result = DateTime::from_str(input).unwrap();
            assert_eq!(result, *expected);
        }
    }
    #[test]
    fn test_from_str_auto_date_5() {
        if let Some((input, expected)) = get_test_data().get(5) {
            let result = DateTime::from_str(input).unwrap();
            assert_eq!(result, *expected);
        }
    }
}
