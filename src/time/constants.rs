//! Internal helper types for working with dates.

use crate::time::constants_utils::YearFlags;

pub const COMMON_TIMESTAMP_FORMATS: [&str; 16] = [
    // ISO 8601 Date and Time Formats
    "yyyy-mm-ddThh:ii:ssz",
    "yyyy-M-ddThh:ii:ssz",
    "yyyy-mm-ddThh:ii:ss.fz",
    "yyyy-M-ddThh:ii:ss.fz",
    // Common Log File Formats
    "dd/mm/yyyy:hh:ii:ss z",
    // ISO 8601 Basic Date and Time Formats
    "yyyymmddThhiissz",
    "yyyymmddThhiissfz",
    "yyyymmddhhiissz",
    "yyyymmddhhiissfz",
    "yyyymmddhhiiss",
    // HTTP Date Formats
    "dd mm yyyy hh:ii:ss GMT",
    "dd-mm-yyyy hh:ii:ss GMT",
    // Database Timestamp Formats
    "yyyy-mm-dd hh:ii:ss.f",
    "yyyy-mm-dd hh:ii:ss",
    // Human-Readable Formats
    "dd mm yyyy hh:ii:ss",
    "mm dd yyyy hh:ii:ss",
    // Additional formats
    // Add more formats as needed
];
/// Starting date
pub const EPOCH: i32 = 1970;
/// Defaults for NANOS
pub const NANOS_PER_YEAR: u64 = 31_536_000_000_000_000;
pub const NANOS_PER_LEAP_YEAR: u64 = 31_622_400_000_000_000;
pub const NANOS_PER_DAY: u64 = 86_400_000_000_000;
pub const NANOS_PER_MONTH: u64 = 2_592_000_000_000_000;
pub const NANOS_PER_LEAP_MONTH: u64 = 2_678_400_000_000_000;
pub const NANOS_PER_HOUR: u64 = 3_600_000_000_000;
pub const NANOS_PER_MINUTE: u64 = 60_000_000_000;
pub const NANOS_PER_SEC: u32 = 1_000_000_000;
pub const NANOS_PER_MILLI: u32 = 1_000_000;
pub const NANOS_PER_MICRO: u32 = 1_000;

/// For milliseconds conversion
pub const MILLIS_PER_SEC: u32 = 1_000;
/// Defaults for SECONDS
pub const SECS_PER_YEAR: u32 = 31536000;
pub const SECS_PER_LEAP_YEAR: u32 = 31622400;
pub const SECS_PER_MONTH: u32 = 2_592_000;
pub const SECS_PER_DAY: u32 = 86_400;
pub const SECS_PER_HOUR: u32 = 3_600;
pub const SECS_PER_MINUTE: u32 = 60;

// Weekday of the last day in the preceding year.
// Allows for quick day of week calculation from the 1-based ordinal.
pub(super) const YEAR_STARTS_AFTER_MONDAY: u8 = 7; // non-zero to allow use with `NonZero*`.
pub(super) const YEAR_STARTS_AFTER_TUESDAY: u8 = 1;
pub(super) const YEAR_STARTS_AFTER_WEDNESDAY: u8 = 2;
pub(super) const YEAR_STARTS_AFTER_THURSDAY: u8 = 3;
pub(super) const YEAR_STARTS_AFTER_FRIDAY: u8 = 4;
pub(super) const YEAR_STARTS_AFTER_SATURDAY: u8 = 5;
pub(super) const YEAR_STARTS_AFTER_SUNDAY: u8 = 6;

pub(super) const COMMON_YEAR: u8 = 1 << 3;
pub(super) const LEAP_YEAR: u8 = 0 << 3;

pub(super) const NSU: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_SUNDAY);
pub(super) const NSA: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_SATURDAY);
pub(super) const NFR: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_FRIDAY);
pub(super) const NTH: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_THURSDAY);
pub(super) const NWE: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_WEDNESDAY);
pub(super) const NTU: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_TUESDAY);
pub(super) const NMO: YearFlags = YearFlags(COMMON_YEAR | YEAR_STARTS_AFTER_MONDAY);

