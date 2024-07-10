use core::fmt::Debug;

pub trait RailsSyslog<T> {
    fn log<L: FnOnce(&T)>(self, o: L) -> Self;
}

impl<T, E> RailsSyslog<Result<T, E>> for Result<T, E>
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
    use super::RailsSyslog;
    use crate::{
        alert, crit, critical, debug, emerg, emergency, err, error, info, notice, warn, warning,
    };

    #[test]
    fn test_logging_implementation() {
        let res: Result<&str, &str> = Ok("Hello");

        res.log(emergency!(Ok))
            .log(emergency!(Err))
            .log(emergency!(Ok, Kernel))
            .log(emergency!(Err, Kernel))
            .log(emergency!(Ok, "Hello {}"))
            .log(emergency!(Err, "Hello: {}"))
            .log(emergency!(Ok, Kernel, "Hello {}"))
            .log(emergency!(Err, Kernel, "Hello: {}"))
            .log(emerg!(Ok))
            .log(emerg!(Err))
            .log(emerg!(Ok, Kernel))
            .log(emerg!(Err, Kernel))
            .log(emerg!(Ok, "Hello {}"))
            .log(emerg!(Err, "Hello: {}"))
            .log(emerg!(Ok, Kernel, "Hello {}"))
            .log(emerg!(Err, Kernel, "Hello: {}"))
            .log(alert!(Ok))
            .log(alert!(Err))
            .log(alert!(Ok, Kernel))
            .log(alert!(Err, Kernel))
            .log(alert!(Ok, "Hello {}"))
            .log(alert!(Err, "Hello: {}"))
            .log(alert!(Ok, Kernel, "Hello {}"))
            .log(alert!(Err, Kernel, "Hello: {}"))
            .log(critical!(Ok))
            .log(critical!(Err))
            .log(critical!(Ok, Kernel))
            .log(critical!(Err, Kernel))
            .log(critical!(Ok, "Hello {}"))
            .log(critical!(Err, "Hello: {}"))
            .log(critical!(Ok, Kernel, "Hello {}"))
            .log(critical!(Err, Kernel, "Hello: {}"))
            .log(crit!(Ok))
            .log(crit!(Err))
            .log(crit!(Ok, Kernel))
            .log(crit!(Err, Kernel))
            .log(crit!(Ok, "Hello {}"))
            .log(crit!(Err, "Hello: {}"))
            .log(crit!(Ok, Kernel, "Hello {}"))
            .log(crit!(Err, Kernel, "Hello: {}"))
            .log(error!(Ok))
            .log(error!(Err))
            .log(error!(Ok, Kernel))
            .log(error!(Err, Kernel))
            .log(error!(Ok, "Hello {}"))
            .log(error!(Err, "Hello: {}"))
            .log(error!(Ok, Kernel, "Hello {}"))
            .log(error!(Err, Kernel, "Hello: {}"))
            .log(err!(Ok))
            .log(err!(Err))
            .log(err!(Ok, Kernel))
            .log(err!(Err, Kernel))
            .log(err!(Ok, "Hello {}"))
            .log(err!(Err, "Hello: {}"))
            .log(err!(Ok, Kernel, "Hello {}"))
            .log(err!(Err, Kernel, "Hello: {}"))
            .log(warning!(Ok))
            .log(warning!(Err))
            .log(warning!(Ok, Kernel))
            .log(warning!(Err, Kernel))
            .log(warning!(Ok, "Hello {}"))
            .log(warning!(Err, "Hello: {}"))
            .log(warning!(Ok, Kernel, "Hello {}"))
            .log(warning!(Err, Kernel, "Hello: {}"))
            .log(warn!(Ok))
            .log(warn!(Err))
            .log(warn!(Ok, Kernel))
            .log(warn!(Err, Kernel))
            .log(warn!(Ok, "Hello {}"))
            .log(warn!(Err, "Hello: {}"))
            .log(warn!(Ok, Kernel, "Hello {}"))
            .log(warn!(Err, Kernel, "Hello: {}"))
            .log(notice!(Ok))
            .log(notice!(Err))
            .log(notice!(Ok, Kernel))
            .log(notice!(Err, Kernel))
            .log(notice!(Ok, "Hello {}"))
            .log(notice!(Err, "Hello: {}"))
            .log(notice!(Ok, Kernel, "Hello {}"))
            .log(notice!(Err, Kernel, "Hello: {}"))
            .log(info!(Ok))
            .log(info!(Err))
            .log(info!(Ok, Kernel))
            .log(info!(Err, Kernel))
            .log(info!(Ok, "Hello {}"))
            .log(info!(Err, "Hello: {}"))
            .log(info!(Ok, Kernel, "Hello {}"))
            .log(info!(Err, Kernel, "Hello: {}"))
            .log(debug!(Ok))
            .log(debug!(Err))
            .log(debug!(Ok, Kernel))
            .log(debug!(Err, Kernel))
            .log(debug!(Ok, "Hello {}"))
            .log(debug!(Err, "Hello: {}"))
            .log(debug!(Ok, Kernel, "Hello {}"))
            .log(debug!(Err, Kernel, "Hello: {}"))
            .expect("TODO: panic message");
    }
}
