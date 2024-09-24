use crate::time::constants::{
    NANOS_PER_MILLI, NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MINUTE,
};
use crate::time::constants_utils::YearFlags;
use crate::time::error::Error;
use alloc::string::{String, ToString};
use core::fmt::Display;

pub(super) const fn is_leap_year(year: &i32) -> bool {
    let year = year.rem_euclid(400);
    let YearFlags(flags) = crate::time::constants::YEAR_TO_FLAGS[year as usize];
    flags & 0b1000 == 0
}

pub(crate) fn days_in_month(year: &i32, month: &u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31, // Invalid month
    }
}
#[allow(unused)]
pub(crate) fn seconds_in_year(year: &i32) -> u64 {
    // Calculate the number of seconds in a non-leap year
    let seconds_in_non_leap_year = 365 * 24 * 60 * 60;

    // Calculate the number of leap years between 1970 and the given year (exclusive)
    let leap_years = (1970..*year).filter(|&y| is_leap_year(&y)).count() as u64;

    // Total number of seconds in all complete years before the given year
    let total_seconds_before_year = leap_years * 366 * 24 * 60 * 60
        + (*year as u64 - 1970 - leap_years) as u64 * seconds_in_non_leap_year;

    total_seconds_before_year
}
#[allow(unused)]
pub(crate) fn seconds_in_month(year: &i32, month: &u8) -> Option<u64> {
    let seconds_in_year = seconds_in_year(year);
    let seconds_in_month = (1..*month)
        .filter_map(|m| Some(days_in_month(year, &m) as u64 * 24 * 60 * 60))
        .sum::<u64>();
    Some(seconds_in_year + seconds_in_month)
}

// pub(crate) fn parse_date_time_format(format: &str) -> Vec<String> {
//     let mut parts = Vec::new();
//     let mut buffer = String::new();
//     let mut control_char = false;
//
//     for c in format.chars() {
//         match c {
//             'Y' | 'm' | 'M' | 'd' | 'h' | 'i' | 's' | 'n' | 'f' | 'z' => {
//                 if control_char {
//                     buffer.push(c);
//                     parts.push(buffer.clone());
//                     buffer.clear();
//                     control_char = false;
//                 } else {
//                     buffer.push(c);
//                     control_char = true;
//                 }
//             }
//         }
//     }
//
//     if !buffer.is_empty() {
//         parts.push(buffer.clone());
//     }
//
//     parts
// }
#[allow(unused)]
pub(crate) fn str_to_zone(zone: &str) -> Result<(i8, u8), Error> {
    let mut sign = 0;
    let mut hour = "".to_string();
    let mut minute = "".to_string();
    let mut wzone = String::new();
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    #[allow(unused)]
    enum Cursor {
        Sign,
        Hour,
        Minute,
    }
    let mut cursor = Cursor::Sign;
    let mut chars = zone.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                sign = 1;
                cursor = Cursor::Hour;
            }
            '-' => {
                sign = -1;
                cursor = Cursor::Hour;
            }
            ':' => {
                if cursor == Cursor::Hour {
                    cursor = Cursor::Minute;
                } else {
                    wzone.push(c);
                }
            }
            '0'..='9' => match cursor {
                Cursor::Sign => sign = 1,
                Cursor::Hour => {
                    hour.push(c);
                }
                Cursor::Minute => {
                    minute.push(c);
                }
            },
            _ => wzone.push(c),
        }
    }
    let hour = hour.parse::<u8>().unwrap_or(0);
    let minute = minute.parse::<u8>().unwrap_or(0);

    if !wzone.is_empty() {
        match wzone.as_str() {
            "UTC" => {
                // Handle UTC timezone
                return Ok((0, 0));
            }
            "GMT" => {
                // Handle GMT timezone
                return Ok((0, 0));
            }
            "EST" => {
                // Handle Eastern Standard Time timezone
                return Ok((-5, 0));
            }
            "PST" => {
                // Handle Pacific Standard Time timezone
                return Ok((-8, 0));
            }
            "CET" => {
                // Handle Central European Time timezone
                return Ok((1, 0));
            }
            "AEST" => {
                // Handle Australian Eastern Standard Time timezone
                return Ok((10, 0));
            }
            // Add more cases for other timezones as needed
            _ => return Err(Error::InvalidTimezone),
        }
    }

    Ok((hour as i8 * sign, minute))
}

mod safe_calc {
    use crate::rails::ext::syn::Merge;
    use crate::time::error::Error;
    use alloc::format;

