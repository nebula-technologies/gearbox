use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};

pub trait Tap
where
    Self: Sized,
{
    /// Immutable access to a value.
    ///
    /// This function permits a value to be viewed by some inspecting function
    /// without affecting the overall shape of the expression that contains this
    /// method call. It is useful for attaching assertions or logging points
    /// into a multi-part expression.
    ///
    /// # Examples
    ///
    /// Here we use `.tap()` to attach logging tracepoints to each stage of a
    /// value-processing pipeline.
    ///
    /// ```rust
    /// use gearbox::rails::ext::blocking::Tap;
    /// # struct Tmp;
    /// # impl Tmp { fn process_value(self) -> Self { self } }
    /// # fn make_value() -> Tmp { Tmp }
    /// # macro_rules! log { ($msg:literal, $x:ident) => {{}}; }
    ///
    /// let end = make_value()
    ///   // this line has no effect on the rest of the code
    ///   .tap(|v| log!("The produced value was: {}", v))
    ///   .process_value();
    /// ```
    #[inline(always)]
    fn tap(self, func: impl FnOnce(&Self)) -> Self {
        func(&self);
        self
    }

    /// Mutable access to a value.
    ///
    /// This function permits a value to be modified by some function without
    /// affecting the overall shape of the expression that contains this method
    /// call. It is useful for attaching modifier functions that have an
    /// `&mut Self -> ()` signature to an expression, without requiring an
    /// explicit `let mut` binding.
    ///
    /// # Examples
    ///
    /// Here we use `.tap_mut()` to sort an array without requring multiple
    /// bindings.
    ///
    /// ```rust
    /// use gearbox::rails::ext::blocking::Tap;
    ///
    /// let sorted = [1i32, 5, 2, 4, 3]
    ///   .tap_mut(|arr| arr.sort());
    /// assert_eq!(sorted, [1, 2, 3, 4, 5]);
    /// ```
    ///
    /// Without tapping, this would be written as
    ///
    /// ```rust
    /// let mut received = [1, 5, 2, 4, 3];
    /// received.sort();
    /// let sorted = received;
    /// ```
    ///
    /// The mutable tap is a convenient alternative when the expression to
    /// produce the collection is more complex, for example, an iterator
    /// pipeline collected into a vector.
    #[inline(always)]
    fn tap_mut(mut self, func: impl FnOnce(&mut Self)) -> Self {
        func(&mut self);
        self
    }

    /// Immutable access to the `Borrow<B>` of a value.
    ///
    /// This function is identcal to [`Tap::tap`], except that the effect
    /// function recevies an `&B` produced by `Borrow::<B>::borrow`, rather than
    /// an `&Self`.
    ///
    /// [`Tap::tap`]: trait.Tap.html#method.tap
    #[inline(always)]
    fn tap_borrow<B>(self, func: impl FnOnce(&B)) -> Self
    where
        Self: Borrow<B>,
        B: ?Sized,
    {
        func(Borrow::<B>::borrow(&self));
        self
    }

    /// Mutable access to the `BorrowMut<B>` of a value.
    ///
    /// This function is identical to [`Tap::tap_mut`], except that the effect
    /// function receives an `&mut B` produced by `BorrowMut::<B>::borrow_mut`,
    /// rather than an `&mut Self`.
    ///
    /// [`Tap::tap_mut`]: trait.Tap.html#method.tap_mut
    #[inline(always)]
    fn tap_borrow_mut<B>(mut self, func: impl FnOnce(&mut B)) -> Self
    where
        Self: BorrowMut<B>,
        B: ?Sized,
    {
        func(BorrowMut::<B>::borrow_mut(&mut self));
        self
    }

    /// Immutable access to the `AsRef<R>` view of a value.
    ///
    /// This function is identical to [`Tap::tap`], except that the effect
    /// function receives an `&R` produced by `AsRef::<R>::as_ref`, rather than
    /// an `&Self`.
    ///
    /// [`Tap::tap`]: trait.Tap.html#method.tap
    #[inline(always)]
    fn tap_ref<R>(self, func: impl FnOnce(&R)) -> Self
    where
        Self: AsRef<R>,
        R: ?Sized,
    {
        func(AsRef::<R>::as_ref(&self));
        self
    }

    /// Mutable access to the `AsMut<R>` view of a value.
    ///
    /// This function is identical to [`Tap::tap_mut`], except that the effect
    /// function receives an `&mut R` produced by `AsMut::<R>::as_mut`, rather
    /// than an `&mut Self`.
    ///
    /// [`Tap::tap_mut`]: trait.Tap.html#method.tap_mut
    #[inline(always)]
    fn tap_ref_mut<R>(mut self, func: impl FnOnce(&mut R)) -> Self
    where
        Self: AsMut<R>,
        R: ?Sized,
    {
        func(AsMut::<R>::as_mut(&mut self));
        self
    }

    /// Immutable access to the `Deref::Target` of a value.
    ///
    /// This function is identical to [`Tap::tap`], except that the effect
    /// function receives an `&Self::Target` produced by `Deref::deref`, rather
    /// than an `&Self`.
    ///
    /// [`Tap::tap`]: trait.Tap.html#method.tap
    #[inline(always)]
    fn tap_deref<T>(self, func: impl FnOnce(&T)) -> Self
    where
        Self: Deref<Target = T>,
        T: ?Sized,
    {
        func(Deref::deref(&self));
        self
    }

    /// Mutable access to the `Deref::Target` of a value.
    ///
    /// This function is identical to [`Tap::tap_mut`], except that the effect
    /// function receives an `&mut Self::Target` produced by
    /// `DerefMut::deref_mut`, rather than an `&mut Self`.
    ///
    /// [`Tap::tap_mut`]: trait.Tap.html#method.tap_mut
    #[inline(always)]
    fn tap_deref_mut<T>(mut self, func: impl FnOnce(&mut T)) -> Self
    where
        Self: DerefMut + Deref<Target = T>,
        T: ?Sized,
    {
        func(DerefMut::deref_mut(&mut self));
        self
    }

    //  debug-build-only copies of the above methods

    /// Calls `.tap()` only in debug builds, and is erased in release builds.
    #[inline(always)]
    fn tap_dbg(self, func: impl FnOnce(&Self)) -> Self {
        if cfg!(debug_assertions) {
            func(&self);
        }
        self
    }

    /// Calls `.tap_mut()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_mut_dbg(mut self, func: impl FnOnce(&mut Self)) -> Self {
        if cfg!(debug_assertions) {
            func(&mut self);
        }
        self
    }

    /// Calls `.tap_borrow()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_borrow_dbg<B>(self, func: impl FnOnce(&B)) -> Self
    where
        Self: Borrow<B>,
        B: ?Sized,
    {
        if cfg!(debug_assertions) {
            func(Borrow::<B>::borrow(&self));
        }
        self
    }

    /// Calls `.tap_borrow_mut()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_borrow_mut_dbg<B>(mut self, func: impl FnOnce(&mut B)) -> Self
    where
        Self: BorrowMut<B>,
        B: ?Sized,
    {
        if cfg!(debug_assertions) {
            func(BorrowMut::<B>::borrow_mut(&mut self));
        }
        self
    }

    /// Calls `.tap_ref()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_ref_dbg<R>(self, func: impl FnOnce(&R)) -> Self
    where
        Self: AsRef<R>,
        R: ?Sized,
    {
        if cfg!(debug_assertions) {
            func(AsRef::<R>::as_ref(&self));
        }
        self
    }

    /// Calls `.tap_ref_mut()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_ref_mut_dbg<R>(mut self, func: impl FnOnce(&mut R)) -> Self
    where
        Self: AsMut<R>,
        R: ?Sized,
    {
        if cfg!(debug_assertions) {
            func(AsMut::<R>::as_mut(&mut self));
        }
        self
    }

    /// Calls `.tap_deref()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_deref_dbg<T>(self, func: impl FnOnce(&T)) -> Self
    where
        Self: Deref<Target = T>,
        T: ?Sized,
    {
        if cfg!(debug_assertions) {
            func(Deref::deref(&self));
        }
        self
    }

    /// Calls `.tap_deref_mut()` only in debug builds, and is erased in release
    /// builds.
    #[inline(always)]
    fn tap_deref_mut_dbg<T>(mut self, func: impl FnOnce(&mut T)) -> Self
    where
        Self: DerefMut + Deref<Target = T>,
        T: ?Sized,
    {
        if cfg!(debug_assertions) {
            func(DerefMut::deref_mut(&mut self));
        }
        self
    }
}

impl<T> Tap for T where T: Sized {}

#[cfg(test)]
mod test_tap {
    use crate::rails::ext::blocking::Tap;

    #[test]
    fn test_tap() {
        let res = "hello".to_string();

        res.tap(|t| assert_eq!(t, &"hello".to_string()));
    }

    #[test]
    fn test_tap_mut() {
        let res = "hello".to_string();

        assert_eq!(
            res.tap_mut(|t| *t = "world".to_string()),
            "world".to_string()
        );
    }
}
