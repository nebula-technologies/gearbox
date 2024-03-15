use crate::time::constants::NANOS_PER_SEC;

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

    pub fn from_secs_nanos(sec: i64, nanos: u32) -> Self {
        if sec.is_negative() {
            Self::Negative(sec.abs() as u64, nanos)
        } else {
            Self::Positive(sec as u64, nanos)
        }
    }

    pub fn as_secs(&self) -> i64 {
        match self {
            Self::Negative(sec, _) => -(sec.clone() as i64),
            Self::Positive(sec, _) => sec.clone() as i64,
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
}

impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        let sec = duration.as_secs() as i64;
        let nanos = duration.subsec_nanos();
        Duration::from_secs_nanos(sec, nanos)
    }
}

impl From<i64> for Duration {
    fn from(duration: i64) -> Self {
        Duration::from_secs(duration)
    }
}

impl From<(i64, u64)> for Duration {
    fn from((sec, nanos): (i64, u64)) -> Self {
        Duration::from_secs_nanos(sec, nanos as u32)
    }
}

impl From<(i64, u32)> for Duration {
    fn from((sec, nanos): (i64, u32)) -> Self {
        Duration::from_secs_nanos(sec, nanos)
    }
}
impl From<(u64, u32)> for Duration {
    fn from((sec, nanos): (u64, u32)) -> Self {
        Duration::from_secs_nanos(sec as i64, nanos)
    }
}
impl From<(u32, u32)> for Duration {
    fn from((sec, nanos): (u32, u32)) -> Self {
        Duration::from_secs_nanos(sec as i64, nanos)
    }
}
impl From<(i32, i32)> for Duration {
    fn from((sec, nanos): (i32, i32)) -> Self {
        Duration::from_secs_nanos(sec as i64, nanos as u32)
    }
}
impl From<(i32, u32)> for Duration {
    fn from((sec, nanos): (i32, u32)) -> Self {
        Duration::from_secs_nanos(sec as i64, nanos)
    }
}
