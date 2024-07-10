use crate::log::fmt::log_value::LogValue;
use alloc::string::{String, ToString};

#[derive(Debug, PartialEq)]
pub enum Facility {
    KernelMessages = 0,
    UserlevelMessages = 1,
    MailSystem = 2,
    SystemDaemons = 3,
    SecurityMessages = 4,
    MessagesGeneratedInternallyBySyslogd = 5,
    LinePrinterSubsystem = 6,
    NetworkNewsSubsystem = 7,
    UucpSubsystem = 8,
    ClockDaemon = 9,
    AuthorizationMessages = 10,
    FtpDaemon = 11,
    NtpSubsystem = 12,
    LogAudit = 13,
    LogAlert = 14,
    Note2 = 15,
    LocalUse0 = 16,
    LocalUse1 = 17,
    LocalUse2 = 18,
    LocalUse3 = 19,
    LocalUse4 = 20,
    LocalUse5 = 21,
    LocalUse6 = 22,
    LocalUse7 = 23,
}

#[derive(Debug, PartialEq)]
pub enum FacilityConversionError {
    IntegerNumberNotPartOfFacilityRange,
    UnsignedIntegerNumberNotPartOfFacilityRange,
    FloatingNumberNotPartOfFacilityRange(String),
    BoolCannotBeConverted,
    UnableToConvertTimestamp,
    StringCannotBeConverted(String),
    DebugStringCannotBeConverted(String),
    UnableToConvertSeverity,
}

impl TryFrom<LogValue> for Facility {
    type Error = FacilityConversionError;
    fn try_from(l: LogValue) -> Result<Self, Self::Error> {
        match l {
            LogValue::I64(i) => Facility::try_from(i),
            LogValue::U64(u) => Facility::try_from(u),
            LogValue::F64(f) => Facility::try_from(f),
            LogValue::Bool(_) => Err(FacilityConversionError::BoolCannotBeConverted),
            LogValue::String(s) => Self::try_from(s),
            LogValue::Debug(s) => Self::try_from(s),
            LogValue::TimeStamp(_) => Err(FacilityConversionError::UnableToConvertTimestamp),
            LogValue::Severity(_) => Err(FacilityConversionError::UnableToConvertSeverity),
        }
    }
}

impl TryFrom<i64> for Facility {
    type Error = FacilityConversionError;

    fn try_from(i: i64) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Facility::KernelMessages),
            1 => Ok(Facility::UserlevelMessages),
            2 => Ok(Facility::MailSystem),
            3 => Ok(Facility::SystemDaemons),
            4 => Ok(Facility::SecurityMessages),
            5 => Ok(Facility::MessagesGeneratedInternallyBySyslogd),
            6 => Ok(Facility::LinePrinterSubsystem),
            7 => Ok(Facility::NetworkNewsSubsystem),
            8 => Ok(Facility::UucpSubsystem),
            9 => Ok(Facility::ClockDaemon),
            10 => Ok(Facility::AuthorizationMessages),
            11 => Ok(Facility::FtpDaemon),
            12 => Ok(Facility::NtpSubsystem),
            13 => Ok(Facility::LogAudit),
            14 => Ok(Facility::LogAlert),
            15 => Ok(Facility::Note2),
            16 => Ok(Facility::LocalUse0),
            17 => Ok(Facility::LocalUse1),
            18 => Ok(Facility::LocalUse2),
            19 => Ok(Facility::LocalUse3),
            20 => Ok(Facility::LocalUse4),
            21 => Ok(Facility::LocalUse5),
            22 => Ok(Facility::LocalUse6),
            23 => Ok(Facility::LocalUse7),
            _ => Err(FacilityConversionError::IntegerNumberNotPartOfFacilityRange),
        }
    }
}

impl TryFrom<String> for Facility {
    type Error = FacilityConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "kernelmessages" => Ok(Facility::KernelMessages),
            "userlevelmessages" => Ok(Facility::UserlevelMessages),
            "mailsystem" => Ok(Facility::MailSystem),
            "systemdaemons" => Ok(Facility::SystemDaemons),
            "securitymessages" => Ok(Facility::SecurityMessages),
            "messagesgeneratedinternallybysyslogd" => {
                Ok(Facility::MessagesGeneratedInternallyBySyslogd)
            }
            "lineprintersubsystem" => Ok(Facility::LinePrinterSubsystem),
            "networknewssubsystem" => Ok(Facility::NetworkNewsSubsystem),
            "uucpsubsystem" => Ok(Facility::UucpSubsystem),
            "clockdaemon" => Ok(Facility::ClockDaemon),
            "authorizationmessages" => Ok(Facility::AuthorizationMessages),
            "ftpdaemon" => Ok(Facility::FtpDaemon),
            "ntpsubsystem" => Ok(Facility::NtpSubsystem),
            "logaudit" => Ok(Facility::LogAudit),
            "logalert" => Ok(Facility::LogAlert),
            "note2" => Ok(Facility::Note2),
            "localuse0" => Ok(Facility::LocalUse0),
            "localuse1" => Ok(Facility::LocalUse1),
            "localuse2" => Ok(Facility::LocalUse2),
            "localuse3" => Ok(Facility::LocalUse3),
            "localuse4" => Ok(Facility::LocalUse4),
            "localuse5" => Ok(Facility::LocalUse5),
            "localuse6" => Ok(Facility::LocalUse6),
            "localuse7" => Ok(Facility::LocalUse7),
            _ => Err(FacilityConversionError::StringCannotBeConverted(s)),
        }
    }
}

impl TryFrom<u64> for Facility {
    type Error = FacilityConversionError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Facility::try_from(u as i64)
    }
}
impl TryFrom<f64> for Facility {
    type Error = FacilityConversionError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        let f = f.round();

