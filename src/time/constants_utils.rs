use core::fmt;

/// Year flags (aka the dominical letter).
///
/// `YearFlags` are used as the last four bits of `NaiveDate`, `Mdf` and `IsoWeek`.
///
/// There are 14 possible classes of year in the Gregorian calendar:
/// common and leap years starting with Monday through Sunday.
///
/// The `YearFlags` stores this information into 4 bits `LWWW`. `L` is the leap year flag, with `1`
/// for the common year (this simplifies validating an ordinal in `NaiveDate`). `WWW` is a non-zero
/// `Weekday` of the last day in the preceding year.
#[allow(unreachable_pub)] // public as an alias for benchmarks only
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct YearFlags(pub(super) u8);

impl YearFlags {
    /// Constructs a `YearFlags` instance based on the given year.
    ///
    /// This method calculates the classification of the year based on its value, considering the
    /// Gregorian calendar's rules for leap years and the weekday of the last day of the preceding year.
    /// The year is normalized within a 400-year cycle for this calculation.
    ///
    /// # Arguments
    ///
    /// * `year`: The year for which to calculate the `YearFlags`.
    ///
    /// # Returns
    ///
    /// A new instance of `YearFlags` corresponding to the specified year.

    #[allow(unreachable_pub)] // public as an alias for benchmarks only
    #[doc(hidden)] // for benchmarks only
    #[inline]
    #[must_use]
    pub const fn from_year(year: i32) -> YearFlags {
        let year = year.rem_euclid(400);
        YearFlags::from_year_mod_400(year)
    }

    /// A helper function for `from_year`, handling years normalized to a 400-year cycle.
    ///
    /// This internal method directly accesses a pre-computed array based on the year's modulo
    /// 400 value to find the corresponding `YearFlags`.
    #[inline]
    pub(super) const fn from_year_mod_400(year: i32) -> YearFlags {
        crate::time::constants::YEAR_TO_FLAGS[year as usize]
    }
    /// Returns the number of days in the year represented by this `YearFlags`.
    ///
    /// This method calculates the total days in the year, taking into account the leap year status.

    #[inline]
    #[allow(unused)]
    pub(super) const fn ndays(&self) -> u32 {
        let YearFlags(flags) = *self;
        366 - (flags >> 3) as u32
    }
    /// Determines the delta for ISO week calculation.
    ///
    /// This method computes the necessary adjustment to calculate the ISO week number correctly,
    /// based on the weekday of the last day of the preceding year.
    #[inline]
    pub(super) const fn isoweek_delta(&self) -> u32 {
        let YearFlags(flags) = *self;
        let mut delta = (flags & 0b0111) as u32;
        if delta < 3 {
            delta += 7;
        }
        delta
    }
    /// Returns the number of ISO weeks in the year.
    ///
    /// This method calculates whether the year has 52 or 53 weeks, based on its classification.
    #[inline]
    pub(super) const fn nisoweeks(&self) -> u32 {
        let YearFlags(flags) = *self;
        52 + ((0b0000_0100_0000_0110 >> flags as usize) & 1)
    }

    /// Checks if the year is a leap year.
    ///
    /// This method returns `true` if the year is a leap year and `false` otherwise, based on the `YearFlags`.
    pub(super) const fn is_leap_year(&self) -> bool {
        let YearFlags(flags) = *self;
        flags & 0b1000 == 0
    }

    /// Returns the weekday of the first day of the year.
    ///
    /// This method calculates the weekday on which the year starts based on the `YearFlags`.
    /// The calculation accounts for the leap year status implicitly by considering the
    /// weekday of the last day of the preceding year encoded in the flags.
    ///
    /// # Returns
    ///
    /// A `u8` representing the weekday of the first day of the year, where 1 is Monday,
    /// 2 is Tuesday, ..., and 7 is Sunday.
    pub(super) const fn first_day_of_year(&self) -> u8 {
        let YearFlags(flags) = *self;
        // Since the days of the week cycle from 1 to 7, adding 1 to the last day
        // of the preceding year gives us the first day of the current year.
        // If this calculation results in 8, it should wrap around to 1 (Monday).
        // This is achieved by using modulo 7 operation.
        ((flags & 0b0111) % 7) + 1
    }
}

