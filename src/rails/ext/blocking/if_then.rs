pub trait RailsIfExt<U, V> {
    fn if_then<P>(self, cmp: P) -> U
    where
        Self: Sized,
        P: FnOnce(&V) -> bool;
}

impl RailsIfExt<Option<bool>, bool> for bool {
    fn if_then<P>(self, cmp: P) -> Option<bool>
    where
        Self: Sized,
        P: FnOnce(&bool) -> bool,
    {
        match cmp(&self) {
            true => Some(self),
            false => None,
        }
    }
}

impl<T> RailsIfExt<Option<T>, T> for Option<T> {
    fn if_then<P>(self, cmp: P) -> Option<T>
    where
        Self: Sized,
        P: FnOnce(&T) -> bool,
    {
        match self {
            Some(t) => match cmp(&t) {
                true => Some(t),
                false => None,
            },
            _ => None,
        }
    }
}

impl<T: Into<E>, E> RailsIfExt<Result<T, E>, T> for Result<T, E> {
    fn if_then<P>(self, cmp: P) -> Result<T, E>
    where
        Self: Sized,
        P: FnOnce(&T) -> bool,
    {
        match self {
            Ok(t) => match cmp(&t) {
                true => Ok(t),
                false => Err(t.into()),
            },
            Err(e) => Err(e),
        }
    }
}

pub enum IfState<T, F, N> {
    True(T),
    False(F),
    Null(N),
}

pub trait RailsWhenStateExt<T, F, N, V>
where
    Self: Sized,
{
    fn when<P>(self, cmp: P) -> IfState<T, F, N>
    where
        P: FnOnce(&V) -> bool;

    // fn when_err<P>(self, cmp: P) -> IfState<Self>
    // where
    //     P: FnOnce(&E) -> bool;
}

impl<T, E> RailsWhenStateExt<T, T, E, T> for Result<T, E> {
    fn when<P>(self, cmp: P) -> IfState<T, T, E>
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            Ok(t) => match cmp(&t) {
                true => IfState::True(t),
                false => IfState::False(t),
            },
            Err(e) => IfState::Null(e),
        }
    }
}

pub trait RailsThenDoExt<UT, T, F, N, U> {
    fn then_do<P>(self, o: P) -> IfState<UT, F, N>
    where
        P: FnOnce(T) -> U;
    fn else_do<P>(self, o: P) -> IfState<T, UT, N>
    where
        P: FnOnce(F) -> U;
}

impl<UT, T, F, E> RailsThenDoExt<UT, T, F, E, Result<UT, E>> for IfState<T, F, E> {
    fn then_do<P>(self, o: P) -> IfState<UT, F, E>
    where
        P: FnOnce(T) -> Result<UT, E>,
    {
        match self {
            IfState::True(t) => match o(t) {
                Ok(t) => IfState::True(t),
                Err(e) => IfState::Null(e),
            },
            IfState::False(t) => IfState::False(t),
            IfState::Null(t) => IfState::Null(t),
        }
    }

    fn else_do<P>(self, o: P) -> IfState<T, UT, E>
    where
        P: FnOnce(F) -> Result<UT, E>,
    {
        match self {
            IfState::True(t) => IfState::True(t),
            IfState::False(t) => match o(t) {
                Ok(t) => IfState::False(t),
                Err(e) => IfState::Null(e),
            },
            IfState::Null(t) => IfState::Null(t),
        }
    }
}

trait RailsWhenDoneExt<T, E> {
    fn done(self) -> Result<T, E>;
}

impl<T, E> RailsWhenDoneExt<T, E> for IfState<T, T, E> {
    fn done(self) -> Result<T, E> {
        match self {
            IfState::True(t) => Ok(t),
            IfState::False(t) => Ok(t),
            IfState::Null(t) => Err(t),
        }
    }
}

pub trait RailsUnlessExt<T, V> {
    fn unless<P: FnOnce(&V) -> bool>(self, cmp: P) -> T;
}

impl<T> RailsUnlessExt<Option<T>, T> for Option<T> {
    fn unless<P: FnOnce(&T) -> bool>(self, cmp: P) -> Option<T> {
        match self {
            Some(t) => match cmp(&t) {
                true => None,
                false => Some(t),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::rails::ext::blocking::if_then::{
        RailsThenDoExt, RailsWhenDoneExt, RailsWhenStateExt,
    };

    #[test]
    fn test_if_then() {
        let _t: Result<_, ()> = Ok(1).when(|t| *t == 1).then_do(|i| Ok(i + 1)).done();
    }
}
