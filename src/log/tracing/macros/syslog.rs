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
    ($r:expr, $t:ident, $m:ident, $f:ident, message: $s:literal) => {
        |res| match ($r, res) {
            (Ok(()), Ok(t)) => tracing::$m!(
                log_level = $crate::log::tracing::entity::syslog::Severity::$t.as_int(),
                log_level_name = $crate::log::tracing::entity::syslog::Severity::$t.as_str(),
                log_facility = $crate::log::tracing::entity::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::tracing::entity::syslog::Facility::$f.as_str(),
                $s
            ),
            (Err(()), Err(e)) => tracing::$m!(
                log_level = $crate::log::tracing::entity::syslog::Severity::$t.as_int(),
                log_level_name = $crate::log::tracing::entity::syslog::Severity::$t.as_str(),
                log_facility = $crate::log::tracing::entity::syslog::Facility::$f.as_int(),
                log_facility_name = $crate::log::tracing::entity::syslog::Facility::$f.as_str(),
                $s
            ),
            _ => {}
        }
    };
}
#[macro_export]
macro_rules! emergency {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Emergency.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Emergency.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! emerg {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Emergency,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Emergency.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Emergency.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! alert {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Alert,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Alert,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Alert,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Alert,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Alert,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Alert,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Alert.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Alert.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! critical {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Critical.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Critical.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! crit {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Critical,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Critical.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Critical.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! error {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Error.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Error.as_str(),
            $ ($generated_args)*
        )
    };
}
#[macro_export]
macro_rules! err {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Error,
            error,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::error!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Error.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Error.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! warning {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::warn!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Warning.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Warning.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Warning,
            warn,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::warn!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Warning.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Warning.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! notice {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Notice,
            info,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Notice,
            info,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Notice,
            info,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Notice,
            info,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Notice,
            info,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Notice,
            info,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::info!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Notice.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Notice.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! info {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Informational,
            info,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Informational,
            info,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Informational,
            info,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Informational,
            info,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Informational,
            info,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Informational,
            info,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::info!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Informational.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Informational.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::trace!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Debug.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Debug.as_str(),
            $ ($generated_args)*
        )
    };
}

#[macro_export]
macro_rules! trace {
    ($i:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            UserlevelMessages,
            "{:?}"
        )
    };
    ($i:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            UserlevelMessages,
            $s
        )
    };
    ($i:ident, $f:ident) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            $f,
            "{:?}"
        )
    };
    ($i:ident, $f:ident, message: $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            $f,
            $s
        )
    };
    ($i:ident, $f:ident, $s:literal) => {
        $crate::syslog_func_generator!(
            $i(()),
            Debug,
            trace,
            $f,
            $s
        )
    };
    ($ ($generated_args:tt)*) => {
        $crate::prelude::tracing::trace!(
            log_level = $crate::log::tracing::entity::syslog::Severity::Debug.as_int(),
            log_level_name = $crate::log::tracing::entity::syslog::Severity::Debug.as_str(),
            $ ($generated_args)*
        )
    };
}
