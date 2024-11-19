//! ## FutureOptional and FutureResult Documentation
//!
//! ### FutureOptional
//!
//! An extension trait for `Future`s that yield `Option<T>` that provides a variety of convenient adapters.
//!
//! #### `map`
//!
//! Map this future's optional output to a different type, returning a new future of the resulting type.
//!
//! This function is similar to the `Option::map` where it will change the type of the underlying future. This is useful to chain along a computation once a future has been resolved and if it is `Some`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(1) };
//! let res = future_opt.map(|t| async move { 5 });
//! let final_res = res.await;
//! assert_eq!(final_res, Some(5));
//! # });
//! ```
//!
//! #### `and_then`
//!
//! Chains this future with another future if the output is `Some`, returning a new future of the resulting type.
//!
//! This function is similar to the `Option::and_then` where it will chain another computation if the future resolves to `Some`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(1) };
//! let res = future_opt.and_then(|t| async move { Some(t + 1) });
//! let final_res = res.await;
//! assert_eq!(final_res, Some(2));
//! # });
//! ```
//!
//! #### `filter`
//!
//! Filters the output of this future, returning `None` if the predicate returns `false`.
//!
//! This function is similar to the `Option::filter` where it will return `None` if the predicate returns `false`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(4) };
//! let res = future_opt.filter(|x| *x > 2);
//! let final_res = res.await;
//! assert_eq!(final_res, Some(4));
//! # });
//! ```
//!
//! #### `or`
//!
//! Returns this future's output if it is `Some`, otherwise returns the provided fallback.
//!
//! This function is similar to the `Option::or` where it will return the provided fallback if the future resolves to `None`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(4) };
//! let res = future_opt.or(Some(10));
//! let final_res = res.await;
//! assert_eq!(final_res, Some(4));
//!
//! let future_opt = async { None };
//! let res = future_opt.or(Some(10));
//! let final_res = res.await;
//! assert_eq!(final_res, Some(10));
//! # });
//! ```
//!
//! #### `or_else`
//!
//! Returns this future's output if it is `Some`, otherwise calls the provided fallback function.
//!
//! This function is similar to the `Option::or_else` where it will call the provided fallback function if the future resolves to `None`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(4) };
//! let res = future_opt.or_else(|| async { Some(10) });
//! let final_res = res.await;
//! assert_eq!(final_res, Some(4));
//!
//! let future_opt = async { None };
//! let res = future_opt.or_else(|| async { Some(10) });
//! let final_res = res.await;
//! assert_eq!(final_res, Some(10));
//! # });
//! ```
//!
//! #### `unwrap_or`
//!
//! Returns this future's output if it is `Some`, otherwise returns the provided default.
//!
//! This function is similar to the `Option::unwrap_or` where it will return the provided default if the future resolves to `None`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(4) };
//! let res = future_opt.unwrap_or(10);
//! let final_res = res.await;
//! assert_eq!(final_res, 4);
//!
//! let future_opt = async { None };
//! let res = future_opt.unwrap_or(10);
//! let final_res = res.await;
//! assert_eq!(final_res, 10);
//! # });
//! ```
//!
//! #### `unwrap_or_else`
//!
//! Returns this future's output if it is `Some`, otherwise calls the provided fallback function.
//!
//! This function is similar to the `Option::unwrap_or_else` where it will call the provided fallback function if the future resolves to `None`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! let future_opt = async { Some(4) };
//! let res = future_opt.unwrap_or_else(|| async { 10 });
//! let final_res = res.await;
//! assert_eq!(final_res, 4);
//!
//! let future_opt = async { None };
//! let res = future_opt.unwrap_or_else(|| async { 10 });
//! let final_res = res.await;
//! assert_eq!(final_res, 10);
//! # });
//! ```
//!
//! #### `merge`
//!
//! Merges this future with an optional value, producing a new future.
//!
//! This function takes an additional option and a function to combine the resolved value of the future and the option into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! async fn func(x: u32, y: u32) -> Option<u32> {
//!     Some(x + y)
//! }
//!
//! let x = async { Some(1) };
//! let y = Some(2);
//!
//! let res = x.merge(y, |var_x, var_y| func(var_x, var_y));
//! assert_eq!(res.await, Some(3));
//! # });
//! ```
//!
//! #### `merge2`
//!
//! Merges this future with two optional values, producing a new future.
//!
//! This function takes two additional options and a function to combine the resolved value of the future and the options into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! async fn func(x: u32, y: u32, z: u32) -> Option<u32> {
//!     Some(x + y + z)
//! }
//!
//! let x = async { Some(1) };
//! let y = Some(2);
//! let z = Some(3);
//!
//! let res = x.merge2(y, z, |var_x, var_y, var_z| func(var_x, var_y, var_z));
//! assert_eq!(res.await, Some(6));
//! # });
//! ```
//!
//! #### `merge3`
//!
//! Merges this future with three optional values, producing a new future.
//!
//! This function takes three additional options and a function to combine the resolved value of the future and the options into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! async fn func(x: u32, y: u32, z: u32, a: u32) -> Option<u32> {
//!     Some(x + y + z + a)
//! }
//!
//! let x = async { Some(1) };
//! let y = Some(2);
//! let z = Some(3);
//! let a = Some(4);
//!
//! let res = x.merge3(y, z, a, |var_x, var_y, var_z, var_a| func(var_x, var_y, var_z, var_a));
//! assert_eq!(res.await, Some(10));
//! # });
//! ```
//!
//! #### `merge4`
//!
//! Merges this future with four optional values, producing a new future.
//!
//! This function takes four additional options and a function to combine the resolved value of the future and the options into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureOptional;
//!
//! async fn func(x: u32, y: u32, z: u32, a: u32, b: u32) -> Option<u32> {
//!     Some(x + y + z + a + b)
//! }
//!
//! let x = async { Some(1) };
//! let y = Some(2);
//! let z = Some(3);
//! let a = Some(4);
//! let b = Some(5);
//!
//! let res = x.merge4(y, z, a, b, |var_x, var_y, var_z, var_a, var_b| func(var_x, var_y, var_z, var_a, var_b));
//! assert_eq!(res.await, Some(15));
//! # });
//! ```
//!
//! ### FutureResult
//!
//! An extension trait for `Future`s that yield `Result<T, E>` that provides a variety of convenient adapters.
//!
//! #### `map`
//!
//! Map this future's result output to a different type, returning a new future of the resulting type.
//!
//! This function is similar to the `Result::map` where it will change the type of the underlying future. This is useful to chain along a computation once a future has been resolved and if it is `Ok`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! let future_res = async { Ok::<_, ()>(1) };
//! let res = future_res.map(|t| async move { 5 });
//! let final_res = res.await;
//! assert_eq!(final_res, Ok(5));
//! # });
//! ```
//!
//! #### `map_or`
//!
//! Maps a `Result` by applying a function to the contained `Ok` value, or a default value if it is `Err`.
//!
//! This function is similar to the `Result::map_or`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! let future_res = async { Ok::<_, ()>(1) };
//! let res = future_res.map_or(10, |t| async move { t + 1 });
//! let final_res = res.await;
//! assert_eq!(final_res, 2);
//!
//! let future_res = async { Err::<i32, _>(()) };
//! let res = future_res.map_or(10, |t| async move { t + 1 });
//! let final_res = res.await;
//! assert_eq!(final_res, 10);
//! # });
//! ```
//!
//! #### `map_err`
//!
//! Maps a `Result` by applying a function to the contained `Err` value.
//!
//! This function is similar to the `Result::map_err`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! let future_res = async { Err::<u32, _>(1) };
//! let res = future_res.map_err(|e| async move { e + 1 });
//! let final_res = res.await;
//! assert_eq!(final_res, Err(2));
//! # });
//! ```
//!
//! #### `and_then`
//!
//! Chains this future with another future if the output is `Ok`, returning a new future of the resulting type.
//!
//! This function is similar to the `Result::and_then` where it will chain another computation if the future resolves to `Ok`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! let future_res = async { Ok::<_, ()>(1) };
//! let res = future_res.and_then(|t| async move { Ok(t + 1) });
//! let final_res = res.await;
//! assert_eq!(final_res, Ok(2));
//! # });
//! ```
//!
//! #### `or_else`
//!
//! Returns this future's result if it is `Ok`, otherwise calls the provided fallback function.
//!
//! This function is similar to the `Result::or_else` where it will call the provided fallback function if the future resolves to `Err`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! let future_res = async { Ok::<_, ()>(4) };
//! let res = future_res.or_else(|_| async { Ok(10) });
//! let final_res = res.await;
//! assert_eq!(final_res, Ok(4));
//!
//! let future_res = async { Err::<i32, _>(()) };
//! let res = future_res.or_else(|_| async { Ok(10) });
//! let final_res = res.await;
//! assert_eq!(final_res, Ok(10));
//! # });
//! ```
//!
//! #### `unwrap_or_else`
//!
//! Returns this future's result if it is `Ok`, otherwise calls the provided fallback function.
//!
//! This function is similar to the `Result::unwrap_or_else` where it will call the provided fallback function if the future resolves to `Err`.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! let future_res = async { Ok::<_, ()>(4) };
//! let res = future_res.unwrap_or_else(|_| async { 10 });
//! let final_res = res.await;
//! assert_eq!(final_res, 4);
//!
//! let future_res = async { Err::<i32, _>(()) };
//! let res = future_res.unwrap_or_else(|_| async { 10 });
//! let final_res = res.await;
//! assert_eq!(final_res, 10);
//! # });
//! ```
//!
//! #### `merge`
//!
//! Merges this future with a result value, producing a new future.
//!
//! This function takes an additional result and a function to combine the resolved value of the future and the result into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! async fn func(x: u32, y: u32) -> Result<u32, ()> {
//!     Ok(x + y)
//! }
//!
//! let x = async { Ok::<_, ()>(1) };
//! let y = Ok(2);
//!
//! let res = x.merge(y, |var_x, var_y| func(var_x, var_y));
//! assert_eq!(res.await, Ok(3));
//! # });
//! ```
//!
//! #### `merge2`
//!
//! Merges this future with two result values, producing a new future.
//!
//! This function takes two additional results and a function to combine the resolved value of the future and the results into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! async fn func(x: u32, y: u32, z: u32) -> Result<u32, ()> {
//!     Ok(x + y + z)
//! }
//!
//! let x = async { Ok::<_, ()>(1) };
//! let y = Ok(2);
//! let z = Ok(3);
//!
//! let res = x.merge2(y, z, |var_x, var_y, var_z| func(var_x, var_y, var_z));
//! assert_eq!(res.await, Ok(6));
//! # });
//! ```
//!
//! #### `merge3`
//!
//! Merges this future with three result values, producing a new future.
//!
//! This function takes three additional results and a function to combine the resolved value of the future and the results into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! async fn func(x: u32, y: u32, z: u32, a: u32) -> Result<u32, ()> {
//!     Ok(x + y + z + a)
//! }
//!
//! let x = async { Ok::<_, ()>(1) };
//! let y = Ok(2);
//! let z = Ok(3);
//! let a = Ok(4);
//!
//! let res = x.merge3(y, z, a, |var_x, var_y, var_z, var_a| func(var_x, var_y, var_z, var_a));
//! assert_eq!(res.await, Ok(10));
//! # });
//! ```
//!
//! #### `merge4`
//!
//! Merges this future with four result values, producing a new future.
//!
//! This function takes four additional results and a function to combine the resolved value of the future and the results into a new future.
//!
//! ##### Example
//!
//! ```rust
//! # #[cfg(feature = "rails-ext")]
//! # futures::executor::block_on(async {
//! use gearbox::rails::ext::future::FutureResult;
//!
//! async fn func(x: u32, y: u32, z: u32, a: u32, b: u32) -> Result<u32, ()> {
//!     Ok(x + y + z + a + b)
//! }
//!
//! let x = async { Ok::<_, ()>(1) };
//! let y = Ok(2);
//! let z = Ok(3);
//! let a = Ok(4);
//! let b = Ok(5);
//!
//! let res = x.merge4(y, z, a, b, |var_x, var_y, var_z, var_a, var_b| func(var_x, var_y, var_z, var_a, var_b));
//! assert_eq!(res.await, Ok(15));
//! # });
//! ```

pub mod ext;
pub mod future_ext;

pub(crate) mod private_utils;

use alloc::boxed::Box;
use core::{future::Future, pin::Pin};

pub use future_ext::*;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub(crate) fn assert_future<T, F>(future: F) -> F
where
    F: Future<Output = T>,
{
    future
}
