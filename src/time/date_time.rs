use crate::time::cache::{CacheKey, CacheKey::*, CacheWrapper};
use crate::time::constants::{
    COMMON_TIMESTAMP_FORMATS, EPOCH, NANOS_PER_DAY, NANOS_PER_MICRO, NANOS_PER_MILLI,
    NANOS_PER_MINUTE, NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_LEAP_YEAR,
    SECS_PER_MINUTE, SECS_PER_MONTH, SECS_PER_YEAR,
};
use crate::time::duration::Duration;
use crate::time::error::Error;
use crate::time::utils::{days_in_month, is_leap_year};

use crate::time::constants_utils::{Mdf, YearFlags};
use crate::time::utils;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct DateTime {
    time: Duration,
    zone: Duration,
    cache: CacheWrapper,
}

impl DateTime {
    fn clear_cache(&self) {
        self.cache.clear();
    }

    fn cache(&self, key: CacheKey) -> Option<(i32, Duration)> {
        self.cache.get(key)
    }

    fn cache_update<D>(&self, key: CacheKey, value: i32, duration: D) -> (i32, Duration)
    where
        D: Into<Duration>,
    {
        self.cache.set(key, value, duration)
    }
}

/// DateTime - Creation Functions
impl DateTime {
    pub fn now() -> Self {
        let system_time = SystemTime::now();
        let duration = system_time.duration_since(UNIX_EPOCH).unwrap();
        Self {
            time: duration.into(),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    pub fn from_secs(sec: i64) -> Self {
        Self {
            time: Duration::from_secs(sec),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    pub fn from_nanos(nanos: i128) -> Self {
        Self {
            time: Duration::from_nanos(nanos),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    pub fn from_secs_nanos(sec: i64, nanos: u32) -> Self {
        Self {
            time: Duration::from_secs_nanos(sec, nanos),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

    pub fn from_timestamp(timestamp: i64) -> Self {
        Self {
            time: Duration::from_secs(timestamp),
            zone: Duration::zero(),
            cache: Default::default(),
        }
    }

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

    pub fn from_str(string: &str) -> Result<Self, Error> {
        for i in COMMON_TIMESTAMP_FORMATS.iter() {
            let date_time_res = utils::str_to_timestamp(string, i).map(|t| {
                println!("{:?}", t);
                Self::from(t)
            });
            if date_time_res.is_ok() {
                return date_time_res;
            }
        }
        Err(Error::InvalidFormat(string.to_string()))
    }
}
/// # DateTime - TimeZone Control
impl DateTime {
    pub fn timezone_offset_seconds(&self) -> i64 {
        self.timezone().as_secs()
    }

    pub fn timezone_offset_hours(&self) -> i64 {
        self.timezone().as_secs() / SECS_PER_HOUR as i64
    }

    fn timezone_abbreviation(&self) -> String {
        let zone = self.timezone();
        let sign = if zone.is_negative() { "-" } else { "+" };
        let hours = zone.as_secs().abs() / SECS_PER_HOUR as i64;
        let minutes = (zone.as_secs() % SECS_PER_HOUR as i64) / SECS_PER_MINUTE as i64;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }

    pub fn zone_to_str(&self) -> String {
        let zone = self.timezone();
        let sign = if zone.is_negative() { "-" } else { "+" };
        let hours = zone.as_secs() / SECS_PER_HOUR as i64;
        let minutes = (zone.as_secs() % SECS_PER_HOUR as i64) / SECS_PER_MINUTE as i64;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }

    pub fn timezone(&self) -> &Duration {
        &self.zone
    }

    pub fn timezone_mut(&mut self) -> &mut Duration {
        &mut self.zone
    }
}

impl DateTime {
    pub fn adujust_days(&mut self, days: i64) {
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

/// # DateTime - common functionalities (public)
impl DateTime {
    pub fn year(&self) -> i32 {
        self.year_with_overflow().0
    }

    pub fn month(&self) -> u8 {
        self.month_with_overflow().0 as u8
    }

    pub fn week(&self) -> u8 {
        self.week_of_year_with_overflow().1
    }

    pub fn iso_week(&self) -> i32 {
        self.week_of_year_with_overflow().1 as i32
    }

    pub fn day_of_year(&self) -> u32 {
        self.day_of_year_with_overflow().0 as u32
    }

    pub fn day_of_month(&self) -> u8 {
        self.day_of_month_with_overflow().0 as u8
    }

    pub fn day_of_week(&self) -> u8 {
        self.day_of_week_with_overflow().0 as u8
    }

    pub fn days_in_month(&self) -> u8 {
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

    pub fn iso_day_of_week(&self) -> u32 {
        self.day_of_week_with_overflow().0 as u32
    }

    pub fn hour(&self) -> u8 {
        self.hour_with_overflow().0 as u8
    }

    pub fn minute(&self) -> u8 {
        self.minute_with_overflow().0 as u8
    }

    pub fn second(&self) -> u8 {
        self.second_with_overflow().0 as u8
    }

    pub fn millisecond(&self) -> u16 {
        self.millisecond_with_overflow().0 as u16
    }

    pub fn microsecond(&self) -> u16 {
        self.microsecond_with_overflow().0 as u16
    }

    /// Calculates the remaining nanoseconds within the current microsecond.
    pub fn nanosecond(&self) -> u64 {
        self.second_with_overflow().1.as_sub_nanos() as u64
    }
}

/// DateTime - Short and long formating
impl DateTime {
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
                    time = (SECS_PER_LEAP_YEAR as i64 + time);
                } else {
                    time = (SECS_PER_YEAR as i64 + time);
                }
            }
            // if there is some nanoseconds in overflow, we will need to remove 1 sec, and add the
            // missing nanoseconds by `NANOS_PER_SEC - nanos_overflow`.
            if nanos_overflow > 0 {
                nanos_overflow = (NANOS_PER_SEC as u64 - nanos_overflow);
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
        let days = overflow_sec / (SECS_PER_DAY as i64);
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
            Duration::from_secs_nanos(excess, overflow_dur.as_sub_nanos()),
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
        let day = ((overflow_sec / SECS_PER_DAY as i64) + start_day as i64) % 7;
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
    fn to_rfc2822(&self) -> String {
        let day = self.day_of_week_short();
        let month = self.month_short();
        let day_of_month = self.day_of_month();
        let year = self.year();
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();
        let zone = self.timezone_offset_hours();
        format!(
            "{}, {} {} {} {:02}:{:02}:{:02} {}",
            day, day_of_month, month, year, hour, minute, second, zone
        )
    }

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

    pub fn to_unix(&self) -> i64 {
        self.time.as_secs()
    }

    pub fn to_unix_with_zone(&self) -> i64 {
        self.time.as_secs() + self.zone.as_secs()
    }
    // Other implementations...

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
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    fn to_swatch_internet_time(&self) -> String {
        let time = self.time.as_secs() % SECS_PER_DAY as i64;
        let beats = (time * 1000 / SECS_PER_DAY as i64) as u16;
        format!("@{:03}", beats)
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
        println!(
            "{:?}",
            (
                year,
                month,
                day,
                hour,
                minute,
                second,
                nanosecond,
                zone_hour,
                zone_minute
            )
        );

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
    use crate::time::error::Error;
    use crate::time::{utils, DateTime};

    fn get_test_data() -> Vec<(String, DateTime)> {
        vec![
            (
                "2031-03-05 12:03:45".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 0, (0, 0))),
            ),
            (
                "2031-03-05T12:03:45".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 0, (0, 0))),
            ),
            (
                "2031-03-05 12:03:45.123".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (0, 0))),
            ),
            (
                "2031-03-05T12:03:45.123".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (0, 0))),
            ),
            (
                "2031-03-05 12:03:45.123+01:00".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (1, 0))),
            ),
            (
                "2031-03-05T12:03:45.123+01:00".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123000000, (1, 0))),
            ),
            (
                "2031-03-05 12:03:45.123-01:00".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123, (-1, 0))),
            ),
            (
                "2031-03-05T12:03:45.123-01:00".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123, (-1, 0))),
            ),
            (
                "2031-03-05 12:03:45.123Z".to_owned(),
                DateTime::from((2031, 3, 5, 12, 3, 45, 123, (0, 0))),
            ),
            (
                "2031-03-05T12:03:45.123456Z".to_owned(),
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
