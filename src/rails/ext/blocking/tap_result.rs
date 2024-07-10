pub trait TapResult<T, E> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self;
    fn tap_mut<F: FnOnce(&mut T)>(self, op: F) -> Self;
    fn tap_err<F: FnOnce(&E)>(self, op: F) -> Self;
    fn tap_err_mut<F: FnOnce(&mut E)>(self, op: F) -> Self;
}

/// # impl of TapRef [`Result<T,E>`]
/// This allows tapping into the result object and interact with a reference of the internal data.
impl<T, E> TapResult<T, E> for Result<T, E> {
    /// # tap_ref
    ///
    /// tap mut gives an immutable reference of the underlying data.
    /// this can often be used as way to log or read data in a cleaner fashion then using a map
    /// where you will hate to return the data anyways even if nothing has been modified.
    /// Taps does not rely on a return value.
    /// ```
    /// use gearbox::rails::ext::blocking::TapResult;
    ///
    /// let res: Result<_,()> = Ok("hello".to_string());
    /// res.tap(|t| assert_eq!(t, &"hello".to_string())).ok();
    /// ```
    #[inline]
    fn tap<F: FnOnce(&T)>(self, op: F) -> Result<T, E> {
        match &self {
            Ok(t) => op(t),
            _ => {}
        }
        self
    }

    /// # tap_mut
    ///
    /// This allows for modifying the data that are recieved through tap.
    /// Normally map will to fine in this instance, though this allows for modifying the data
    /// behind the reference.
    /// The difference between map and tap_mut is that it operates directly on the reference and
    /// that the datatype is not allowed to change.
    /// ```
    /// use gearbox::rails::ext::blocking::TapResult;
    ///
    /// let mut res: Result<_,()> = Ok("hello".to_string());
    ///
    /// assert_eq!(res.tap_mut(|t| *t = "world".to_string()).unwrap(), "world".to_string());
    /// ```
    #[inline]
    fn tap_mut<F: FnOnce(&mut T)>(mut self, op: F) -> Self {
        match &mut self {
            Ok(t) => op(t),
            _ => {}
        }
        self
    }

    /// # tap_ref
    ///
    /// tap mut gives an immutable reference of the underlying data.
    /// this can often be used as way to log or read data in a cleaner fashion then using a map
    /// where you will hate to return the data anyways even if nothing has been modified.
    /// Taps does not rely on a return value.
    /// ```
    /// use gearbox::rails::ext::blocking::TapResult;
    ///
    /// let res: Result<(),_> = Err("hello".to_string());
    /// res.tap_err(|t| assert_eq!(t, &"hello".to_string())).ok();
    /// ```
    #[inline]
    fn tap_err<F: FnOnce(&E)>(self, op: F) -> Result<T, E> {
        match &self {
            Err(e) => op(e),
            _ => {}
        }
        self
    }

    /// # tap_mut
    ///
    /// This allows for modifying the data that are recieved through tap.
    /// Normally map will to fine in this instance, though this allows for modifying the data
    /// behind the reference.
    /// The difference between map and tap_mut is that it operates directly on the reference and
    /// that the datatype is not allowed to change.
    /// ```
    /// use gearbox::rails::ext::blocking::TapResult;
    ///
    /// let res: Result<(),_> = Err("hello".to_string());
    ///
    /// assert_eq!(res.tap_err_mut(|t| *t = "world".to_string()).unwrap_err(), "world".to_string());
    /// ```
    #[inline]
    fn tap_err_mut<F: FnOnce(&mut E)>(mut self, op: F) -> Self {
        match &mut self {
            Err(e) => op(e),
            _ => {}
        }
        self
    }
}
