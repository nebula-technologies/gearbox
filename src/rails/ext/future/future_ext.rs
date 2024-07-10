use super::assert_future;
use super::ext::{option, result};
use crate::rails::ext::future::private_utils::FutureContainer;
use core::future::Future;

impl<T> IntoFutureOptional<T> for Option<T> {
    fn into_future(self) -> FutureContainer<Option<T>> {
        FutureContainer::new_from_opt(self)
    }
}
pub trait IntoFutureOptional<T> {
    fn into_future(self) -> FutureContainer<Option<T>>;
}

impl<T, U> FutureOptional<T> for U where U: Future<Output = Option<T>> {}

/// An extension trait for `Future`s that yield `Option<T>` that provides a variety
/// of convenient adapters.
pub trait FutureOptional<T>: Future<Output = Option<T>> {
    /// Map this future's optional output to a different type, returning a new future of
    /// the resulting type.
    ///
    /// This function is similar to the `Option::map` where it will change the type of the
    /// underlying future. This is useful to chain along a computation once a future has been
    /// resolved and if it is `Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(1) };
    /// let res = future_opt.map(|t| async move { 5 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(5));
    /// # });
    /// ```
    fn map<U, F, F2>(self, f: F) -> option::Map<Self, F, F2>
    where
        F: FnOnce(T) -> F2,
        F2: Future<Output = U>,
        Self: Sized,
    {
        assert_future(option::Map::new(self, f.into()))
    }

    /// Chains this future with another future if the output is `Some`, returning a new future of
    /// the resulting type.
    ///
    /// This function is similar to the `Option::and_then` where it will chain another computation
    /// if the future resolves to `Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(1) };
    /// let res = future_opt.and_then(|t| async move { Some(t + 1) });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(2));
    /// # });
    /// ```
    fn and_then<U, F, F2>(self, f: F) -> option::AndThen<Self, F, F2>
    where
        F: FnOnce(T) -> F2,
        F2: Future<Output = Option<U>>,
        Self: Sized,
    {
        assert_future(option::AndThen::new(self, f))
    }

