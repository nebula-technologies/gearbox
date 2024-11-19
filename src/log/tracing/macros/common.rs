#[macro_export]
macro_rules! log_func_generator {
    ($r:expr, $m:ident, $f:ident, $s:literal) => {
        |res| match ($r, res) {
            (Ok(()), Ok(t)) => tracing::$m!(
                log_facility = $crate::log::tracing::entity::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::tracing::entity::syslog::Facility::$f.as_str(),
                $s,
                t
            ),
            (Err(()), Err(e)) => tracing::$m!(
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
macro_rules! error {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), error, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), error, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), error, $f, $s)
    };
    ($($arg:tt)*) => {
        $crate::externs::tracing::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! warning {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), warn, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), warn, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, $f, $s)
    };
    ($($arg:tt)*) => {
        $crate::externs::tracing::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($i:ident) => {
        $crate::log_func_generator!($i(()),  warn, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), warn, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, $f, $s)
    };
    ($($arg:tt)*) => {
        $crate::externs::tracing::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), info, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), info, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), info, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), info, $f, $s)
    };
    ($($arg:tt)*) => {
        $crate::externs::tracing::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), debug, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), debug, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), debug, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), debug, $f, $s)
    };
    ($($arg:tt)*) => {
        $crate::externs::tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! trace {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), Debug, trace, UserlevelMessages, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Debug, trace, UserlevelMessages, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), Debug, trace, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Debug, trace, $f, $s)
    };
    ($($arg:tt)*) => {
        $crate::externs::tracing::trace!($($arg)*);
    };
}
