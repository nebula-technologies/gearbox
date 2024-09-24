#[cfg(feature = "collections-const-hash-map")]
pub mod const_hash_map;
#[cfg(feature = "collections-hash-map")]
pub mod hash_map;
#[cfg(feature = "collections-simple-linked-list")]
pub mod simple_linked_list;
#[cfg(feature = "collections-vec-deque")]
pub mod vec_deque;

#[cfg(feature = "collections-const-hash-map")]
pub use const_hash_map::HashMap as ConstHashMap;
#[cfg(feature = "collections-hash-map")]
pub use hash_map::HashMap;
#[cfg(feature = "collections-simple-linked-list")]
pub use simple_linked_list::SimpleLinkedList;
#[cfg(feature = "collections-vec-deque")]
pub use vec_deque::VecDeque;

#[cfg(not(any(
    feature = "collections-vec-deque",
    feature = "collections-simple-linked-list",
    feature = "collections-hash-map",
    feature = "collections-const-hash-map"
)))]
pub struct Empty {}