    /// Filters the output of this future, returning `None` if the predicate returns `false`.
    ///
    /// This function is similar to the `Option::filter` where it will return `None` if the predicate
    /// returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(4) };
    /// let res = future_opt.filter(|x| *x > 2);
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(4));
    /// # });
    /// ```
    fn filter<F>(self, f: F) -> option::Filter<Self, F>
    where
        F: FnOnce(&T) -> bool,
        Self: Sized,
    {
        assert_future(option::Filter::new(self, f))
    }

    /// Returns this future's output if it is `Some`, otherwise returns the provided fallback.
    ///
    /// This function is similar to the `Option::or` where it will return the provided fallback
    /// if the future resolves to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(4) };
    /// let res = future_opt.or(Some(10));
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(4));
    ///
    /// let future_opt = async { None };
    /// let res = future_opt.or(Some(10));
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(10));
    /// # });
    /// ```
    fn or(self, other: Option<T>) -> option::Or<Self, T>
    where
        Self: Sized,
    {
        assert_future(option::Or::new(self, other))
    }

    /// Returns this future's output if it is `Some`, otherwise calls the provided fallback function.
    ///
    /// This function is similar to the `Option::or_else` where it will call the provided fallback
    /// function if the future resolves to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(4) };
    /// let res = future_opt.or_else(|| async { Some(10) });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(4));
    ///
    /// let future_opt = async { None };
    /// let res = future_opt.or_else(|| async { Some(10) });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Some(10));
    /// # });
    /// ```
    fn or_else<F, F2>(self, f: F) -> option::OrElse<Self, F, F2>
    where
        F: FnOnce() -> F2,
        F2: Future<Output = Option<T>>,
        Self: Sized,
    {
        assert_future(option::OrElse::new(self, f))
    }

    /// Returns this future's output if it is `Some`, otherwise returns the provided default.
    ///
    /// This function is similar to the `Option::unwrap_or` where it will return the provided default
    /// if the future resolves to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(4) };
    /// let res = future_opt.unwrap_or(10);
    /// let final_res = res.await;
    /// assert_eq!(final_res, 4);
    ///
    /// let future_opt = async { None };
    /// let res = future_opt.unwrap_or(10);
    /// let final_res = res.await;
    /// assert_eq!(final_res, 10);
    /// # });
    /// ```
    fn unwrap_or(self, default: T) -> option::UnwrapOr<Self, T>
    where
        Self: Sized,
    {
        assert_future(option::UnwrapOr::new(self, default))
    }

    /// Returns this future's output if it is `Some`, otherwise calls the provided fallback function.
    ///
    /// This function is similar to the `Option::unwrap_or_else` where it will call the provided fallback
    /// function if the future resolves to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// let future_opt = async { Some(4) };
    /// let res = future_opt.unwrap_or_else(|| async { 10 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, 4);
    ///
    /// let future_opt = async { None };
    /// let res = future_opt.unwrap_or_else(|| async { 10 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, 10);
    /// # });
    /// ```
    fn unwrap_or_else<F, F2>(self, f: F) -> option::UnwrapOrElse<Self, F, F2>
    where
        F: FnOnce() -> F2,
        F2: Future<Output = T>,
        Self: Sized,
    {
        assert_future(option::UnwrapOrElse::new(self, f))
    }

    /// Merges this future with an optional value, producing a new future.
    ///
    /// This function takes an additional option and a function to combine the resolved value of the
    /// future and the option into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// async fn func(x: u32, y: u32) -> Option<u32> {
    ///     Some(x + y)
    /// }
    ///
    /// let x = async { Some(1) };
    /// let y = Some(2);
    ///
    /// let res = x.merge(y, |var_x, var_y| func(var_x, var_y));
    /// assert_eq!(res.await, Some(3));
    /// # });
    /// ```
    fn merge<T1, U, F, F2>(self, res1: Option<T1>, op: F) -> option::Merge<Self, T1, F, F2>
    where
        F: FnOnce(T, T1) -> F2,
        F2: Future<Output = Option<U>>,
        Self: Sized,
    {
        assert_future(option::Merge::new(self, op, res1))
    }

    /// Merges this future with two optional values, producing a new future.
    ///
    /// This function takes two additional options and a function to combine the resolved value of the
    /// future and the options into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// async fn func(x: u32, y: u32, z: u32) -> Option<u32> {
    ///     Some(x + y + z)
    /// }
    ///
    /// let x = async { Some(1) };
    /// let y = Some(2);
    /// let z = Some(3);
    ///
    /// let res = x.merge2(y, z, |var_x, var_y, var_z| func(var_x, var_y, var_z));
    /// assert_eq!(res.await, Some(6));
    /// # });
    /// ```
    fn merge2<T1, T2, U, F, F2>(
        self,
        res1: Option<T1>,
        res2: Option<T2>,
        op: F,
    ) -> option::Merge2<Self, T1, T2, F, F2>
    where
        F: FnOnce(T, T1, T2) -> F2,
        F2: Future<Output = Option<U>>,
        Self: Sized,
    {
        assert_future(option::Merge2::new(self, op, res1, res2))
    }

    /// Merges this future with three optional values, producing a new future.
    ///
    /// This function takes three additional options and a function to combine the resolved value of the
    /// future and the options into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// async fn func(x: u32, y: u32, z: u32, a: u32) -> Option<u32> {
    ///     Some(x + y + z + a)
    /// }
    ///
    /// let x = async { Some(1) };
    /// let y = Some(2);
    /// let z = Some(3);
    /// let a = Some(4);
    ///
    /// let res = x.merge3(y, z, a, |var_x, var_y, var_z, var_a| func(var_x, var_y, var_z, var_a));
    /// assert_eq!(res.await, Some(10));
    /// # });
    /// ```
    fn merge3<T1, T2, T3, U, F, F2>(
        self,
        res1: Option<T1>,
        res2: Option<T2>,
        res3: Option<T3>,
        op: F,
    ) -> option::Merge3<Self, T1, T2, T3, F, F2>
    where
        F: FnOnce(T, T1, T2, T3) -> F2,
        F2: Future<Output = Option<U>>,
        Self: Sized,
    {
        assert_future(option::Merge3::new(self, op, res1, res2, res3))
    }

    /// Merges this future with four optional values, producing a new future.
    ///
    /// This function takes four additional options and a function to combine the resolved value of the
    /// future and the options into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureOptional;
    ///
    /// async fn func(x: u32, y: u32, z: u32, a: u32, b: u32) -> Option<u32> {
    ///     Some(x + y + z + a + b)
    /// }
    ///
    /// let x = async { Some(1) };
    /// let y = Some(2);
    /// let z = Some(3);
    /// let a = Some(4);
    /// let b = Some(5);
    ///
    /// let res = x.merge4(y, z, a, b, |var_x, var_y, var_z, var_a, var_b| func(var_x, var_y, var_z, var_a, var_b));
    /// assert_eq!(res.await, Some(15));
    /// # });
    /// ```
    fn merge4<T1, T2, T3, T4, U, F, F2>(
        self,
        res1: Option<T1>,
        res2: Option<T2>,
        res3: Option<T3>,
        res4: Option<T4>,
        op: F,
    ) -> option::Merge4<Self, T1, T2, T3, T4, F, F2>
    where
        F: FnOnce(T, T1, T2, T3, T4) -> F2,
        F2: Future<Output = Option<U>>,
        Self: Sized,
    {
        assert_future(option::Merge4::new(self, op, res1, res2, res3, res4))
    }
}

