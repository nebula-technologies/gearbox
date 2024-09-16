pub mod entity;
pub mod formatter;
pub mod index;
pub mod layer;
pub mod macros;
pub mod value;

pub use formatter::LogFormatter;
pub use index::Index;
pub use value::Value;

use alloc::string::String;

pub fn get_exec_name() -> Option<String> {
    #[cfg(feature = "std")]
    {
        std::env::current_exe()
            .ok()
            .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
            .and_then(|s| s.into_string().ok())
    }
    #[cfg(not(feature = "std"))]
    {
        None
    }
}

pub enum IntegerConversionError {
    Underflow,
    NaN,
    Overflow,
}

pub trait ExtTryInto<I> {
    type Error;

    fn ext_try_into(self) -> Result<I, Self::Error>;
}

impl ExtTryInto<i64> for f64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<i64, Self::Error> {
        let f = self.round();

        if f.is_nan() {
            return Err(IntegerConversionError::NaN);
        }

        if f < 0.0 {
            return Err(IntegerConversionError::Underflow);
        }
        if f > i64::MAX as f64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(f as i64)
    }
}

impl ExtTryInto<i64> for u64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<i64, Self::Error> {
        let f = self;
        if f > i64::MAX as u64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(f as i64)
    }
}

impl ExtTryInto<u32> for f64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<u32, Self::Error> {
        let f = self.round();

        if f.is_nan() {
            return Err(IntegerConversionError::NaN);
        }

        if f < 0.0 {
            return Err(IntegerConversionError::Underflow);
        }
        if f > u32::MAX as f64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(f as u32)
    }
}
impl ExtTryInto<u32> for u64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<u32, Self::Error> {
        if self > u32::MAX as u64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(self as u32)
    }
}

impl ExtTryInto<u32> for i64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<u32, Self::Error> {
        if self < 0 {
            return Err(IntegerConversionError::Underflow);
        }
        if self > u32::MAX as i64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(self as u32)
    }
}

impl ExtTryInto<i32> for f64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<i32, Self::Error> {
        let f = self.round();

        if f.is_nan() {
            return Err(IntegerConversionError::NaN);
        }

        if f < i32::MIN as f64 {
            return Err(IntegerConversionError::Underflow);
        }
        if f > u32::MAX as f64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(f as i32)
    }
}
impl ExtTryInto<i32> for u64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<i32, Self::Error> {
        if self > i32::MAX as u64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(self as i32)
    }
}

impl ExtTryInto<i32> for i64 {
    type Error = IntegerConversionError;
    fn ext_try_into(self) -> Result<i32, Self::Error> {
        if self < i32::MIN as i64 {
            return Err(IntegerConversionError::Underflow);
        }
        if self > i32::MAX as i64 {
            return Err(IntegerConversionError::Overflow);
        }

        Ok(self as i32)
    }
}