    pub(super) fn calc_u8(mut v: u8, c: char) -> Result<u8, Error> {
        v.checked_mul(10)
            .ok_or(Error::CalculationOverflow(format!(
                "Error: Failed to multiply milliseconds({}) x 10",
                v
            )))
            .merge(
                c.to_digit(10).ok_or(Error::StringParser(format!(
                    "Failed to convert: {} to number",
                    c
                ))),
                |t1, t2| {
                    t1.checked_add(t2 as u8)
                        .ok_or(Error::CalculationOverflow(format!(
                            "Error: Failed to add milliseconds({}) + {}",
                            v, c
                        )))
                },
            )
            .map(|t| {
                v = t;
                v
            })
            .map_err(|e| {
                Error::CalculationOverflow(format!(
                    "Error:{} - {:?} - appending milliseconds: {}",
                    e, v, c
                ))
            })
    }
    pub(super) fn calc_i32(mut v: i32, c: char) -> Result<i32, Error> {
        v.checked_mul(10)
            .ok_or(Error::CalculationOverflow(format!(
                "Error: Failed to multiply milliseconds({}) x 10",
                v
            )))
            .merge(
                c.to_digit(10).ok_or(Error::StringParser(format!(
                    "Failed to convert: {} to number",
                    c
                ))),
                |t1, t2| {
                    t1.checked_add(t2 as i32)
                        .ok_or(Error::CalculationOverflow(format!(
                            "Error: Failed to add milliseconds({}) + {}",
                            v, c
                        )))
                },
            )
            .map(|t| {
                v = t;
                v
            })
            .map_err(|e| {
                Error::CalculationOverflow(format!(
                    "Error:{} - {:?} - appending milliseconds: {}",
                    e, v, c
                ))
            })
    }
    pub(super) fn calc_u64(mut v: u64, c: char) -> Result<u64, Error> {
        v.checked_mul(10)
            .ok_or(Error::CalculationOverflow(format!(
                "Error: Failed to multiply milliseconds({}) x 10",
                v
            )))
            .merge(
                c.to_digit(10).ok_or(Error::StringParser(format!(
                    "Failed to convert: {} to number",
                    c
                ))),
                |t1, t2| {
                    t1.checked_add(t2 as u64)
                        .ok_or(Error::CalculationOverflow(format!(
                            "Error: Failed to add milliseconds({}) + {}",
                            v, c
                        )))
                },
            )
            .map(|t| {
                v = t;
                v
            })
            .map_err(|e| {
                Error::CalculationOverflow(format!(
                    "Error:{} - {:?} - appending milliseconds: {}",
                    e, v, c
                ))
            })
    }
}

#[derive(Debug)]
struct TimestampChunks {
    year: i32,
    month: u8,
    month_str: String,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    milliseconds: u64,
    nanosecond: u64,
    zone: ZoneChunk,
}

impl TimestampChunks {
    fn append_year(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_i32(self.year, c).map(|t| {
            self.year = t;
        })
    }
    fn append_month(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u8(self.month, c).map(|t| {
            self.month = t;
        })
    }
    fn append_month_str(&mut self, c: char) {
        self.month_str.push(c);
    }
    fn append_day(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u8(self.day, c).map(|t| {
            self.day = t;
        })
    }
    fn append_hour(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u8(self.hour, c).map(|t| {
            self.hour = t;
        })
    }
    fn append_min(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u8(self.minute, c).map(|t| {
            self.minute = t;
        })
    }
    fn append_sec(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u8(self.second, c).map(|t| {
            self.second = t;
        })
    }
    fn append_millis(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u64(self.milliseconds, c).map(|t| {
            self.milliseconds = t;
        })
    }
    fn append_nanos(&mut self, c: char) -> Result<(), Error> {
        safe_calc::calc_u64(self.nanosecond, c).map(|t| {
            self.nanosecond = t;
        })
    }

    fn zone_mut(&mut self) -> &mut ZoneChunk {
        &mut self.zone
    }

    fn year(&self) -> i32 {
        if self.year < 0 {
            0
        } else {
            self.year
        }
    }