#[cfg(test)]
pub mod tests_future_optional {
    use crate::rails::ext::blocking::Merge;

    #[tokio::test]
    async fn test_map_optional() {
        use super::FutureOptional;
        let future_opt = async { Some(1) };
        let res = future_opt.map(|_t| async move { 5 });
        let final_res = res.await;
        assert_eq!(final_res, Some(5));
    }

    #[tokio::test]
    async fn test_and_then_optional() {
        use super::FutureOptional;
        let future_opt = async { Some(1) };
        let res = future_opt
            .and_then(|_t| async move { None as Option<i32> })
            .map(|_t| async move { 5 });
        let final_res = res.await;
        assert_eq!(final_res, None);

        let future_opt = async { Some(1) };
        let res = future_opt.and_then(|_t| async move { None as Option<i32> });
        let final_res = res.await;
        assert_eq!(final_res, None);
    }

    #[tokio::test]
    async fn test_filter() {
        use super::FutureOptional;
        let future_opt = async { Some(4) };
        let res = future_opt.filter(|x| *x > 2);
        let final_res = res.await;
        assert_eq!(final_res, Some(4));

        let future_opt = async { Some(1) };
        let res = future_opt.filter(|x| *x > 2);
        let final_res = res.await;
        assert_eq!(final_res, None);
    }

    #[tokio::test]
    async fn test_or() {
        use super::FutureOptional;
        let future_opt = async { Some(4) };
        let res = future_opt.or(Some(10));
        let final_res = res.await;
        assert_eq!(final_res, Some(4));

        let future_opt = async { None };
        let res = future_opt.or(Some(10));
        let final_res = res.await;
        assert_eq!(final_res, Some(10));
    }

    #[tokio::test]
    async fn test_or_else() {
        use super::FutureOptional;
        let future_opt = async { Some(4) };
        let res = future_opt.or_else(|| async { Some(10) });
        let final_res = res.await;
        assert_eq!(final_res, Some(4));

        let future_opt = async { None };
        let res = future_opt.or_else(|| async { Some(10) });
        let final_res = res.await;
        assert_eq!(final_res, Some(10));
    }

    #[tokio::test]
    async fn test_unwrap_or() {
        use super::FutureOptional;
        let future_opt = async { Some(4) };
        let res = future_opt.unwrap_or(10);
        let final_res = res.await;
        assert_eq!(final_res, 4);

        let future_opt = async { None };
        let res = future_opt.unwrap_or(10);
        let final_res = res.await;
        assert_eq!(final_res, 10);
    }

    #[tokio::test]
    async fn test_unwrap_or_else() {
        use super::FutureOptional;
        let future_opt = async { Some(4) };
        let res = future_opt.unwrap_or_else(|| async { 10 });
        let final_res = res.await;
        assert_eq!(final_res, 4);

        let future_opt = async { None };
        let res = future_opt.unwrap_or_else(|| async { 10 });
        let final_res = res.await;
        assert_eq!(final_res, 10);
    }

