pub trait IntoOptional {
    fn into_opt(self) -> Option<Self>
    where
        Self: Sized;
}

impl IntoOptional for bool {
    fn into_opt(self) -> Option<bool> {
        match self {
            true => Some(self),
            false => None,
        }
    }
}
impl IntoOptional for String {
    fn into_opt(self) -> Option<String> {
        match self {
            s if s.is_empty() => None,
            _ => Some(self),
        }
    }
}

impl IntoOptional for i32 {
    fn into_opt(self) -> Option<i32> {
        match self {
            0 => None,
            _ => Some(self),
        }
    }
}

impl IntoOptional for i64 {
    fn into_opt(self) -> Option<i64> {
        match self {
            0 => None,
            _ => Some(self),
        }
    }
}

impl IntoOptional for f32 {
    fn into_opt(self) -> Option<f32> {
        match self {
            0.0 => None,
            _ => Some(self),
        }
    }
}

impl IntoOptional for f64 {
    fn into_opt(self) -> Option<f64> {
        match self {
            0.0 => None,
            _ => Some(self),
        }
    }
}

impl<T> IntoOptional for Vec<T> {
    fn into_opt(self) -> Option<Vec<T>> {
        match self.is_empty() {
            true => None,
            false => Some(self),
        }
    }
}