pub(super) const LSU: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_SUNDAY);
pub(super) const LSA: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_SATURDAY);
pub(super) const LFR: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_FRIDAY);
pub(super) const LTH: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_THURSDAY);
pub(super) const LWE: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_WEDNESDAY);
pub(super) const LTU: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_TUESDAY);
pub(super) const LMO: YearFlags = YearFlags(LEAP_YEAR | YEAR_STARTS_AFTER_MONDAY);

pub(super) const YEAR_TO_FLAGS: &[YearFlags; 400] = &[
    LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU,
    NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE,
    NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR,
    NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO,
    LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE, NTH,
    NFR, LSA, NMO, NTU, NWE, NTH, NFR, NSA, NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA,
    NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO,
    NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH,
    LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU,
    NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE,
    NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, NTU, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH,
    NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU,
    LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE,
    NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA,
    NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO,
    NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, NSU, NMO, NTU, NWE,
    LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA,
    NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU,
    NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU, LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH,
    NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE, NTH, LFR, NSU, NMO, NTU, LWE, NFR, NSA, NSU,
    LMO, NWE, NTH, NFR, LSA, NMO, NTU, NWE, LTH, NSA, NSU, NMO, LTU, NTH, NFR, NSA, LSU, NTU, NWE,
    NTH,
];
#[allow(unused)]
const ORDINAL_MASK: i32 = 0b1_1111_1111_0000;
#[allow(unused)]
const LEAP_YEAR_MASK: i32 = 0b1000;
// OL: ordinal and leap year flag.
// With only these parts of the date an ordinal 366 in a common year would be encoded as
// `((366 << 1) | 1) << 3`, and in a leap year as `((366 << 1) | 0) << 3`, which is less.
// This allows for efficiently checking the ordinal exists depending on whether this is a leap year.
#[allow(unused)]
const OL_MASK: i32 = ORDINAL_MASK | LEAP_YEAR_MASK;
// OL: (ordinal << 1) | leap year flag
pub(super) const MAX_OL: u32 = 366 << 1; // `(366 << 1) | 1` would be day 366 in a non-leap year
pub(super) const MAX_MDL: u32 = (12 << 6) | (31 << 1) | 1;

// The next table are adjustment values to convert a date encoded as month-day-leapyear to
// ordinal-leapyear. OL = MDL - adjustment.
// Dates that do not exist are encoded as `XX`.
pub(super) const XX: i8 = 0;
pub(super) const MDL_TO_OL: &[i8; MAX_MDL as usize + 1] = &[
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0
    XX, XX, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, // 1
    XX, XX, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, XX, XX, XX, XX, XX, // 2
    XX, XX, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74,
    72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74,
    72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, // 3
    XX, XX, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76,
    74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76,
    74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, XX, XX, // 4
    XX, XX, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80,
    78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80,
    78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, // 5
    XX, XX, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82,
    80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82,
    80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, XX, XX, // 6
    XX, XX, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86,
    84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86,
    84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, // 7
    XX, XX, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88,
    86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88,
    86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, // 8
    XX, XX, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90,
    88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90,
    88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, XX, XX, // 9
    XX, XX, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94,
    92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94,
    92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, // 10
    XX, XX, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96,
    94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96,
    94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, XX, XX, // 11
    XX, XX, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98,
    100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100,
    98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98,
    100, // 12
];

pub(super) const OL_TO_MDL: &[u8; MAX_OL as usize + 1] = &[
    0, 0, // 0
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, // 1
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, // 2
    74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72,
    74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72,
    74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, // 3
    76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74,
    76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74,
    76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, // 4
    80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78,
    80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78,
    80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, // 5
    82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80,
    82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80,
    82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, // 6
    86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84,
    86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84,
    86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, // 7
    88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86,
    88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86,
    88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, // 8
    90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88,
    90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88,
    90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, // 9
    94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92,
    94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92,
    94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, // 10
    96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94,
    96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94,
    96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, // 11
    100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100,
    98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98,
    100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100,
    98, // 12
];