    #[tokio::test]
    async fn test_merge() {
        use super::FutureOptional;

        async fn func_xy(x: u32, y: u32) -> Option<u32> {
            Some(x + y)
        }

        // Case 1: Both the future and the option are Some
        let x = async { Some(1) };
        let y = Some(2);

        let res = x.merge(y, |var_x, var_y| func_xy(var_x, var_y));

        assert_eq!(res.await, Some(3));

        // Case 2: The initial future is None
        let x = async { None };
        let y = Some(2);

        let res = x.merge(y, |var_x, var_y| func_xy(var_x, var_y));

        assert_eq!(res.await, None);

        // Case 3: The option is None
        let x = async { Some(1) };
        let y = None;

        let res = x.merge(y, |var_x, var_y| func_xy(var_x, var_y));

        assert_eq!(res.await, None);

        // Case 4: The function returns None
        async fn func_xy_none(_: u32, _: u32) -> Option<u32> {
            None
        }

        let x = async { Some(1) };
        let y = Some(2);

        let res = x.merge(y, |var_x, var_y| func_xy_none(var_x, var_y));

        assert_eq!(res.await, None);
    }
    #[tokio::test]
    async fn test_merge2() {
        use super::FutureOptional;

        async fn func_xyz(x: u32, y: u32, z: u32) -> Option<u32> {
            Some(x + y + z)
        }

        // Case 1: All futures and options are Some
        let x = async { Some(1) };
        let y = Some(2);
        let z = Some(3);

        let res = x.merge2(y, z, |var_x, var_y, var_z| func_xyz(var_x, var_y, var_z));

        assert_eq!(res.await, Some(6));

        // Case 2: The initial future is None
        let x = async { None };
        let y = Some(2);
        let z = Some(3);

        let res = x.merge2(y, z, |var_x, var_y, var_z| func_xyz(var_x, var_y, var_z));

        assert_eq!(res.await, None);

        // Case 3: One of the options is None
        let x = async { Some(1) };
        let y = None;
        let z = Some(3);

        let res = x.merge2(y, z, |var_x, var_y, var_z| func_xyz(var_x, var_y, var_z));

        assert_eq!(res.await, None);

        // Case 4: Multiple options are None
        let x = async { Some(1) };
        let y = None;
        let z = None;

        let res = x.merge2(y, z, |var_x, var_y, var_z| func_xyz(var_x, var_y, var_z));

        assert_eq!(res.await, None);

        // Case 5: The function returns None
        async fn func_xyz_none(_: u32, _: u32, _: u32) -> Option<u32> {
            None
        }

        let x = async { Some(1) };
        let y = Some(2);
        let z = Some(3);

        let res = x.merge2(y, z, |var_x, var_y, var_z| {
            func_xyz_none(var_x, var_y, var_z)
        });

        assert_eq!(res.await, None);
    }

    #[tokio::test]
    async fn test_merge3() {
        use super::FutureOptional;

        async fn func_xyz(v: u32, w: u32, x: u32, y: u32) -> Option<u32> {
            Some(v + w + x + y)
        }

        // Case 1: All futures and options are Some
        let v = async { Some(1) };
        let w = Some(2);
        let x = Some(3);
        let y = Some(4);

        let res = v.merge3(w, x, y, |var_v, var_w, var_x, var_y| {
            func_xyz(var_v, var_w, var_x, var_y)
        });

        assert_eq!(res.await, Some(10));

        // Case 2: The initial future is None
        let v = async { None };
        let w = Some(2);
        let x = Some(3);
        let y = Some(4);

        let res = v.merge3(w, x, y, |var_v, var_w, var_x, var_y| {
            func_xyz(var_v, var_w, var_x, var_y)
        });

        assert_eq!(res.await, None);

        // Case 3: One of the options is None
        let v = async { Some(1) };
        let w = None;
        let x = Some(3);
        let y = Some(4);

        let res = v.merge3(w, x, y, |var_v, var_w, var_x, var_y| {
            func_xyz(var_v, var_w, var_x, var_y)
        });

        assert_eq!(res.await, None);

        // Case 4: Multiple options are None
        let v = async { Some(1) };
        let w = None;
        let x = None;
        let y = Some(4);

        let res = v.merge3(w, x, y, |var_v, var_w, var_x, var_y| {
            func_xyz(var_v, var_w, var_x, var_y)
        });

        assert_eq!(res.await, None);

        // Case 5: The function returns None
        async fn func_xyz_none(_: u32, _: u32, _: u32, _: u32) -> Option<u32> {
            None
        }

        let v = async { Some(1) };
        let w = Some(2);
        let x = Some(3);
        let y = Some(4);

        let res = v.merge3(w, x, y, |var_v, var_w, var_x, var_y| {
            func_xyz_none(var_v, var_w, var_x, var_y)
        });

        assert_eq!(res.await, None);
    }

