pub trait RailsMapErrInto<T, U> {
    fn map_err_into(self) -> Result<T, U>
    where
        Self: Sized;
}

impl<T, E: Into<U>, U> RailsMapErrInto<T, U> for Result<T, E> {
    fn map_err_into(self) -> Result<T, U>
    where
        Self: Sized,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()),
        }
    }
}

pub trait RailsMapInto<U, E> {
    fn map_into(self) -> Result<U, E>
    where
        Self: Sized;
}

impl<T: Into<U>, E, U> RailsMapInto<U, E> for Result<T, E> {
    fn map_into(self) -> Result<U, E>
    where
        Self: Sized,
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
pub trait RailsMapErrIntoBox<T, E, U>
where
    E: std::error::Error,
    Box<E>: Into<U>,
{
    fn map_err_box_into(self) -> Result<T, U>
    where
        Self: Sized;
}

impl<T, E, U> RailsMapErrIntoBox<T, E, U> for Result<T, E>
where
    E: std::error::Error,
    Box<E>: Into<U>,
{
    fn map_err_box_into(self) -> Result<T, U>
    where
        Self: Sized,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Box::new(e).into()),
        }
    }
}