        if f.is_nan() {
            return Err(
                FacilityConversionError::FloatingNumberNotPartOfFacilityRange(
                    "Float NaN".to_string(),
                ),
            );
        }
        if f < 0.0 {
            return Err(
                FacilityConversionError::FloatingNumberNotPartOfFacilityRange(
                    "Float Underflow".to_string(),
                ),
            );
        }
        if f > i64::MAX as f64 {
            return Err(
                FacilityConversionError::FloatingNumberNotPartOfFacilityRange(
                    "Float Overflow".to_string(),
                ),
            );
        }
        Facility::try_from(f as i64)
    }
}

impl From<Facility> for u32 {
    fn from(facility: Facility) -> u32 {
        Self::from(&facility)
    }
}

impl From<&Facility> for u32 {
    fn from(facility: &Facility) -> u32 {
        match facility {
            Facility::KernelMessages => 0,
            Facility::UserlevelMessages => 1,
            Facility::MailSystem => 2,
            Facility::SystemDaemons => 3,
            Facility::SecurityMessages => 4,
            Facility::MessagesGeneratedInternallyBySyslogd => 5,
            Facility::LinePrinterSubsystem => 6,
            Facility::NetworkNewsSubsystem => 7,
            Facility::UucpSubsystem => 8,
            Facility::ClockDaemon => 9,
            Facility::AuthorizationMessages => 10,
            Facility::FtpDaemon => 11,
            Facility::NtpSubsystem => 12,
            Facility::LogAudit => 13,
            Facility::LogAlert => 14,
            Facility::Note2 => 15,
            Facility::LocalUse0 => 16,
            Facility::LocalUse1 => 17,
            Facility::LocalUse2 => 18,
            Facility::LocalUse3 => 19,
            Facility::LocalUse4 => 20,
            Facility::LocalUse5 => 21,
            Facility::LocalUse6 => 22,
            Facility::LocalUse7 => 23,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::fmt::severity::Severity;
    use crate::time::DateTime;

    #[test]
    fn test_i64_to_facility() {
        assert_eq!(Facility::try_from(0i64), Ok(Facility::KernelMessages));
        assert_eq!(Facility::try_from(1i64), Ok(Facility::UserlevelMessages));
        assert_eq!(Facility::try_from(23i64), Ok(Facility::LocalUse7));
        assert_eq!(
            Facility::try_from(24i64),
            Err(FacilityConversionError::IntegerNumberNotPartOfFacilityRange)
        );
    }

    #[test]
    fn test_string_to_facility() {
        assert_eq!(
            Facility::try_from("kernelmessages".to_string()),
            Ok(Facility::KernelMessages)
        );
        assert_eq!(
            Facility::try_from("userlevelmessages".to_string()),
            Ok(Facility::UserlevelMessages)
        );
        assert_eq!(
            Facility::try_from("localuse7".to_string()),
            Ok(Facility::LocalUse7)
        );
        assert_eq!(
            Facility::try_from("invalid".to_string()),
            Err(FacilityConversionError::StringCannotBeConverted(
                "invalid".to_string()
            ))
        );
    }

    #[test]
    fn test_u64_to_facility() {
        assert_eq!(Facility::try_from(0u64), Ok(Facility::KernelMessages));
        assert_eq!(Facility::try_from(1u64), Ok(Facility::UserlevelMessages));
        assert_eq!(Facility::try_from(23u64), Ok(Facility::LocalUse7));
        assert_eq!(
            Facility::try_from(24u64),
            Err(FacilityConversionError::IntegerNumberNotPartOfFacilityRange)
        );
    }

    #[test]
    fn test_f64_to_facility() {
        assert_eq!(Facility::try_from(0.0), Ok(Facility::KernelMessages));
        assert_eq!(Facility::try_from(1.0), Ok(Facility::UserlevelMessages));
        assert_eq!(Facility::try_from(23.0), Ok(Facility::LocalUse7));
        assert_eq!(
            Facility::try_from(24.0),
            Err(FacilityConversionError::IntegerNumberNotPartOfFacilityRange)
        );
        assert_eq!(
            Facility::try_from(f64::NAN),
            Err(
                FacilityConversionError::FloatingNumberNotPartOfFacilityRange(
                    "Float NaN".to_string()
                )
            )
        );
    }

    #[test]
    fn test_logvalue_to_facility() {
        assert_eq!(
            Facility::try_from(LogValue::I64(0)),
            Ok(Facility::KernelMessages)
        );
        assert_eq!(
            Facility::try_from(LogValue::U64(1)),
            Ok(Facility::UserlevelMessages)
        );
        assert_eq!(
            Facility::try_from(LogValue::F64(2.0)),
            Ok(Facility::MailSystem)
        );
        assert_eq!(
            Facility::try_from(LogValue::Bool(true)),
            Err(FacilityConversionError::BoolCannotBeConverted)
        );
        assert_eq!(
            Facility::try_from(LogValue::String("localuse4".to_string())),
            Ok(Facility::LocalUse4)
        );
        assert_eq!(
            Facility::try_from(LogValue::Debug("invalid".to_string())),
            Err(FacilityConversionError::StringCannotBeConverted(
                "invalid".to_string()
            ))
        );
        assert_eq!(
            Facility::try_from(LogValue::TimeStamp(DateTime::from_secs(0))),
            Err(FacilityConversionError::UnableToConvertTimestamp)
        );
        assert_eq!(
            Facility::try_from(LogValue::Severity(Severity::try_from(0i64).unwrap())),
            Err(FacilityConversionError::UnableToConvertSeverity)
        );
    }
}