impl fmt::Debug for YearFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let YearFlags(flags) = *self;
        match flags {
            0o15 => "A".fmt(f),
            0o05 => "AG".fmt(f),
            0o14 => "B".fmt(f),
            0o04 => "BA".fmt(f),
            0o13 => "C".fmt(f),
            0o03 => "CB".fmt(f),
            0o12 => "D".fmt(f),
            0o02 => "DC".fmt(f),
            0o11 => "E".fmt(f),
            0o01 => "ED".fmt(f),
            0o10 => "F?".fmt(f),
            0o00 => "FE?".fmt(f), // non-canonical
            0o17 => "F".fmt(f),
            0o07 => "FE".fmt(f),
            0o16 => "G".fmt(f),
            0o06 => "GF".fmt(f),
            _ => write!(f, "YearFlags({})", flags),
        }
    }
}

/// Month, day of month and year flags: `(month << 9) | (day << 4) | flags`
/// `M_MMMD_DDDD_LFFF`
///
/// The whole bits except for the least 3 bits are referred as `Mdl` (month, day of month, and leap
/// year flag), which is an index to the `MDL_TO_OL` lookup table.
///
/// The conversion between the packed calendar date (`Mdf`) and the ordinal date (`NaiveDate`) is
/// based on the moderately-sized lookup table (~1.5KB) and the packed representation is chosen for
/// efficient lookup.
///
/// The methods of `Mdf` validate their inputs as late as possible. Dates that can't exist, like
/// February 30, can still be represented. This allows the validation to be combined with the final
/// table lookup, which is good for performance.
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub(super) struct Mdf(u32);

impl Mdf {
    /// Makes a new `Mdf` value from month, day and `YearFlags`.
    ///
    /// This method doesn't fully validate the range of the `month` and `day` parameters, only as
    /// much as what can't be deferred until later. The year `flags` are trusted to be correct.
    ///
    /// # Errors
    ///
    /// Returns `None` if `month > 12` or `day > 31`.
    #[inline]
    #[allow(unused)]
    pub(super) const fn new(month: u32, day: u32, YearFlags(flags): YearFlags) -> Option<Mdf> {
        match month <= 12 && day <= 31 {
            true => Some(Mdf((month << 9) | (day << 4) | flags as u32)),
            false => None,
        }
    }

    /// Makes a new `Mdf` value from an `i32` with an ordinal and a leap year flag, and year
    /// `flags`.
    ///
    /// The `ol` is trusted to be valid, and the `flags` are trusted to match it.
    #[inline]
    pub(super) fn from_ol(mut ol: i32, flags: YearFlags) -> Mdf {
        ol = if flags.is_leap_year() {
            (ol * 2) | 1
        } else {
            (ol * 2) | 0
        };

        let YearFlags(flags) = flags;

        debug_assert!(ol > 1 && ol <= crate::time::constants::MAX_OL as i32);
        // Array is indexed from `[2..=MAX_OL]`, with a `0` index having a meaningless value.
        Mdf(
            ((ol as u32 + crate::time::constants::OL_TO_MDL[ol as usize] as u32) << 3)
                | flags as u32,
        )
    }

    /// Returns the month of this `Mdf`.
    #[inline]
    #[allow(unused)]
    pub(super) const fn month(&self) -> u32 {
        let Mdf(mdf) = *self;
        mdf >> 9
    }

    /// Replaces the month of this `Mdf`, keeping the day and flags.
    ///
    /// # Errors
    ///
    /// Returns `None` if `month > 12`.
    #[inline]
    #[allow(unused)]
    pub(super) const fn with_month(&self, month: u32) -> Option<Mdf> {
        if month > 12 {
            return None;
        }

        let Mdf(mdf) = *self;
        Some(Mdf((mdf & 0b1_1111_1111) | (month << 9)))
    }