    #[tokio::test]
    async fn test_merge4() {
        use super::FutureOptional;

        async fn func_xyz(v: u32, w: u32, x: u32, y: u32, z: u32) -> Option<u32> {
            Some(v + w + x + y + z)
        }

        // Case 1: All futures and options are Some
        let v = async { Some(1) };
        let w = Some(2);
        let x = Some(3);
        let y = Some(4);
        let z = Some(5);

        let res = v.merge4(w, x, y, z, |var_v, var_w, var_x, var_y, var_z| {
            func_xyz(var_v, var_w, var_x, var_y, var_z)
        });

        assert_eq!(res.await, Some(15));

        // Case 2: The initial future is None
        let v = async { None };
        let w = Some(2);
        let x = Some(3);
        let y = Some(4);
        let z = Some(5);

        let res = v.merge4(w, x, y, z, |var_v, var_w, var_x, var_y, var_z| {
            func_xyz(var_v, var_w, var_x, var_y, var_z)
        });

        assert_eq!(res.await, None);

        // Case 3: One of the options is None
        let v = async { Some(1) };
        let w = None;
        let x = Some(3);
        let y = Some(4);
        let z = Some(5);

        let res = v.merge4(w, x, y, z, |var_v, var_w, var_x, var_y, var_z| {
            func_xyz(var_v, var_w, var_x, var_y, var_z)
        });

        assert_eq!(res.await, None);

        // Case 4: Multiple options are None
        let v = async { Some(1) };
        let w = None;
        let x = None;
        let y = Some(4);
        let z = Some(5);

        let res = v.merge4(w, x, y, z, |var_v, var_w, var_x, var_y, var_z| {
            func_xyz(var_v, var_w, var_x, var_y, var_z)
        });

        assert_eq!(res.await, None);

        // Case 5: The function returns None
        async fn func_xyz_none(_: u32, _: u32, _: u32, _: u32, _: u32) -> Option<u32> {
            None
        }

        let v = async { Some(1) };
        let w = Some(2);
        let x = Some(3);
        let y = Some(4);
        let z = Some(5);

        let res = v.merge4(w, x, y, z, |var_v, var_w, var_x, var_y, var_z| {
            func_xyz_none(var_v, var_w, var_x, var_y, var_z)
        });

        assert_eq!(res.await, None);
    }
}

impl<T, E> IntoFutureResult<T, E> for Result<T, E> {
    fn into_future(self) -> FutureContainer<Result<T, E>> {
        FutureContainer::new_from_res(self)
    }
}
pub trait IntoFutureResult<T, E> {
    fn into_future(self) -> FutureContainer<Result<T, E>>;
}

impl<T, E, U> FutureResult<T, E> for U where U: Future<Output = Result<T, E>> {}

/// An extension trait for `Future`s that yield `Result<T, E>` that provides a variety
/// of convenient adapters.
pub trait FutureResult<T, E>: Future<Output = Result<T, E>> {
    /// Map this future's result output to a different type, returning a new future of
    /// the resulting type.
    ///
    /// This function is similar to the `Result::map` where it will change the type of the
    /// underlying future. This is useful to chain along a computation once a future has been
    /// resolved and if it is `Ok`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// let future_res = async { Ok::<_, ()>(1) };
    /// let res = future_res.map(|t| async move { 5 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Ok(5));
    /// # });
    /// ```
    fn map<U, F, F2>(self, f: F) -> result::Map<Self, F, F2>
    where
        F: FnOnce(T) -> F2,
        F2: Future<Output = U>,
        Self: Sized,
    {
        assert_future(result::Map::new(self, f.into()))
    }

