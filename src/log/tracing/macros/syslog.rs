#[macro_export]
macro_rules! syslog_func_generator {
    ($r:expr, $t:ident, $m:ident, $f:ident, $s:literal) => {
        |res| match ($r, res) {
            (Ok(()), Ok(t)) => tracing::$m!(
                log_level = $crate::log::tracing::entity::syslog::Severity::$t.as_int(),
                log_level_name = $crate::log::tracing::entity::syslog::Severity::$t.as_str(),
                log_facility = $crate::log::tracing::entity::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::tracing::entity::syslog::Facility::$f.as_str(),
                $s,
                t
            ),
            (Err(()), Err(e)) => tracing::$m!(
                log_level = $crate::log::tracing::entity::syslog::Severity::$t.as_int(),
                log_level_name = $crate::log::tracing::entity::syslog::Severity::$t.as_str(),
                log_facility = $crate::log::tracing::entity::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::tracing::entity::syslog::Facility::$f.as_str(),
                $s,
                e
            ),
            _ => {}
        }
    };
}
#[macro_export]
macro_rules! emergency {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, $f, $s)
    };
    ($($arg:tt)*) => {
              tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Emergency.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Emergency.as_str(), $($arg)*);
          };
}

#[macro_export]
macro_rules! emerg {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Emergency, error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Emergency.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Emergency.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! alert {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Alert, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Alert, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Alert, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Alert, error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Alert.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Alert.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! critical {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Critical, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Critical, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Critical, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Critical, error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Critical.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Critical.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! crit {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Critical, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Critical, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Critical, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Critical, error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Critical.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Critical.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Error, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Error, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Error, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Error, error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Error.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Error.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! err {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Error, error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Error, error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Error, error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Error, error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!(log_level=$crate::log::tracing::subscriber::entity::Severity::Error.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Error.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warning {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::warn!(log_level=$crate::log::tracing::subscriber::entity::Severity::Warning.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Warning.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Warning, warn, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::warn!(log_level=$crate::log::tracing::subscriber::entity::Severity::Warning.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Warning.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! notice {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Notice, info, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Notice, info, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Notice, info, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Notice, info, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::info!(log_level=$crate::log::tracing::subscriber::entity::Severity::Notice.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Notice.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Informational, debug, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Informational, debug, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Informational, debug, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Informational, debug, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::debug!(log_level=$crate::log::tracing::subscriber::entity::Severity::Info.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Info.as_str(), $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($i:ident) => {
        $crate::syslog_func_generator!($i(()), Debug, trace, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Debug, trace, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!($i(()), Debug, trace, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!($i(()), Debug, trace, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::trace!(log_level=$crate::log::tracing::subscriber::entity::Severity::Debug.as_int(), log_level_name=$crate::log::tracing::subscriber::entity::Severity::Debug.as_str(), $($arg)*);
    };
}
