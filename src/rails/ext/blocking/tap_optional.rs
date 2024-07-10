pub trait TapOptional<T> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self;
    fn tap_mut<F: FnOnce(&mut T)>(self, op: F) -> Self;
    fn tap_none<F: FnOnce()>(self, op: F) -> Self;
}

impl<T> TapOptional<T> for Option<T> {
    /// # tap
    ///
    /// tap mut gives an immutable reference of the underlying data.
    /// this can often be used as way to log or read data in a cleaner fashion then using a map
    /// where you will hate to return the data anyways even if nothing has been modified.
    /// Taps does not rely on a return value.
    /// ```
    /// use gearbox::rails::ext::blocking::TapOptional;
    ///
    /// let res = Some("hello".to_string());
    /// res.tap(|t| assert_eq!(t, &"hello".to_string()));
    /// ```
    #[inline]
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self {
        match &self {
            Some(t) => op(t),
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
    /// use gearbox::rails::ext::blocking::TapOptional;
    ///
    /// let mut res = Some("hello".to_string());
    /// assert_eq!(res.tap_mut(|t| *t = "world".to_string()).unwrap(), "world".to_string());
    /// ```
    #[inline]
    fn tap_mut<F: FnOnce(&mut T)>(mut self, op: F) -> Self {
        match &mut self {
            Some(t) => {
                op(t);
                self
            }
            None => self,
        }
    }

    fn tap_none<F: FnOnce()>(self, op: F) -> Self {
        match &self {
            _ => {}
            None => op(),
        }
        self
    }
}
