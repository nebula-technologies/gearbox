use core::fmt::Debug;

pub trait RailsLog<T> {
    fn log<L: FnOnce(&T)>(self, o: L) -> Self;
}

impl<T, E> RailsLog<Result<T, E>> for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn log<L: FnOnce(&Result<T, E>)>(self, log: L) -> Self {
        log(&self);
        self
    }
}

#[cfg(test)]
mod test {
    use super::RailsLog;
    use crate::{debug, error};

    #[test]
    fn test_logging_implementation() {
        let res: Result<&str, &str> = Ok("Hello");
        res.log(info!(Ok)).log(error!(Err)).ok();
    }
}
