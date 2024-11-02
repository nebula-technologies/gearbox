pub trait VecExt<U> {
    fn if_empty<F>(self, f: F) -> U
    where
        F: FnOnce() -> U;
}

impl<T> VecExt<Vec<T>> for Vec<T> {
    fn if_empty<F>(self, f: F) -> Vec<T>
    where
        F: FnOnce() -> Vec<T>,
    {
        if self.is_empty() {
            f()
        } else {
            self
        }
    }
}

impl<T: Clone> VecExt<Vec<T>> for &Vec<T> {
    fn if_empty<F>(self, f: F) -> Vec<T>
    where
        F: FnOnce() -> Vec<T>,
    {
        if self.is_empty() {
            f()
        } else {
            self.clone()
        }
    }
}

impl<T> VecExt<Vec<T>> for Option<Vec<T>> {
    fn if_empty<F>(self, f: F) -> Vec<T>
    where
        F: FnOnce() -> Vec<T>,
    {
        if let Some(v) = self {
            if v.is_empty() {
                f()
            } else {
                v
            }
        } else {
            f()
        }
    }
}

impl<T: Clone> VecExt<Vec<T>> for &Option<Vec<T>> {
    fn if_empty<F>(self, f: F) -> Vec<T>
    where
        F: FnOnce() -> Vec<T>,
    {
        if let Some(v) = self {
            if v.is_empty() {
                f()
            } else {
                v.clone()
            }
        } else {
            f()
        }
    }
}
