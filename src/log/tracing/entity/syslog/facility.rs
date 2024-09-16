use crate::log::tracing::entity::syslog::ConversionError;
use crate::log::tracing::Value;
use alloc::string::{String, ToString};

#[derive(Debug, PartialEq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
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

impl Facility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KernelMessages => "kernel",
            Self::UserlevelMessages => "user",
            Self::MailSystem => "mail",
            Self::SystemDaemons => "system",
            Self::SecurityMessages => "security",
            Self::MessagesGeneratedInternallyBySyslogd => "syslog",
            Self::LinePrinterSubsystem => "printer",
            Self::NetworkNewsSubsystem => "news",
            Self::UucpSubsystem => "uucp",
            Self::ClockDaemon => "clock",
            Self::AuthorizationMessages => "auth",
            Self::FtpDaemon => "ftp",
            Self::NtpSubsystem => "ntp",
            Self::LogAudit => "audit",
            Self::LogAlert => "alert",
            Self::Note2 => "clock2",
            Self::LocalUse0 => "local0",
            Self::LocalUse1 => "local1",
            Self::LocalUse2 => "local2",
            Self::LocalUse3 => "local3",
            Self::LocalUse4 => "local4",
            Self::LocalUse5 => "local5",
            Self::LocalUse6 => "local6",
            Self::LocalUse7 => "local7",
        }
    }

    pub fn as_int(&self) -> u8 {
        match self {
            Self::KernelMessages => 0,
            Self::UserlevelMessages => 1,
            Self::MailSystem => 2,
            Self::SystemDaemons => 3,
            Self::SecurityMessages => 4,
            Self::MessagesGeneratedInternallyBySyslogd => 5,
            Self::LinePrinterSubsystem => 6,
            Self::NetworkNewsSubsystem => 7,
            Self::UucpSubsystem => 8,
            Self::ClockDaemon => 9,
            Self::AuthorizationMessages => 10,
            Self::FtpDaemon => 11,
            Self::NtpSubsystem => 12,
            Self::LogAudit => 13,
            Self::LogAlert => 14,
            Self::Note2 => 15,
            Self::LocalUse0 => 16,
            Self::LocalUse1 => 17,
            Self::LocalUse2 => 18,
            Self::LocalUse3 => 19,
            Self::LocalUse4 => 20,
            Self::LocalUse5 => 21,
            Self::LocalUse6 => 22,
            Self::LocalUse7 => 23,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FacilityError {
    ConversionError(ConversionError),
}

impl TryFrom<Value> for Facility {
    type Error = FacilityError;
    fn try_from(l: Value) -> Result<Self, Self::Error> {
        (&l).try_into()
    }
}
impl TryFrom<&Value> for Facility {
    type Error = FacilityError;
    fn try_from(l: &Value) -> Result<Self, Self::Error> {
        match l {
            Value::Int(i) => Self::try_from(i),
            Value::UInt(u) => Self::try_from(u),
            Value::Float(f) => Self::try_from(f),
            Value::Bool(_) => Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertBool,
            )),
            Value::String(s) => Self::try_from(s),
            Value::Debug(s) => Self::try_from(s),
            Value::TimeStamp(_) => Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertTimestamp,
            )),
            Value::Severity(_) => Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertSeverity,
            )),
            Value::Null => Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertNull,
            )),
            Value::Array(_) => Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertArray,
            )),
            Value::Map(_) => Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertMap,
            )),
        }
    }
}

impl TryFrom<i64> for Facility {
    type Error = FacilityError;

    fn try_from(i: i64) -> Result<Self, Self::Error> {
        (&i).try_into()
    }
}

impl TryFrom<&i64> for Facility {
    type Error = FacilityError;

    fn try_from(i: &i64) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Self::KernelMessages),
            1 => Ok(Self::UserlevelMessages),
            2 => Ok(Self::MailSystem),
            3 => Ok(Self::SystemDaemons),
            4 => Ok(Self::SecurityMessages),
            5 => Ok(Self::MessagesGeneratedInternallyBySyslogd),
            6 => Ok(Self::LinePrinterSubsystem),
            7 => Ok(Self::NetworkNewsSubsystem),
            8 => Ok(Self::UucpSubsystem),
            9 => Ok(Self::ClockDaemon),
            10 => Ok(Self::AuthorizationMessages),
            11 => Ok(Self::FtpDaemon),
            12 => Ok(Self::NtpSubsystem),
            13 => Ok(Self::LogAudit),
            14 => Ok(Self::LogAlert),
            15 => Ok(Self::Note2),
            16 => Ok(Self::LocalUse0),
            17 => Ok(Self::LocalUse1),
            18 => Ok(Self::LocalUse2),
            19 => Ok(Self::LocalUse3),
            20 => Ok(Self::LocalUse4),
            21 => Ok(Self::LocalUse5),
            22 => Ok(Self::LocalUse6),
            23 => Ok(Self::LocalUse7),
            _ => Err(FacilityError::ConversionError(
                ConversionError::IntegerOutOfBounds,
            )),
        }
    }
}

impl TryFrom<String> for Facility {
    type Error = FacilityError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        (&s).try_into()
    }
}

