pub trait RailsDoExt<T, E> {
    fn r#do<F>(&self, f: F)
    where
        F: FnOnce(&T);
    fn r#do_err<F>(&self, f: F)
    where
        F: FnOnce(&E);
}

impl<T> RailsDoExt<T, ()> for Option<T> {
    fn r#do<F>(&self, f: F)
    where
        F: FnOnce(&T),
    {
        match self {
            Some(t) => f(t),
            _ => {}
        }
    }

    fn r#do_err<F>(&self, f: F)
    where
        F: FnOnce(&()),
    {
        match self {
            None => f(&()),
            _ => {}
        }
    }
}

impl<T, E> RailsDoExt<T, E> for Result<T, E> {
    fn r#do<F>(&self, f: F)
    where
        F: FnOnce(&T),
    {
        match self {
            Ok(t) => f(t),
            _ => {}
        }
    }

    fn r#do_err<F>(&self, f: F)
    where
        F: FnOnce(&E),
    {
        match self {
            Err(e) => f(e),
            _ => {}
        }
    }
}
