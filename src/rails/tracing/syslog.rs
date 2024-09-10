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
    use crate::log::tracing::subscriber::entity::Facility::*;
    use crate::{
        alert, crit, critical, debug, emerg, emergency, err, error, info, notice, warn, warning,
    };

    #[test]
    fn test_logging_implementation() {
        let res: Result<&str, &str> = Ok("Hello");

        res.log(emergency!(Ok))
            .log(emergency!(Err))
            .log(emergency!(Ok, KernelMessages))
            .log(emergency!(Err, KernelMessages))
            .log(emergency!(Ok, "Hello {}"))
            .log(emergency!(Err, "Hello: {}"))
            .log(emergency!(Ok, KernelMessages, "Hello {}"))
            .log(emergency!(Err, KernelMessages, "Hello: {}"))
            .log(emerg!(Ok))
            .log(emerg!(Err))
            .log(emerg!(Ok, KernelMessages))
            .log(emerg!(Err, KernelMessages))
            .log(emerg!(Ok, "Hello {}"))
            .log(emerg!(Err, "Hello: {}"))
            .log(emerg!(Ok, KernelMessages, "Hello {}"))
            .log(emerg!(Err, KernelMessages, "Hello: {}"))
            .log(alert!(Ok))
            .log(alert!(Err))
            .log(alert!(Ok, KernelMessages))
            .log(alert!(Err, KernelMessages))
            .log(alert!(Ok, "Hello {}"))
            .log(alert!(Err, "Hello: {}"))
            .log(alert!(Ok, KernelMessages, "Hello {}"))
            .log(alert!(Err, KernelMessages, "Hello: {}"))
            .log(critical!(Ok))
            .log(critical!(Err))
            .log(critical!(Ok, KernelMessages))
            .log(critical!(Err, KernelMessages))
            .log(critical!(Ok, "Hello {}"))
            .log(critical!(Err, "Hello: {}"))
            .log(critical!(Ok, KernelMessages, "Hello {}"))
            .log(critical!(Err, KernelMessages, "Hello: {}"))
            .log(crit!(Ok))
            .log(crit!(Err))
            .log(crit!(Ok, KernelMessages))
            .log(crit!(Err, KernelMessages))
            .log(crit!(Ok, "Hello {}"))
            .log(crit!(Err, "Hello: {}"))
            .log(crit!(Ok, KernelMessages, "Hello {}"))
            .log(crit!(Err, KernelMessages, "Hello: {}"))
            .log(error!(Ok))
            .log(error!(Err))
            .log(error!(Ok, KernelMessages))
            .log(error!(Err, KernelMessages))
            .log(error!(Ok, "Hello {}"))
            .log(error!(Err, "Hello: {}"))
            .log(error!(Ok, KernelMessages, "Hello {}"))
            .log(error!(Err, KernelMessages, "Hello: {}"))
            .log(err!(Ok))
            .log(err!(Err))
            .log(err!(Ok, KernelMessages))
            .log(err!(Err, KernelMessages))
            .log(err!(Ok, "Hello {}"))
            .log(err!(Err, "Hello: {}"))
            .log(err!(Ok, KernelMessages, "Hello {}"))
            .log(err!(Err, KernelMessages, "Hello: {}"))
            .log(warning!(Ok))
            .log(warning!(Err))
            .log(warning!(Ok, KernelMessages))
            .log(warning!(Err, KernelMessages))
            .log(warning!(Ok, "Hello {}"))
            .log(warning!(Err, "Hello: {}"))
            .log(warning!(Ok, KernelMessages, "Hello {}"))
            .log(warning!(Err, KernelMessages, "Hello: {}"))
            .log(warn!(Ok))
            .log(warn!(Err))
            .log(warn!(Ok, KernelMessages))
            .log(warn!(Err, KernelMessages))
            .log(warn!(Ok, "Hello {}"))
            .log(warn!(Err, "Hello: {}"))
            .log(warn!(Ok, KernelMessages, "Hello {}"))
            .log(warn!(Err, KernelMessages, "Hello: {}"))
            .log(notice!(Ok))
            .log(notice!(Err))
            .log(notice!(Ok, KernelMessages))
            .log(notice!(Err, KernelMessages))
            .log(notice!(Ok, "Hello {}"))
            .log(notice!(Err, "Hello: {}"))
            .log(notice!(Ok, KernelMessages, "Hello {}"))
            .log(notice!(Err, KernelMessages, "Hello: {}"))
            .log(info!(Ok))
            .log(info!(Err))
            .log(info!(Ok, KernelMessages))
            .log(info!(Err, KernelMessages))
            .log(info!(Ok, "Hello {}"))
            .log(info!(Err, "Hello: {}"))
            .log(info!(Ok, KernelMessages, "Hello {}"))
            .log(info!(Err, KernelMessages, "Hello: {}"))
            .log(debug!(Ok))
            .log(debug!(Err))
            .log(debug!(Ok, KernelMessages))
            .log(debug!(Err, KernelMessages))
            .log(debug!(Ok, "Hello {}"))
            .log(debug!(Err, "Hello: {}"))
            .log(debug!(Ok, KernelMessages, "Hello {}"))
            .log(debug!(Err, KernelMessages, "Hello: {}"))
            .expect("TODO: panic message");
    }
}