impl TryFrom<&String> for Facility {
    type Error = FacilityError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "kernelmessages" => Ok(Self::KernelMessages),
            "userlevelmessages" => Ok(Self::UserlevelMessages),
            "mailsystem" => Ok(Self::MailSystem),
            "systemdaemons" => Ok(Self::SystemDaemons),
            "securitymessages" => Ok(Self::SecurityMessages),
            "messagesgeneratedinternallybysyslogd" => {
                Ok(Self::MessagesGeneratedInternallyBySyslogd)
            }
            "lineprintersubsystem" => Ok(Self::LinePrinterSubsystem),
            "networknewssubsystem" => Ok(Self::NetworkNewsSubsystem),
            "uucpsubsystem" => Ok(Self::UucpSubsystem),
            "clockdaemon" => Ok(Self::ClockDaemon),
            "authorizationmessages" => Ok(Self::AuthorizationMessages),
            "ftpdaemon" => Ok(Self::FtpDaemon),
            "ntpsubsystem" => Ok(Self::NtpSubsystem),
            "logaudit" => Ok(Self::LogAudit),
            "logalert" => Ok(Self::LogAlert),
            "note2" => Ok(Self::Note2),
            "localuse0" => Ok(Self::LocalUse0),
            "localuse1" => Ok(Self::LocalUse1),
            "localuse2" => Ok(Self::LocalUse2),
            "localuse3" => Ok(Self::LocalUse3),
            "localuse4" => Ok(Self::LocalUse4),
            "localuse5" => Ok(Self::LocalUse5),
            "localuse6" => Ok(Self::LocalUse6),
            "localuse7" => Ok(Self::LocalUse7),
            _ => Err(FacilityError::ConversionError(
                ConversionError::StringDoesNotMatchValidValues,
            )),
        }
    }
}

impl TryFrom<u64> for Facility {
    type Error = FacilityError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        (&u).try_into()
    }
}

impl TryFrom<&u64> for Facility {
    type Error = FacilityError;

    fn try_from(u: &u64) -> Result<Self, Self::Error> {
        Self::try_from(u.to_owned() as i64)
    }
}

impl TryFrom<f64> for Facility {
    type Error = FacilityError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        (&f).try_into()
    }
}
impl TryFrom<&f64> for Facility {
    type Error = FacilityError;

    fn try_from(f: &f64) -> Result<Self, Self::Error> {
        let f = f.round();

        if f.is_nan() {
            return Err(FacilityError::ConversionError(ConversionError::FloatNaN));
        }
        if f < 0.0 {
            return Err(FacilityError::ConversionError(
                ConversionError::FloatOverflow,
            ));
        }
        if f > i64::MAX as f64 {
            return Err(FacilityError::ConversionError(
                ConversionError::FloatOverflow,
            ));
        }
        Self::try_from(f as i64)
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
    use crate::log::tracing::entity::syslog::Severity;
    use crate::log::tracing::Value;
    use crate::time::DateTime;

    #[test]
    fn test_i64_to_facility() {
        assert_eq!(Facility::try_from(0i64), Ok(Facility::KernelMessages));
        assert_eq!(Facility::try_from(1i64), Ok(Facility::UserlevelMessages));
        assert_eq!(Facility::try_from(23i64), Ok(Facility::LocalUse7));
        assert_eq!(
            Facility::try_from(24i64),
            Err(FacilityError::ConversionError(
                ConversionError::IntegerOutOfBounds
            ))
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
            Err(FacilityError::ConversionError(
                ConversionError::StringDoesNotMatchValidValues
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
            Err(FacilityError::ConversionError(
                ConversionError::IntegerOutOfBounds
            ))
        );
    }

    #[test]
    fn test_f64_to_facility() {
        assert_eq!(Facility::try_from(0.0), Ok(Facility::KernelMessages));
        assert_eq!(Facility::try_from(1.0), Ok(Facility::UserlevelMessages));
        assert_eq!(Facility::try_from(23.0), Ok(Facility::LocalUse7));
        assert_eq!(
            Facility::try_from(24.0),
            Err(FacilityError::ConversionError(
                ConversionError::IntegerOutOfBounds
            ))
        );
        assert_eq!(
            Facility::try_from(f64::NAN),
            Err(FacilityError::ConversionError(ConversionError::FloatNaN))
        );
    }

    #[test]
    fn test_logvalue_to_facility() {
        assert_eq!(
            Facility::try_from(Value::Int(0)),
            Ok(Facility::KernelMessages)
        );
        assert_eq!(
            Facility::try_from(Value::UInt(1)),
            Ok(Facility::UserlevelMessages)
        );
        assert_eq!(
            Facility::try_from(Value::Float(2.0)),
            Ok(Facility::MailSystem)
        );
        assert_eq!(
            Facility::try_from(Value::Bool(true)),
            Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertBool
            ))
        );
        assert_eq!(
            Facility::try_from(Value::String("localuse4".to_string())),
            Ok(Facility::LocalUse4)
        );
        assert_eq!(
            Facility::try_from(Value::Debug("invalid".to_string())),
            Err(FacilityError::ConversionError(
                ConversionError::StringDoesNotMatchValidValues
            ))
        );
        assert_eq!(
            Facility::try_from(Value::TimeStamp(DateTime::from_secs(0))),
            Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertTimestamp
            ))
        );
        assert_eq!(
            Facility::try_from(Value::Severity(Severity::try_from(0i64).unwrap())),
            Err(FacilityError::ConversionError(
                ConversionError::UnableToConvertSeverity
            ))
        );
    }
}