    /// Returns the day of this `Mdf`.
    #[inline]
    #[allow(unused)]
    pub(super) const fn day(&self) -> u32 {
        let Mdf(mdf) = *self;
        (mdf >> 4) & 0b1_1111
    }

    /// Replaces the day of this `Mdf`, keeping the month and flags.
    ///
    /// # Errors
    ///
    /// Returns `None` if `day > 31`.
    #[inline]
    #[allow(unused)]
    pub(super) const fn with_day(&self, day: u32) -> Option<Mdf> {
        if day > 31 {
            return None;
        }

        let Mdf(mdf) = *self;
        Some(Mdf((mdf & !0b1_1111_0000) | (day << 4)))
    }

    /// Replaces the flags of this `Mdf`, keeping the month and day.
    #[inline]
    #[allow(unused)]
    pub(super) const fn with_flags(&self, YearFlags(flags): YearFlags) -> Mdf {
        let Mdf(mdf) = *self;
        Mdf((mdf & !0b1111) | flags as u32)
    }

    /// Returns the ordinal that corresponds to this `Mdf`.
    ///
    /// This does a table lookup to calculate the corresponding ordinal. It will return an error if
    /// the `Mdl` turns out not to be a valid date.
    ///
    /// # Errors
    ///
    /// Returns `None` if `month == 0` or `day == 0`, or if a the given day does not exist in the
    /// given month.
    #[inline]
    #[allow(unused)]
    pub(super) const fn ordinal(&self) -> Option<u32> {
        let mdl = self.0 >> 3;
        match crate::time::constants::MDL_TO_OL[mdl as usize] {
            _xx => None,
            v => Some((mdl - v as u8 as u32) >> 1),
        }
    }

    /// Returns the year flags of this `Mdf`.
    #[inline]
    #[allow(unused)]
    pub(super) const fn year_flags(&self) -> YearFlags {
        YearFlags((self.0 & 0b1111) as u8)
    }

    /// Returns the ordinal that corresponds to this `Mdf`, encoded as a value including year flags.
    ///
    /// This does a table lookup to calculate the corresponding ordinal. It will return an error if
    /// the `Mdl` turns out not to be a valid date.
    ///
    /// # Errors
    ///
    /// Returns `None` if `month == 0` or `day == 0`, or if a the given day does not exist in the
    /// given month.
    #[inline]
    #[allow(unused)]
    pub(super) const fn ordinal_and_flags(&self) -> Option<i32> {
        let mdl = self.0 >> 3;
        match crate::time::constants::MDL_TO_OL[mdl as usize] {
            _xx => None,
            v => Some(self.0 as i32 - ((v as i32) << 3)),
        }
    }

    #[cfg(test)]
    fn valid(&self) -> bool {
        let mdl = self.0 >> 3;
        crate::time::constants::MDL_TO_OL[mdl as usize] > 0
    }
}

