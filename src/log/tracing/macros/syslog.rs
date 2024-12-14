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
macro_rules! generate_log_facility {
    ($({$macro_name:ident, $syslog_level:ident, $tracing_level:ident, $facility:ident, #internal $dollar:tt}),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($i:ident) => {
                    $crate::syslog_func_generator!(
                        $i(()),
                        $syslog_level,
                        $tracing_level,
                        $facility,
                        "{:?}"
                    )
                };
                ($i:ident, message: $s:literal) => {
                    $crate::syslog_func_generator!(
                        $i(()),
                        $syslog_level,
                        $tracing_level,
                        $facility,
                        $s
                    )
                };
                ($i:ident, $s:literal) => {
                    $crate::syslog_func_generator!(
                        $i(()),
                        $syslog_level,
                        $tracing_level,
                        $facility,
                        $s
                    )
                };
                ($i:ident, $f:ident) => {
                    $crate::syslog_func_generator!(
                        $i(()),
                        $syslog_level,
                        $tracing_level,
                        $f,
                        "{:?}"
                    )
                };
                ($i:ident, $f:ident, message: $s:literal) => {
                    $crate::syslog_func_generator!(
                        $i(()),
                        $syslog_level,
                        $tracing_level,
                        $f,
                        $s
                    )
                };
                ($i:ident, $f:ident, $s:literal) => {
                    $crate::syslog_func_generator!(
                        $i(()),
                        $syslog_level,
                        $tracing_level,
                        $f,
                        $s
                    )
                };
                ($dollar ($dollar generated_args:tt)*) => {
                    $crate::prelude::tracing::$tracing_level!(
                        log_level = $crate::log::tracing::entity::syslog::Severity::$syslog_level.as_int(),
                        log_level_name = $crate::log::tracing::entity::syslog::Severity::$syslog_level.as_str(),
                        $dollar ($dollar generated_args)*
                    )
                };
            }

            pub use $macro_name;
        )*
    };
}

generate_log_facility!(
    {emergency, Emergency, error, UserlevelMessages, #internal $},
    {emerg, Emergency, error, UserlevelMessages, #internal $},
    {alert, Alert, error, UserlevelMessages, #internal $},
    {critical, Critical, error, UserlevelMessages, #internal $},
    {crit, Critical, error, UserlevelMessages, #internal $},
    {error, Error, error, UserlevelMessages, #internal $},
    {warning, Warning, warn, UserlevelMessages, #internal $},
    {notice, Notice, info, UserlevelMessages, #internal $},
    {info, Informational, info, UserlevelMessages, #internal $},
    {debug, Debug, trace, UserlevelMessages, #internal $},
    {trace, Debug, trace, UserlevelMessages, #internal $},
);