    fn month(&self) -> u8 {
        if self.month == 0 && !self.month_str.is_empty() {
            let cleaned_month = {
                if self.month_str.len() > 3 {
                    self.month_str.to_lowercase().as_str()[0..3].to_string()
                } else {
                    self.month_str.to_lowercase().to_string()
                }
            };
            match cleaned_month.as_str() {
                "ja" | "jan" => 1,
                "fe" | "feb" => 2,
                "mar" => 3,
                "ap" | "apr" => 4,
                "may" => 5,
                "jun" => 6,
                "jul" => 7,
                "au" | "aug" => 8,
                "se" | "sep" => 9,
                "oc" | "oct" => 10,
                "no" | "nov" => 11,
                "de" | "dec" => 12,
                _ => 0,
            }
        } else {
            self.month
        }
    }
    fn day(&self) -> u8 {
        if self.day == 0 {
            0
        } else {
            self.day
        }
    }
    fn hour(&self) -> u8 {
        if self.hour == 0 {
            0
        } else {
            self.hour
        }
    }
    fn minute(&self) -> u8 {
        if self.minute == 0 {
            0
        } else {
            self.minute
        }
    }
    fn second(&self) -> u8 {
        if self.second == 0 {
            0
        } else {
            self.second
        }
    }
    fn milliseconds(&self) -> u64 {
        if self.milliseconds == 0 {
            0
        } else {
            self.milliseconds
        }
    }
    fn nanosecond(&self) -> u64 {
        if self.nanosecond == 0 {
            0
        } else {
            self.nanosecond
        }
    }
    fn zone(&self) -> (i8, u8) {
        self.zone.time()
    }
}
impl Default for TimestampChunks {
    fn default() -> Self {
        TimestampChunks {
            year: 0,
            month: 0,
            month_str: String::new(),
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            milliseconds: 0,
            nanosecond: 0,
            zone: ZoneChunk::default(),
        }
    }
}

#[derive(Debug)]
struct ZoneChunk {
    sign: i8,
    hour: u8,
    minute: u8,
    zone_str: String,
    cursor: ZoneCursor,
}

impl ZoneChunk {
    fn append(&mut self, c: char) -> Result<(), Error> {
        match self.cursor {
            ZoneCursor::Hour => safe_calc::calc_u8(self.hour, c).map(|t| {
                self.hour = t;
            }),
            ZoneCursor::Minute => safe_calc::calc_u8(self.minute, c).map(|t| {
                self.minute = t;
            }),
        }
    }
    fn append_str(&mut self, c: char) {
        self.zone_str.push(c);
    }
    fn next(&mut self) {
        self.cursor = match self.cursor {
            ZoneCursor::Hour => ZoneCursor::Minute,
            ZoneCursor::Minute => ZoneCursor::Hour,
        }
    }

    fn sign(&mut self, sign: i8) {
        self.sign = sign;
    }

    fn time(&self) -> (i8, u8) {
        (self.sign * self.hour as i8, self.minute)
    }
}

impl Default for ZoneChunk {
    fn default() -> Self {
        ZoneChunk {
            sign: 1,
            hour: 0,
            minute: 0,
            zone_str: String::new(),
            cursor: ZoneCursor::Hour,
        }
    }
}

#[derive(Debug)]
enum ZoneCursor {
    Hour,
    Minute,
}

impl Default for ZoneCursor {
    fn default() -> Self {
        ZoneCursor::Hour
    }
}

