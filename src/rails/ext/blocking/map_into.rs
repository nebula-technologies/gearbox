use alloc::boxed::Box;

pub trait RailsMapErrInto<T, E> {
    fn map_err_into<U>(self) -> Result<T, U>
    where
        Self: Sized,
        E: Into<U>;
}

impl<T, E> RailsMapErrInto<T, E> for Result<T, E> {
    fn map_err_into<U>(self) -> Result<T, U>
    where
        Self: Sized,
        E: Into<U>,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()), // Using Into here
        }
    }
}

pub trait RailsMapInto<T, E> {
    fn map_into<U>(self) -> Result<U, E>
    where
        Self: Sized,
        T: Into<U>;
}

impl<T, E> RailsMapInto<T, E> for Result<T, E> {
    fn map_into<U>(self) -> Result<U, E>
    where
        Self: Sized,
        T: Into<U>,
    {
        match self {
            Ok(t) => Ok(t.into()),
            Err(e) => Err(e),
        }
    }
}

pub trait RailsBoxErr<T, U> {
    fn box_err(self) -> Result<T, Box<U>>
    where
        Self: Sized;
}

impl<T, E> RailsBoxErr<T, E> for Result<T, E> {
    fn box_err(self) -> Result<T, Box<E>>
    where
        Self: Sized,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Box::new(e)),
        }
    }
}
pub trait RailsMapErrIntoBox<T, E>
where
    E: crate::error::tracer::ErrorDebug,
{
    fn map_err_box_into<U>(self) -> Result<T, U>
    where
        Self: Sized,
        Box<E>: Into<U>;
}

impl<T, E> RailsMapErrIntoBox<T, E> for Result<T, E>
where
    E: crate::error::tracer::ErrorDebug,
{
    fn map_err_box_into<U>(self) -> Result<T, U>
    where
        Self: Sized,
        Box<E>: Into<U>,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Box::new(e).into()),
        }
    }
}
