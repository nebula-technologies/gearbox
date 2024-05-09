pub mod facility;
#[cfg(feature = "syslog-macro")]
pub mod macros;
pub mod severity;

pub use facility::Facility;
pub use severity::Severity;