pub(crate) fn str_to_timestamp(
    timestamp: &str,
    pattern: &str,
) -> Result<(i32, u8, u8, u8, u8, u8, u64, (i8, u8)), Error> {
    if timestamp.is_empty() {
        return Err(Error::InvalidTimestamp);
    } else if timestamp.is_empty() {
        return Err(Error::InvalidPattern);
    }

    let mut chunks = TimestampChunks::default();
    let mut timestamp_iter = timestamp.chars().peekable();
    let mut pattern_iter = pattern.chars();
    let mut previous_pattern_char = ' ';

    while let Some(mut pattern) = pattern_iter.next() {
        let time_char = if let Some(time_char) = timestamp_iter.next() {
            time_char
        } else {
            break;
        };
        if previous_pattern_char == pattern {
            match time_char {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '+' | ':' => {}
                _ => {
                    if let Some(t) = pattern_iter.next() {
                        pattern = t;
                    } else {
                        return Err(Error::InvalidPattern);
                    }
                }
            }
        }
        match pattern {
            'y' => match time_char {
                '0'..='9' => chunks.append_year(time_char)?,
                _ => {}
            },
            'M' => {
                chunks.append_month_str(time_char);
                while let Some(t) = timestamp_iter.peek() {
                    match t {
                        'a'..='z' | 'A'..='Z' => chunks.append_month_str(*t),
                        '0'..='9' => return Err(Error::InvalidTimestamp),
                        _ => break,
                    }
                    timestamp_iter.next();
                }
            }
            'm' => match time_char {
                '0'..='9' => chunks.append_month(time_char)?,
                _ => {}
            },
            'd' => match time_char {
                '0'..='9' => chunks.append_day(time_char)?,
                _ => {}
            },
            'h' => match time_char {
                '0'..='9' => chunks.append_hour(time_char)?,
                _ => {}
            },
            'i' => match time_char {
                '0'..='9' => chunks.append_min(time_char)?,
                _ => {}
            },
            's' => match time_char {
                '0'..='9' => chunks.append_sec(time_char)?,
                _ => {}
            },
            'n' => {
                chunks.append_nanos(time_char)?;
                while let Some(t) = timestamp_iter.peek() {
                    match *t {
                        '0'..='9' => chunks.append_nanos(*t)?,
                        _ => break,
                    }
                    timestamp_iter.next();
                }
            }
            'f' => {
                chunks.append_millis(time_char)?;
                let mut chars_processed = 1;
                while let Some(t) = timestamp_iter.peek() {
                    match *t {
                        '0'..='9' => chunks.append_millis(*t)?,
                        _ => break,
                    }
                    chars_processed += 1;
                    if chars_processed > 4 {
                        return Err(Error::InvalidTimestamp);
                    }
                    timestamp_iter.next();
                }
            }
            'z' => {
                chunks.append_month_str(time_char);
                match time_char {
                    'a'..='z' | 'A'..='Z' | '-' | '+' | ':' | '0'..='9' => {}
                    _ => return Err(Error::InvalidZoneState),
                }
                let mut tick = 1;
                while let Some(t) = timestamp_iter.peek() {
                    if tick == 3 && *t != ':' {
                        chunks.zone_mut().next();
                    }
                    match *t {
                        'a'..='z' | 'A'..='Z' => chunks.zone_mut().append_str(*t),
                        '-' => chunks.zone_mut().sign(-1),
                        '+' => chunks.zone_mut().sign(1),
                        ':' => chunks.zone_mut().next(),
                        '0'..='9' => chunks.zone_mut().append(*t)?,
                        _ => break,
                    }
                    tick += 1;

                    timestamp_iter.next();
                }
            }
            _ => {}
        }
        previous_pattern_char = pattern;
    }

    let res = (
        chunks.year(),
        chunks.month(),
        chunks.day(),
        chunks.hour(),
        chunks.minute(),
        chunks.second(),
        (chunks.milliseconds() * NANOS_PER_MILLI as u64) + chunks.nanosecond(),
        chunks.zone(),
    );
    if res == (0, 0, 0, 0, 0, 0, 0, (0, 0)) {
        Err(Error::InvalidTimestamp)
    } else {
        Ok(res)
    }
}

