pub mod and_then;
pub mod filter;
pub mod map;
pub mod merge;
pub mod or;
pub mod or_else;
pub mod unwrap_or;

pub mod unwrap_or_else;

pub use {
    and_then::AndThen,
    filter::Filter,
    map::Map,
    merge::{Merge, Merge2, Merge3, Merge4},
    or::Or,
    or_else::OrElse,
    unwrap_or::UnwrapOr,
    unwrap_or_else::UnwrapOrElse,
};
