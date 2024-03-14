#[macro_export]
macro_rules! emergency {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Emergency.as_int(), log_level_name=crate::log::syslog::Severity::Emergency.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! emerg {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Emergency.as_int(), log_level_name=crate::log::syslog::Severity::Emergency.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! alert {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Alert.as_int(), log_level_name=crate::log::syslog::Severity::Alert.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Critical.as_int(), log_level_name=crate::log::syslog::Severity::Critical.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! crit {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Critical.as_int(), log_level_name=crate::log::syslog::Severity::Critical.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Error.as_int(), log_level_name=crate::log::syslog::Severity::Error.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        tracing::error!(log_level=crate::log::syslog::Severity::Error.as_int(), log_level_name=crate::log::syslog::Severity::Error.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        tracing::warn!(log_level=crate::log::syslog::Severity::Warning.as_int(), log_level_name=crate::log::syslog::Severity::Warning.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        tracing::warn!(log_level=crate::log::syslog::Severity::Warning.as_int(), log_level_name=crate::log::syslog::Severity::Warning.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => {
        tracing::info!(log_level=crate::log::syslog::Severity::Notice.as_int(), log_level_name=crate::log::syslog::Severity::Notice.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        tracing::debug!(log_level=crate::log::syslog::Severity::Info.as_int(), log_level_name=crate::log::syslog::Severity::Info.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        tracing::trace!(log_level=crate::log::syslog::Severity::Debug.as_int(), log_level_name=crate::log::syslog::Severity::Debug.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! kernel {
    () => {
        crate::log::Facility::Kernel
    };
}

#[macro_export]
macro_rules! user {
    () => {
        crate::log::Facility::User
    };
}

#[macro_export]
macro_rules! mail {
    () => {
        crate::log::Facility::Mail
    };
}

#[macro_export]
macro_rules! system {
    () => {
        crate::log::Facility::System
    };
}

#[macro_export]
macro_rules! security {
    () => {
        crate::log::Facility::Security
    };
}

#[macro_export]
macro_rules! syslog {
    () => {
        crate::log::Facility::Syslog
    };
}

#[macro_export]
macro_rules! printer {
    () => {
        crate::log::Facility::Printer
    };
}

#[macro_export]
macro_rules! news {
    () => {
        crate::log::Facility::News
    };
}

#[macro_export]
macro_rules! uucp {
    () => {
        crate::log::Facility::Uucp
    };
}

#[macro_export]
macro_rules! clock {
    () => {
        crate::log::Facility::Clock
    };
}

#[macro_export]
macro_rules! auth {
    () => {
        crate::log::Facility::Auth
    };
}

#[macro_export]
macro_rules! ftp {
    () => {
        crate::log::Facility::Ftp
    };
}

#[macro_export]
macro_rules! ntp {
    () => {
        crate::log::Facility::Ntp
    };
}

#[macro_export]
macro_rules! audit {
    () => {
        crate::log::Facility::Audit
    };
}

#[macro_export]
macro_rules! f_alert {
    () => {
        crate::log::Facility::Alert
    };
}

#[macro_export]
macro_rules! clock2 {
    () => {
        crate::log::Facility::Clock2
    };
}

#[macro_export]
macro_rules! local0 {
    () => {
        crate::log::Facility::Local0
    };
}

#[macro_export]
macro_rules! local1 {
    () => {
        crate::log::Facility::Local1
    };
}

#[macro_export]
macro_rules! local2 {
    () => {
        crate::log::Facility::Local2
    };
}

#[macro_export]
macro_rules! local3 {
    () => {
        crate::log::Facility::Local3
    };
}

#[macro_export]
macro_rules! local4 {
    () => {
        crate::log::Facility::Local4
    };
}

#[macro_export]
macro_rules! local5 {
    () => {
        crate::log::Facility::Local5
    };
}

#[macro_export]
macro_rules! local6 {
    () => {
        crate::log::Facility::Local6
    };
}

#[macro_export]
macro_rules! local7 {
    () => {
        crate::log::Facility::Local7
    };
}