    /// Maps a `Result` by applying a function to the contained `Ok` value, or a default value if it is `Err`.
    ///
    /// This function is similar to the `Result::map_or`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// let future_res = async { Ok::<_, ()>(1) };
    /// let res = future_res.map_or(10, |t| async move { t + 1 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, 2);
    ///
    /// let future_res = async { Err::<i32, _>(()) };
    /// let res = future_res.map_or(10, |t| async move { t + 1 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, 10);
    /// # });
    /// ```
    fn map_or<U, F, F2>(self, default: U, f: F) -> result::MapOr<Self, U, F, F2>
    where
        F: FnOnce(T) -> F2,
        F2: Future<Output = U>,
        Self: Sized,
    {
        assert_future(result::MapOr::new(self, default, f.into()))
    }

    /// Maps a `Result` by applying a function to the contained `Err` value.
    ///
    /// This function is similar to the `Result::map_err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// let future_res = async { Err::<u32, _>(1) };
    /// let res = future_res.map_err(|e| async move { e + 1 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Err(2));
    /// # });
    /// ```
    fn map_err<F, F2, U>(self, f: F) -> result::MapErr<Self, F, F2>
    where
        F: FnOnce(E) -> F2,
        F2: Future<Output = U>,
        Self: Sized,
    {
        assert_future(result::MapErr::new(self, f.into()))
    }

    /// Chains this future with another future if the output is `Ok`, returning a new future of
    /// the resulting type.
    ///
    /// This function is similar to the `Result::and_then` where it will chain another computation
    /// if the future resolves to `Ok`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// let future_res = async { Ok::<_, ()>(1) };
    /// let res = future_res.and_then(|t| async move { Ok(t + 1) });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Ok(2));
    /// # });
    /// ```
    fn and_then<U, F, F2>(self, f: F) -> result::AndThen<Self, F, F2>
    where
        F: FnOnce(T) -> F2,
        F2: Future<Output = Result<U, E>>,
        Self: Sized,
    {
        assert_future(result::AndThen::new(self, f))
    }

    /// Returns this future's result if it is `Ok`, otherwise calls the provided fallback function.
    ///
    /// This function is similar to the `Result::or_else` where it will call the provided fallback
    /// function if the future resolves to `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// let future_res = async { Ok::<_, ()>(4) };
    /// let res = future_res.or_else(|_| async { Ok(10) });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Ok(4));
    ///
    /// let future_res = async { Err::<i32, _>(()) };
    /// let res = future_res.or_else(|_| async { Ok(10) });
    /// let final_res = res.await;
    /// assert_eq!(final_res, Ok(10));
    /// # });
    /// ```
    fn or_else<F, F2>(self, f: F) -> result::OrElse<Self, F, F2>
    where
        F: FnOnce(E) -> F2,
        F2: Future<Output = Result<T, E>>,
        Self: Sized,
    {
        assert_future(result::OrElse::new(self, f))
    }

    /// Returns this future's result if it is `Ok`, otherwise calls the provided fallback function.
    ///
    /// This function is similar to the `Result::unwrap_or_else` where it will call the provided fallback
    /// function if the future resolves to `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// let future_res = async { Ok::<_, ()>(4) };
    /// let res = future_res.unwrap_or_else(|_| async { 10 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, 4);
    ///
    /// let future_res = async { Err::<i32, _>(()) };
    /// let res = future_res.unwrap_or_else(|_| async { 10 });
    /// let final_res = res.await;
    /// assert_eq!(final_res, 10);
    /// # });
    /// ```
    fn unwrap_or_else<F, F2>(self, f: F) -> result::UnwrapOrElse<Self, F, F2>
    where
        F: FnOnce(E) -> F2,
        F2: Future<Output = T>,
        Self: Sized,
    {
        assert_future(result::UnwrapOrElse::new(self, f))
    }

    /// Merges this future with a result value, producing a new future.
    ///
    /// This function takes an additional result and a function to combine the resolved value of the
    /// future and the result into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// async fn func(x: u32, y: u32) -> Result<u32, ()> {
    ///     Ok(x + y)
    /// }
    ///
    /// let x = async { Ok::<_, ()>(1) };
    /// let y = Ok(2);
    ///
    /// let res = x.merge(y, |var_x, var_y| func(var_x, var_y));
    /// assert_eq!(res.await, Ok(3));
    /// # });
    /// ```
    fn merge<T1, U, F, F2>(self, res1: Result<T1, E>, op: F) -> result::Merge<Self, T1, E, F, F2>
    where
        F: FnOnce(T, T1) -> F2,
        F2: Future<Output = Result<U, E>>,
        Self: Sized,
    {
        assert_future(result::Merge::new(self, op, res1))
    }

