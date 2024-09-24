#[cfg(feature = "error-tracer")]
pub mod tracer;
#[cfg(feature = "error-type-registry")]
pub mod type_registry;

#[cfg(feature = "error-tracer")]
pub use tracer::{DynTracerError, TracerError};

#[cfg(feature = "error-type-registry")]
pub use type_registry::TypeRegistry;
