use std::fmt::Debug;
use tracing::{debug, error, info, trace, warn};

pub trait RailsLog<T> {
    fn log<'a>(self, o: Log<'a, Result<(), ()>>) -> Self;
}

impl<T: Debug, E: Debug> RailsLog<Result<T, E>> for Result<T, E> {
    fn log<'a>(self, log: Log<'a, Result<(), ()>>) -> Self {
        match (log.state, &self) {
            (Ok(_), Ok(t)) => match log.level {
                TracingLevels::Error => {
                    error!("{}{:?}", log.msg, t);
                }
                TracingLevels::Warn => {
                    warn!("{}{:?}", log.msg, t);
                }
                TracingLevels::Info => {
                    info!("{}{:?}", log.msg, t);
                }
                TracingLevels::Debug => {
                    debug!("{}{:?}", log.msg, t);
                }
                TracingLevels::Trace => {
                    trace!("{}{:?}", log.msg, t);
                }
            },
            (Err(_), Err(e)) => match log.level {
                TracingLevels::Error => {
                    error!("{}{:?}", log.msg, e);
                }
                TracingLevels::Warn => {
                    warn!("{}{:?}", log.msg, e);
                }
                TracingLevels::Info => {
                    info!("{}{:?}", log.msg, e);
                }
                TracingLevels::Debug => {
                    debug!("{}{:?}", log.msg, e);
                }
                TracingLevels::Trace => {
                    trace!("{}{:?}", log.msg, e);
                }
            },
            (_, _) => {}
        };
        self
    }
}

pub struct Log<'a, T> {
    msg: &'a str,
    state: T,
    level: TracingLevels,
}

pub trait RailsLogState<'a, O> {
    fn event(self, level: TracingLevels) -> Log<'a, O>;
    fn error(self) -> Log<'a, O>;
    fn warn(self) -> Log<'a, O>;
    fn info(self) -> Log<'a, O>;
    fn debug(self) -> Log<'a, O>;
    fn trace(self) -> Log<'a, O>;
}

impl<'a, O> RailsLogState<'a, Result<(), ()>> for O
    where
        O: FnOnce(()) -> Result<(), ()>,
{
    fn event(self, level: TracingLevels) -> Log<'a, Result<(), ()>> {
        Log {
            msg: "",
            state: self(()),
            level,
        }
    }
    fn error(self) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Error, "")
    }
    fn warn(self) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Warn, "")
    }
    fn info(self) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Info, "")
    }
    fn debug(self) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Debug, "")
    }
    fn trace(self) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Trace, "")
    }
}

pub enum TracingLevels {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub trait RailsLogMsgState<'a, O> {
    fn event_msg(self, level: TracingLevels, msg: &'a str) -> Log<'a, Result<(), ()>>;
    fn error_msg(self, msg: &'a str) -> Log<'a, O>;
    fn warn_msg(self, msg: &'a str) -> Log<'a, O>;
    fn info_msg(self, msg: &'a str) -> Log<'a, O>;
    fn debug_msg(self, msg: &'a str) -> Log<'a, O>;
    fn trace_msg(self, msg: &'a str) -> Log<'a, O>;
}

impl<'a, O> RailsLogMsgState<'a, Result<(), ()>> for O
    where
        O: FnOnce(()) -> Result<(), ()>,
{
    fn event_msg(self, level: TracingLevels, msg: &'a str) -> Log<'a, Result<(), ()>> {
        Log {
            msg,
            state: self(()),
            level,
        }
    }
    fn error_msg(self, msg: &'a str) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Error, msg)
    }
    fn warn_msg(self, msg: &'a str) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Warn, msg)
    }
    fn info_msg(self, msg: &'a str) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Info, msg)
    }
    fn debug_msg(self, msg: &'a str) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Debug, msg)
    }
    fn trace_msg(self, msg: &'a str) -> Log<'a, Result<(), ()>> {
        self.event_msg(TracingLevels::Trace, msg)
    }
}

#[cfg(test)]
mod test {
    use super::{RailsLog, RailsLogState};

    #[test]
    fn test_logging_implementation() {
        let res: Result<&str, &str> = Ok("Hello");
        res.log(Ok.debug()).log(Err.error());
    }
}
