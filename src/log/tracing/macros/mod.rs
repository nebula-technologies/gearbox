#[cfg(all(
    feature = "log-tracing-macros",
    not(feature = "log-tracing-macros-syslog")
))]
pub mod common;
#[cfg(feature = "log-tracing-macros-syslog")]
pub mod syslog;
