use core::fmt::Debug;
use tracing::{debug, error, info, trace, warn};

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

    #[test]
    fn test_logging_implementation() {
        let res: Result<&str, &str> = Ok("Hello");
        res.log(Ok.debug()).log(Err.error());
    }
}
