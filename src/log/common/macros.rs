#[macro_export]
macro_rules! log_func_generator {
    ($r:expr, $m:ident, $f:ident, $s:literal) => {
        |res| match ($r, res) {
            (Ok(()), Ok(t)) => tracing::$m!(
                log_facility = $crate::log::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::syslog::Facility::$f.as_str(),
                $s,
                t
            ),
            (Err(()), Err(e)) => tracing::$m!(
                log_facility = $crate::log::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::syslog::Facility::$f.as_str(),
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
        $crate::log_func_generator!($i(()), error, User, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), error, User, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), error, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), error, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! warning {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), warn, User, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, User, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), warn, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($i:ident) => {
        $crate::log_func_generator!($i(()),  warn, User, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, User, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), warn, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), warn, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), Info, info, User, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Info, info, User, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), Info, info, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Info, info, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), Debug, debug, User, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Debug, debug, User, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), Debug, debug, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Debug, debug, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! trace {
    ($i:ident) => {
        $crate::log_func_generator!($i(()), Debug, trace, User, "{:?}")
    };
    ($i:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Debug, trace, User, $s)
    };
    ($i:ident, $f:ident) => {
        $crate::log_func_generator!($i(()), Debug, trace, $f, "{:?}")
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::log_func_generator!($i(()), Debug, trace, $f, $s)
    };
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*);
    };
}