    /// Merges this future with two result values, producing a new future.
    ///
    /// This function takes two additional results and a function to combine the resolved value of the
    /// future and the results into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// async fn func(x: u32, y: u32, z: u32) -> Result<u32, ()> {
    ///     Ok(x + y + z)
    /// }
    ///
    /// let x = async { Ok::<_, ()>(1) };
    /// let y = Ok(2);
    /// let z = Ok(3);
    ///
    /// let res = x.merge2(y, z, |var_x, var_y, var_z| func(var_x, var_y, var_z));
    /// assert_eq!(res.await, Ok(6));
    /// # });
    /// ```
    fn merge2<T1, T2, U, F, F2>(
        self,
        res1: Result<T1, E>,
        res2: Result<T2, E>,
        op: F,
    ) -> result::Merge2<Self, T1, T2, E, F, F2>
    where
        F: FnOnce(T, T1, T2) -> F2,
        F2: Future<Output = Result<U, E>>,
        Self: Sized,
    {
        assert_future(result::Merge2::new(self, op, res1, res2))
    }

    /// Merges this future with three result values, producing a new future.
    ///
    /// This function takes three additional results and a function to combine the resolved value of the
    /// future and the results into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// async fn func(x: u32, y: u32, z: u32, a: u32) -> Result<u32, ()> {
    ///     Ok(x + y + z + a)
    /// }
    ///
    /// let x = async { Ok::<_, ()>(1) };
    /// let y = Ok(2);
    /// let z = Ok(3);
    /// let a = Ok(4);
    ///
    /// let res = x.merge3(y, z, a, |var_x, var_y, var_z, var_a| func(var_x, var_y, var_z, var_a));
    /// assert_eq!(res.await, Ok(10));
    /// # });
    /// ```
    fn merge3<T1, T2, T3, U, F, F2>(
        self,
        res1: Result<T1, E>,
        res2: Result<T2, E>,
        res3: Result<T3, E>,
        op: F,
    ) -> result::Merge3<Self, T1, T2, T3, E, F, F2>
    where
        F: FnOnce(T, T1, T2, T3) -> F2,
        F2: Future<Output = Result<U, E>>,
        Self: Sized,
    {
        assert_future(result::Merge3::new(self, op, res1, res2, res3))
    }

    /// Merges this future with four result values, producing a new future.
    ///
    /// This function takes four additional results and a function to combine the resolved value of the
    /// future and the results into a new future.
    ///
    /// # Examples
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// use gearbox::rails::ext::future::FutureResult;
    ///
    /// async fn func(x: u32, y: u32, z: u32, a: u32, b: u32) -> Result<u32, ()> {
    ///     Ok(x + y + z + a + b)
    /// }
    ///
    /// let x = async { Ok::<_, ()>(1) };
    /// let y = Ok(2);
    /// let z = Ok(3);
    /// let a = Ok(4);
    /// let b = Ok(5);
    ///
    /// let res = x.merge4(y, z, a, b, |var_x, var_y, var_z, var_a, var_b| func(var_x, var_y, var_z, var_a, var_b));
    /// assert_eq!(res.await, Ok(15));
    /// # });
    /// ```
    fn merge4<T1, T2, T3, T4, U, F, F2>(
        self,
        res1: Result<T1, E>,
        res2: Result<T2, E>,
        res3: Result<T3, E>,
        res4: Result<T4, E>,
        op: F,
    ) -> result::Merge4<Self, T1, T2, T3, T4, E, F, F2>
    where
        F: FnOnce(T, T1, T2, T3, T4) -> F2,
        F2: Future<Output = Result<U, E>>,
        Self: Sized,
    {
        assert_future(result::Merge4::new(self, op, res1, res2, res3, res4))
    }
}
#[cfg(test)]
mod tests_future_result {
    use super::FutureResult;
    use crate::rails::ext::future::private_utils::FutureContainer;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_map_result() {
        let future_res = async { Ok::<_, ()>(1) };
        let res = future_res.map(|t| async move { t + 1 });
        let final_res = res.await;
        assert_eq!(final_res, Ok(2));
    }