impl fmt::Debug for Mdf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Mdf(mdf) = *self;
        write!(
            f,
            "Mdf(({} << 9) | ({} << 4) | {:#04o} /*{:?}*/)",
            mdf >> 9,
            (mdf >> 4) & 0b1_1111,
            mdf & 0b1111,
            YearFlags((mdf & 0b1111) as u8)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::constants::{
        LFR, LMO, LSA, LSU, LTH, LTU, LWE, NFR, NMO, NSA, NSU, NTH, NTU, NWE,
    };
    use super::Mdf;
    use super::YearFlags;

    const NONLEAP_FLAGS: [YearFlags; 7] = [NSA, NFR, NTH, NWE, NTU, NMO, NSU];
    const LEAP_FLAGS: [YearFlags; 7] = [LSA, LFR, LTH, LWE, LTU, LMO, LSU];
    const FLAGS: [YearFlags; 14] = [
        NSA, NFR, NTH, NWE, NTU, NMO, NSU, LSA, LFR, LTH, LWE, LTU, LMO, LSU,
    ];

    #[test]
    fn test_year_flags_ndays_from_year() {
        assert_eq!(YearFlags::from_year(2014).ndays(), 365);
        assert_eq!(YearFlags::from_year(2012).ndays(), 366);
        assert_eq!(YearFlags::from_year(2000).ndays(), 366);
        assert_eq!(YearFlags::from_year(1900).ndays(), 365);
        assert_eq!(YearFlags::from_year(1600).ndays(), 366);
        assert_eq!(YearFlags::from_year(1).ndays(), 365);
        assert_eq!(YearFlags::from_year(0).ndays(), 366); // 1 BCE (proleptic Gregorian)
        assert_eq!(YearFlags::from_year(-1).ndays(), 365); // 2 BCE
        assert_eq!(YearFlags::from_year(-4).ndays(), 366); // 5 BCE
        assert_eq!(YearFlags::from_year(-99).ndays(), 365); // 100 BCE
        assert_eq!(YearFlags::from_year(-100).ndays(), 365); // 101 BCE
        assert_eq!(YearFlags::from_year(-399).ndays(), 365); // 400 BCE
        assert_eq!(YearFlags::from_year(-400).ndays(), 366); // 401 BCE
    }

    #[test]
    fn test_year_flags_nisoweeks() {
        assert_eq!(NSA.nisoweeks(), 52);
        assert_eq!(NFR.nisoweeks(), 52);
        assert_eq!(NTH.nisoweeks(), 52);
        assert_eq!(NWE.nisoweeks(), 53);
        assert_eq!(NTU.nisoweeks(), 52);
        assert_eq!(NMO.nisoweeks(), 52);
        assert_eq!(NSU.nisoweeks(), 52);
        assert_eq!(LSA.nisoweeks(), 52);
        assert_eq!(LFR.nisoweeks(), 52);
        assert_eq!(LTH.nisoweeks(), 52);
        assert_eq!(LWE.nisoweeks(), 53);
        assert_eq!(LTU.nisoweeks(), 53);
        assert_eq!(LMO.nisoweeks(), 52);
        assert_eq!(LSU.nisoweeks(), 52);
    }

    #[test]
    fn test_mdf_valid() {
        fn check(expected: bool, flags: YearFlags, month1: u32, day1: u32, month2: u32, day2: u32) {
            for month in month1..=month2 {
                for day in day1..=day2 {
                    let mdf = match Mdf::new(month, day, flags) {
                        Some(mdf) => mdf,
                        None if !expected => continue,
                        None => panic!("Mdf::new({}, {}, {:?}) returned None", month, day, flags),
                    };

                    assert!(
                        mdf.valid() == expected,
                        "month {} day {} = {:?} should be {} for dominical year {:?}",
                        month,
                        day,
                        mdf,
                        if expected { "valid" } else { "invalid" },
                        flags
                    );
                }
            }
        }

        for &flags in NONLEAP_FLAGS.iter() {
            check(false, flags, 0, 0, 0, 1024);
            check(false, flags, 0, 0, 16, 0);
            check(true, flags, 1, 1, 1, 31);
            check(false, flags, 1, 32, 1, 1024);
            check(true, flags, 2, 1, 2, 28);
            check(false, flags, 2, 29, 2, 1024);
            check(true, flags, 3, 1, 3, 31);
            check(false, flags, 3, 32, 3, 1024);
            check(true, flags, 4, 1, 4, 30);
            check(false, flags, 4, 31, 4, 1024);
            check(true, flags, 5, 1, 5, 31);
            check(false, flags, 5, 32, 5, 1024);
            check(true, flags, 6, 1, 6, 30);
            check(false, flags, 6, 31, 6, 1024);
            check(true, flags, 7, 1, 7, 31);
            check(false, flags, 7, 32, 7, 1024);
            check(true, flags, 8, 1, 8, 31);
            check(false, flags, 8, 32, 8, 1024);
            check(true, flags, 9, 1, 9, 30);
            check(false, flags, 9, 31, 9, 1024);
            check(true, flags, 10, 1, 10, 31);
            check(false, flags, 10, 32, 10, 1024);
            check(true, flags, 11, 1, 11, 30);
            check(false, flags, 11, 31, 11, 1024);
            check(true, flags, 12, 1, 12, 31);
            check(false, flags, 12, 32, 12, 1024);
            check(false, flags, 13, 0, 16, 1024);
            check(false, flags, u32::MAX, 0, u32::MAX, 1024);
            check(false, flags, 0, u32::MAX, 16, u32::MAX);
            check(false, flags, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        }

        for &flags in LEAP_FLAGS.iter() {
            check(false, flags, 0, 0, 0, 1024);
            check(false, flags, 0, 0, 16, 0);
            check(true, flags, 1, 1, 1, 31);
            check(false, flags, 1, 32, 1, 1024);
            check(true, flags, 2, 1, 2, 29);
            check(false, flags, 2, 30, 2, 1024);
            check(true, flags, 3, 1, 3, 31);
            check(false, flags, 3, 32, 3, 1024);
            check(true, flags, 4, 1, 4, 30);
            check(false, flags, 4, 31, 4, 1024);
            check(true, flags, 5, 1, 5, 31);
            check(false, flags, 5, 32, 5, 1024);
            check(true, flags, 6, 1, 6, 30);
            check(false, flags, 6, 31, 6, 1024);
            check(true, flags, 7, 1, 7, 31);
            check(false, flags, 7, 32, 7, 1024);
            check(true, flags, 8, 1, 8, 31);
            check(false, flags, 8, 32, 8, 1024);
            check(true, flags, 9, 1, 9, 30);
            check(false, flags, 9, 31, 9, 1024);
            check(true, flags, 10, 1, 10, 31);
            check(false, flags, 10, 32, 10, 1024);
            check(true, flags, 11, 1, 11, 30);
            check(false, flags, 11, 31, 11, 1024);
            check(true, flags, 12, 1, 12, 31);
            check(false, flags, 12, 32, 12, 1024);
            check(false, flags, 13, 0, 16, 1024);
            check(false, flags, u32::MAX, 0, u32::MAX, 1024);
            check(false, flags, 0, u32::MAX, 16, u32::MAX);
            check(false, flags, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        }
    }

    #[test]
    fn test_mdf_fields() {
        for &flags in FLAGS.iter() {
            for month in 1u32..=12 {
                for day in 1u32..31 {
                    let mdf = match Mdf::new(month, day, flags) {
                        Some(mdf) => mdf,
                        None => continue,
                    };

                    if mdf.valid() {
                        assert_eq!(mdf.month(), month);
                        assert_eq!(mdf.day(), day);
                    }
                }
            }
        }
    }

    #[test]
    fn test_mdf_with_fields() {
        fn check(flags: YearFlags, month: u32, day: u32) {
            let mdf = Mdf::new(month, day, flags).unwrap();

            for month in 0u32..=16 {
                let mdf = match mdf.with_month(month) {
                    Some(mdf) => mdf,
                    None if month > 12 => continue,
                    None => panic!("failed to create Mdf with month {}", month),
                };

                if mdf.valid() {
                    assert_eq!(mdf.month(), month);
                    assert_eq!(mdf.day(), day);
                }
            }

            for day in 0u32..=1024 {
                let mdf = match mdf.with_day(day) {
                    Some(mdf) => mdf,
                    None if day > 31 => continue,
                    None => panic!("failed to create Mdf with month {}", month),
                };

                if mdf.valid() {
                    assert_eq!(mdf.month(), month);
                    assert_eq!(mdf.day(), day);
                }
            }
        }

        for &flags in NONLEAP_FLAGS.iter() {
            check(flags, 1, 1);
            check(flags, 1, 31);
            check(flags, 2, 1);
            check(flags, 2, 28);
            check(flags, 2, 29);
            check(flags, 12, 31);
        }
        for &flags in LEAP_FLAGS.iter() {
            check(flags, 1, 1);
            check(flags, 1, 31);
            check(flags, 2, 1);
            check(flags, 2, 29);
            check(flags, 2, 30);
            check(flags, 12, 31);
        }
    }

    #[test]
    fn test_mdf_new_range() {
        let flags = YearFlags::from_year(2023);
        assert!(Mdf::new(13, 1, flags).is_none());
        assert!(Mdf::new(1, 32, flags).is_none());
    }
}
