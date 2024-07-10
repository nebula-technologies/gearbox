use crate::time::constants::{MILLIS_PER_SEC, NANOS_PER_SEC};
use crate::time::NANOS_PER_MILLI;
use core::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Duration {
    Negative(u64, u32),
    Positive(u64, u32),
    Zero,
}

impl Duration {
    pub fn zero() -> Self {
        Self::Zero
    }
    pub fn from_secs(sec: i64) -> Self {
        if sec.is_negative() {
            Self::Negative(sec.abs() as u64, 0)
        } else {
            Self::Positive(sec.abs() as u64, 0)
        }
    }

    pub fn from_nanos(nanos: i128) -> Self {
        let new_sec = (nanos / NANOS_PER_SEC as i128) as u64;
        let new_nanos = (NANOS_PER_SEC as i128 % nanos) as u32;
        if nanos.is_negative() {
            Self::Negative(new_sec, new_nanos)
        } else {
            Self::Positive(new_sec, new_nanos)
        }
    }

    pub fn from_secs_nanos(sec: &i64, nanos: &u32) -> Self {
        if sec.is_negative() {
            Self::Negative(sec.abs() as u64, *nanos)
        } else {
            Self::Positive(*sec as u64, *nanos)
        }
    }

    pub fn as_secs(&self) -> i64 {
        match self {
            Self::Negative(sec, _) => -(sec.clone() as i64),
            Self::Positive(sec, _) => sec.clone() as i64,
            Self::Zero => 0,
        }
    }

    pub fn as_millis(&self) -> i64 {
        match self {
            Self::Negative(sec, nanos) => {
                -((sec.clone() as i64 * MILLIS_PER_SEC as i64)
                    + (nanos.clone() as i64 / NANOS_PER_MILLI as i64))
            }
            Self::Positive(sec, nanos) => {
                (sec.clone() as i64 * MILLIS_PER_SEC as i64)
                    + (nanos.clone() as i64 / NANOS_PER_MILLI as i64)
            }
            Self::Zero => 0,
        }
    }

    pub fn as_nanos(&self) -> i128 {
        match self {
            Self::Negative(sec, nanos) => {
                -(sec.clone() as i128 * NANOS_PER_SEC as i128 + nanos.clone() as i128)
            }
            Self::Positive(sec, nanos) => {
                sec.clone() as i128 * NANOS_PER_SEC as i128 + nanos.clone() as i128
            }
            Self::Zero => 0,
        }
    }

    pub fn as_sub_nanos(&self) -> u32 {
        match self {
            Self::Negative(_sec, nanos) => nanos.clone(),
            Self::Positive(_sec, nanos) => nanos.clone(),
            Self::Zero => 0,
        }
    }

    pub fn into_nanos_as_i64_unchecked(&self) -> i64 {
        match self {
            Self::Negative(sec, nanos) => {
                -(sec.clone() as i64 * NANOS_PER_SEC as i64 + nanos.clone() as i64)
            }
            Self::Positive(sec, nanos) => {
                sec.clone() as i64 * NANOS_PER_SEC as i64 + nanos.clone() as i64
            }
            Self::Zero => 0,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Self::Negative(_, _) => true,
            _ => false,
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Self::Positive(_, _) => true,
            _ => false,
        }
    }

    pub fn add_secs(&mut self, secs: u64) {
        match self {
            Self::Negative(ref sec, _) => {
                let new_sec = *sec as i64 - secs as i64;
                if new_sec.is_negative() {
                    *self = Self::Negative(new_sec.abs() as u64, 0);
                } else {
                    *self = Self::Positive(new_sec as u64, 0);
                }
            }
            Self::Positive(sec, _) => {
                *sec += secs;
            }
            Self::Zero => *self = Self::Positive(secs, 0),
        }
    }

    pub fn remove_secs(&mut self, secs: u64) {
        match self {
            Self::Negative(sec, _) => {
                *sec += secs;
            }
            Self::Positive(ref sec, _) => {
                let new_sec = *sec as i64 - secs as i64;
                if new_sec.is_negative() {
                    *self = Self::Positive(new_sec.abs() as u64, 0);
                } else {
                    *self = Self::Negative(new_sec as u64, 0);
                }
            }
            Self::Zero => *self = Self::Negative(secs, 0),
        }
    }