    #[tokio::test]
    async fn test_map_or_result() {
        let future_res = async { Ok::<_, ()>(1) };
        let res = future_res.map_or(10, |t| async move { t + 1 });
        let final_res = res.await;
        assert_eq!(final_res, 2);

        let future_res = async { Err::<i32, _>(()) };
        let res = future_res.map_or(10, |t| async move { t + 1 });
        let final_res = res.await;
        assert_eq!(final_res, 10);
    }

    #[tokio::test]
    async fn test_map_err_result() {
        let future_res = async { Err::<u32, _>(1) };
        let res = future_res.map_err(|e| async move { e + 1 });
        let final_res = res.await;
        assert_eq!(final_res, Err(2));
    }

    #[tokio::test]
    async fn test_and_then_result() {
        let future_res = async { Ok::<_, ()>(1) };
        let res = future_res.and_then(|t| async move { Ok(t + 1) });
        let final_res = res.await;
        assert_eq!(final_res, Ok(2));
    }

    #[tokio::test]
    async fn test_or_else_result() {
        let future_res = async { Ok::<_, ()>(4) };
        let res = future_res.or_else(|_| async { Ok(10) });
        let final_res = res.await;
        assert_eq!(final_res, Ok(4));

        let future_res = async { Err::<i32, _>(()) };
        let res = future_res.or_else(|_| async { Ok(10) });
        let final_res = res.await;
        assert_eq!(final_res, Ok(10));
    }

    #[tokio::test]
    async fn test_unwrap_or_else_result() {
        let future_res = async { Ok::<_, ()>(4) };
        let res = future_res.unwrap_or_else(|_| async { 10 });
        let final_res = res.await;
        assert_eq!(final_res, 4);

        let future_res = async { Err::<i32, _>(()) };
        let res = future_res.unwrap_or_else(|_| async { 10 });
        let final_res = res.await;
        assert_eq!(final_res, 10);
    }

    #[tokio::test]
    async fn test_merge_result() {
        async fn func_xy(x: u32, y: u32) -> Result<u32, ()> {
            Ok(x + y)
        }

        let x = async { Ok::<_, ()>(1) };
        let y = Ok(2);

        let res = x.merge(y, |var_x, var_y| func_xy(var_x, var_y));
        assert_eq!(res.await, Ok(3));
    }

    #[tokio::test]
    async fn test_merge2_result() {
        async fn func_xyz(x: u32, y: u32, z: u32) -> Result<u32, ()> {
            Ok(x + y + z)
        }

        let x = async { Ok::<_, ()>(1) };
        let y = Ok(2);
        let z = Ok(3);

        let res = x.merge2(y, z, |var_x, var_y, var_z| func_xyz(var_x, var_y, var_z));
        assert_eq!(res.await, Ok(6));
    }

    #[tokio::test]
    async fn test_merge3_result() {
        async fn func_xyz(v: u32, w: u32, x: u32, y: u32) -> Result<u32, ()> {
            Ok(v + w + x + y)
        }

        let v = async { Ok::<_, ()>(1) };
        let w = Ok(2);
        let x = Ok(3);
        let y = Ok(4);

        let res = v.merge3(w, x, y, |var_v, var_w, var_x, var_y| {
            func_xyz(var_v, var_w, var_x, var_y)
        });
        assert_eq!(res.await, Ok(10));
    }

    #[tokio::test]
    async fn test_merge4_result() {
        async fn func_xyz(v: u32, w: u32, x: u32, y: u32, z: u32) -> Result<u32, ()> {
            Ok(v + w + x + y + z)
        }

        let v = async { Ok::<_, ()>(1) };
        let w = Ok(2);
        let x = Ok(3);
        let y = Ok(4);
        let z = Ok(5);

        let res = v.merge4(w, x, y, z, |var_v, var_w, var_x, var_y, var_z| {
            func_xyz(var_v, var_w, var_x, var_y, var_z)
        });
        assert_eq!(res.await, Ok(15));
    }
}