/// TODO: Confirm usage and possibly remove
#[allow(unused)]
pub(crate) fn nanos_to_timestamp(nanos: i64) -> (i32, u8, u8, u8, u8, u8, u64) {
    let secs = nanos / NANOS_PER_SEC as i64;
    let nano_remainder = (nanos.abs() % NANOS_PER_SEC as i64) as u64;

    // Calculate seconds since UNIX epoch
    let mut remaining_secs = secs.abs();

    // Determine the sign of the timestamp
    let sign = if nanos < 0 { -1i32 } else { 1i32 };

    // Calculate year
    let mut year = 1970i32;
    let mut days_in_year = if is_leap_year(&year) { 366 } else { 365 };
    while remaining_secs >= SECS_PER_DAY as i64 * days_in_year {
        remaining_secs -= SECS_PER_DAY as i64 * days_in_year;
        year += sign;
        days_in_year = if is_leap_year(&year) { 366 } else { 365 };
    }

    // Calculate month and day
    let mut month = 1;
    let mut days_in_the_month = days_in_month(&year, &month);
    while remaining_secs >= (SECS_PER_DAY as i64 * days_in_the_month as i64) {
        remaining_secs -= SECS_PER_DAY as i64 * days_in_the_month as i64;
        month += 1;
        days_in_the_month = days_in_month(&year, &month);
    }
    let day = (remaining_secs as i64 / SECS_PER_DAY as i64 + 1) as u8;
    remaining_secs %= SECS_PER_DAY as i64;

    // Calculate hour, minute, and second
    let hour = (remaining_secs / SECS_PER_HOUR as i64) as u8;
    remaining_secs %= SECS_PER_HOUR as i64;
    let minute = (remaining_secs / SECS_PER_MINUTE as i64) as u8;
    let second = (remaining_secs % SECS_PER_MINUTE as i64) as u8;

    (year, month, day, hour, minute, second, nano_remainder)
}
//
// trait ResultCompare
// where
//     Self: Sized,
// {
//     type Error: crate::error::tracer::ErrorDebug;
//     fn if_gt(self, other: &Self) -> Result<Self, Self::Error>;
//     fn if_lt(self, other: &Self) -> Result<Self, Self::Error>;
// }
//
// impl ResultCompare for u64 {
//     type Error = CompareError;
//     fn if_gt(self, other: &u64) -> Result<u64, Self::Error> {
//         if &self > other {
//             Ok(self)
//         } else {
//             Err(Self::Error::IsNotGreaterThan)
//         }
//     }
//
//     fn if_lt(self, other: &u64) -> Result<u64, Self::Error> {
//         if &self < other {
//             Ok(self)
//         } else {
//             Err(Self::Error::IsNotGreaterThan)
//         }
//     }
// }
// impl ResultCompare for i64 {
//     type Error = CompareError;
//     fn if_gt(self, other: &i64) -> Result<i64, Self::Error> {
//         if &self > other {
//             Ok(self)
//         } else {
//             Err(Self::Error::IsNotGreaterThan)
//         }
//     }
//
//     fn if_lt(self, other: &i64) -> Result<i64, Self::Error> {
//         if &self < other {
//             Ok(self)
//         } else {
//             Err(Self::Error::IsNotGreaterThan)
//         }
//     }
// }
// impl ResultCompare for u32 {
//     type Error = CompareError;
//     fn if_gt(self, other: &u32) -> Result<u32, Self::Error> {
//         if &self > other {
//             Ok(self)
//         } else {
//             Err(Self::Error::IsNotGreaterThan)
//         }
//     }
//
//     fn if_lt(self, other: &u32) -> Result<u32, Self::Error> {
//         if &self < other {
//             Ok(self)
//         } else {
//             Err(Self::Error::IsNotGreaterThan)
//         }
//     }
// }

#[derive(Debug, Clone)]

pub enum CompareError {
    IsNotGreaterThan,
}

impl Display for CompareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "The value is not greater than the other value")
    }
}

#[cfg(test)]
mod test {
    // Import the necessary modules
    use super::*;

    #[test]
    fn test_str_to_timestamp_basic() {
        let result = str_to_timestamp("2024-03-05 12:30:45.123", "yyyy-mm-dd hh:ii:ss.f");

        println!("{:?}", result);
        assert_eq!(result.unwrap(), (2024, 3, 5, 12, 30, 45, 123000000, (0, 0)));
    }

    #[test]
    fn test_str_to_timestamp_another_basic() {
        let result =
            str_to_timestamp("2024-03-05T12:30:45.123Z", "yyyy-mm-ddThh:ii:ss.fz").unwrap();
        assert_eq!(result, (2024, 3, 5, 12, 30, 45, 123000000, (0, 0)));
    }

    #[test]
    fn test_str_to_timestamp_different_formats() {
        let result = str_to_timestamp("05-Mar-2024 12:30:45", "dd-M-yyyy hh:ii:ss").unwrap();
        assert_eq!(result, (2024, 3, 5, 12, 30, 45, 0, (0, 0)));
    }

    #[test]
    fn test_str_to_timestamp_non_zero_padded_month_day() {
        let result = str_to_timestamp("2024-3-05 12:30:45", "yyyy-mm-dd hh:ii:ss").unwrap();
        assert_eq!(result, (2024, 3, 5, 12, 30, 45, 0, (0, 0)));
    }

    #[test]
    fn test_str_to_timestamp_with_timezone() {
        let result =
            str_to_timestamp("2024-03-05 12:30:45.0123+0300", "yyyy-dd-mm hh:ii:ss.fz").unwrap();
        assert_eq!(result, (2024, 5, 3, 12, 30, 45, 123000000, (3, 0)));
    }

    #[test]
    fn test_str_to_timestamp_invalid_timestamp_format() {
        let result = str_to_timestamp("2024-03-05 12:30:45.012300000", "yyyy-dd-mm hh:ii:ss.f");
        assert!(result.is_err());
    }

    #[test]
    fn test_str_to_timestamp_empty_timestamp() {
        let result = str_to_timestamp("", "yyyy-dd-mm hh:ii:ss.f");
        assert!(result.is_err());
    }

    #[test]
    fn test_str_to_timestamp_empty_pattern() {
        let result = str_to_timestamp("2024-03-05 12:30:45.123000000", "");
        assert!(result.is_err());
    }
}