    pub fn add_nanos(&mut self, nanos: u128) {
        let mut add_secs = (nanos / NANOS_PER_SEC as u128) as i64;
        let add_nanos = (nanos % NANOS_PER_SEC as u128) as u32;
        match self {
            Self::Negative(ref mut sec, ref mut n) => {
                let total_nanos = *n as i64 - add_nanos as i64;
                if total_nanos.is_negative() {
                    // If subtracting nanos results in negative nanoseconds, adjust the seconds
                    add_secs += 1; // Borrow one second to adjust nanos
                    *n = (NANOS_PER_SEC as i64 + total_nanos) as u32; // Adjust nanos to be positive
                } else {
                    *n = total_nanos as u32;
                }
                let new_secs = *sec as i64 - add_secs;
                if new_secs.is_negative() {
                    // If total seconds are non-negative, flip to Positive
                    *self = Self::Positive(new_secs.abs() as u64, *n);
                } else {
                    *sec = new_secs as u64;
                }
            }
            Self::Positive(ref mut sec, ref mut n) => {
                let mut total_nanos = *n + add_nanos;
                add_secs += (total_nanos / NANOS_PER_SEC) as i64;
                total_nanos = total_nanos % NANOS_PER_SEC;
                *sec += add_secs as u64;
                *n = total_nanos;
            }
            Self::Zero => *self = Self::Positive(add_secs as u64, add_nanos as u32),
        }
    }

    pub fn remove_nanos(&mut self, nanos: u128) {
        let mut remove_secs = (nanos / NANOS_PER_SEC as u128) as i64;
        let remove_nanos = (nanos % NANOS_PER_SEC as u128) as u32;
        match self {
            Self::Negative(ref mut sec, ref mut n) => {
                let mut total_nanos = *n + remove_nanos;
                remove_secs += (total_nanos / NANOS_PER_SEC) as i64;
                total_nanos = total_nanos % NANOS_PER_SEC;
                *sec += remove_secs as u64;
                *n = total_nanos;
            }
            Self::Positive(ref mut sec, ref mut n) => {
                let total_nanos = *n as i64 - remove_nanos as i64;
                if total_nanos.is_negative() {
                    // If subtracting nanos results in negative nanoseconds, adjust the seconds
                    remove_secs += 1; // Borrow one second to adjust nanos
                    *n = (NANOS_PER_SEC as i64 + total_nanos) as u32; // Adjust nanos to be positive
                } else {
                    *n = total_nanos as u32;
                }
                let new_secs = *sec as i64 - remove_secs;
                if new_secs.is_negative() {
                    // If total seconds are non-negative, flip to Positive
                    *self = Self::Negative(new_secs.abs() as u64, *n);
                } else {
                    *sec = new_secs as u64;
                }
            }
            Self::Zero => *self = Self::Negative(remove_secs as u64, remove_nanos as u32),
        }
    }

    pub fn duration_since<D: Into<Duration>>(&self, earlier: D) -> Self {
        self - &(earlier.into())
    }

    pub(crate) fn subtract_time(&mut self, time: Self) {
        match self {
            Self::Negative(ref mut sec, ref mut n) => match time {
                Self::Negative(sec2, n2) => {
                    let new_nanos = n.clone() as i32 - n2 as i32;
                    if new_nanos.is_negative() {
                        *n = (NANOS_PER_SEC as i32 + new_nanos) as u32;
                        *sec -= 1;
                    } else {
                        *n = new_nanos as u32;
                    }
                    let new_secs = sec.clone() as i64 - sec2 as i64;
                    if new_secs.is_negative() {
                        *self = Self::Positive(new_secs.abs() as u64, *n);
                    } else {
                        *sec = new_secs as u64;
                    }
                }
                Self::Positive(sec2, n2) => {
                    let new_nanos = n.clone() as i32 + n2 as i32;
                    if new_nanos >= NANOS_PER_SEC as i32 {
                        *n = (new_nanos - NANOS_PER_SEC as i32) as u32;
                        *sec += 1;
                    } else {
                        *n = new_nanos as u32;
                    }
                    *sec = (sec.clone() as i64 + sec2 as i64) as u64;
                }
                Self::Zero => {}
            },
            Self::Positive(ref mut sec, ref mut n) => match time {
                Self::Negative(s2, n2) => {
                    let new_nanos = n.clone() as i32 + n2 as i32;
                    if new_nanos >= NANOS_PER_SEC as i32 {
                        *n = (new_nanos - NANOS_PER_SEC as i32) as u32;
                        *sec += 1;
                    } else {
                        *n = new_nanos as u32;
                    }
                    *sec = (sec.clone() as i64 + s2 as i64) as u64;
                }
                Self::Positive(s2, n2) => {
                    let new_nanos = n.clone() as i32 - n2 as i32;
                    if new_nanos.is_negative() {
                        *n = (new_nanos + NANOS_PER_SEC as i32) as u32;
                        *sec -= 1;
                    } else {
                        *n = new_nanos as u32;
                    }
                    let new_secs = sec.clone() as i64 - s2 as i64;
                    if new_secs.is_negative() {
                        *self = Self::Negative(new_secs.abs() as u64, *n);
                    } else {
                        *sec = new_secs as u64;
                    }
                }
                Duration::Zero => {}
            },
            Self::Zero => match time {
                Duration::Negative(s, n) => {
                    *self = Duration::Positive(s, n);
                }
                Duration::Positive(s, n) => {
                    *self = Duration::Negative(s, n);
                }
                Duration::Zero => {}
            },
        }
    }
}

