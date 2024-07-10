pub mod blocking;
pub mod future;

pub mod fut {
    pub use super::future::*;
}
pub mod syn {
    pub use super::blocking::*;
}
