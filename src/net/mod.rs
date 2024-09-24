#[cfg(feature = "net-hostname")]
pub mod hostname;
#[cfg(feature = "net-http")]
pub mod http;
#[cfg(feature = "net-signature")]
pub mod signature;

#[cfg(feature = "net-hostname")]
pub use hostname::gethostname;
#[cfg(feature = "net-signature")]
pub use signature::Signature;
