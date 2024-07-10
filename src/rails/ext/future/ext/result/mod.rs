pub mod and_then;
pub mod map;
pub mod map_err;
pub mod map_or;
pub mod merge;
pub mod merge2;
pub mod merge3;
pub mod merge4;
pub mod or_else;
pub mod unwrap_or_else;

pub use {
    and_then::AndThen, map::Map, map_err::MapErr, map_or::MapOr, merge::Merge, merge2::Merge2,
    merge3::Merge3, merge4::Merge4, or_else::OrElse, unwrap_or_else::UnwrapOrElse,
};
