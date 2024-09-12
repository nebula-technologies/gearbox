pub mod facility;
pub mod severity;

pub use facility::Facility;
pub use severity::Severity;

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    IntegerOutOfBounds,
    FloatOutOfBounds,
    FloatOverflow,
    FloatUnderflow,
    FloatNaN,
    UnableToConvertBool,
    StringDoesNotMatchValidValues,
    UnableToConvertTimestamp,
    UnableToConvertSeverity,
    UnableToConvertNull,
    UnableToConvertArray,
    UnableToConvertMap,
}