#[cfg(feature = "std")]
impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        let sec = duration.as_secs() as i64;
        let nanos = duration.subsec_nanos();
        Duration::from_secs_nanos(&sec, &nanos)
    }
}

impl From<i64> for Duration {
    fn from(duration: i64) -> Self {
        Duration::from_secs(duration)
    }
}

impl From<(i64, u64)> for Duration {
    fn from((sec, nanos): (i64, u64)) -> Self {
        Duration::from_secs_nanos(&sec, &(nanos as u32))
    }
}

impl From<(i64, u32)> for Duration {
    fn from((sec, nanos): (i64, u32)) -> Self {
        Duration::from_secs_nanos(&sec, &nanos)
    }
}
impl From<(u64, u32)> for Duration {
    fn from((sec, nanos): (u64, u32)) -> Self {
        Duration::from_secs_nanos(&(sec as i64), &nanos)
    }
}
impl From<(u32, u32)> for Duration {
    fn from((sec, nanos): (u32, u32)) -> Self {
        Duration::from_secs_nanos(&(sec as i64), &nanos)
    }
}
impl From<(i32, i32)> for Duration {
    fn from((sec, nanos): (i32, i32)) -> Self {
        Duration::from_secs_nanos(&(sec as i64), &(nanos as u32))
    }
}
impl From<(i32, u32)> for Duration {
    fn from((sec, nanos): (i32, u32)) -> Self {
        Duration::from_secs_nanos(&(sec as i64), &nanos)
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let self_nanos = self.as_sub_nanos();
        let other_nanos = other.as_sub_nanos();
        let mut calc_nanos = self_nanos + other_nanos;

        let self_secs = self.as_secs();
        let other_secs = other.as_secs();
        let mut calc_secs = self_secs + other_secs;

        if calc_nanos > NANOS_PER_SEC {
            calc_secs += (calc_nanos / NANOS_PER_SEC) as i64;
            calc_nanos = calc_nanos % NANOS_PER_SEC;
        }
        if calc_secs.is_negative() {
            Self::Negative(calc_secs.abs() as u64, calc_nanos as u32)
        } else {
            Self::Positive(calc_secs as u64, calc_nanos as u32)
        }
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        // Assume these methods convert durations to a total of nanoseconds within the current second
        let self_nanos = self.as_sub_nanos();
        let other_nanos = other.as_sub_nanos();

        // Initial calculation might result in negative nanoseconds
        let mut calc_nanos = self_nanos as i64 - other_nanos as i64;

        // Convert durations to total seconds
        let self_secs = self.as_secs() as i64;
        let other_secs = other.as_secs() as i64;
        let mut calc_secs = self_secs - other_secs;

        // Adjust for negative nanoseconds, ensuring calc_nanos is within 0..NANOS_PER_SEC
        if calc_nanos < 0 {
            calc_secs -= 1;
            calc_nanos += NANOS_PER_SEC as i64; // Adjust nanos back to positive within a second
        }

        // Convert back to Duration, deciding on Positive, Negative, or Zero
        if calc_secs > 0 || (calc_secs == 0 && calc_nanos > 0) {
            Self::Positive(calc_secs as u64, calc_nanos as u32)
        } else if calc_secs < 0 || (calc_secs == 0 && calc_nanos < 0) {
            Self::Negative(calc_secs.abs() as u64, calc_nanos.abs() as u32)
        } else {
            Self::Zero
        }
    }
}

impl Sub for &Duration {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.clone().sub(rhs.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::time::Duration;

    #[test]
    fn test_add_sub_time() {
        let epoch = Duration::from_secs_nanos(&0, &0);
        let sec_10 = Duration::from_secs_nanos(&10, &0);

        assert_eq!(10, (epoch.clone() + sec_10.clone()).as_secs());
        assert_eq!(-10, (epoch.clone() - sec_10.clone()).as_secs());
    }
}
