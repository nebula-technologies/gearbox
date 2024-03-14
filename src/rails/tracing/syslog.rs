use crate::log::syslog::Facility;
use crate::log::syslog::Severity;
use crate::{alert, critical, debug, emergency, error, info, notice, warning};
use std::fmt::Debug;

pub trait RailsSyslog<T> {
    fn log<'a>(self, o: Syslog<'a, Result<(), ()>>) -> Self;
}

impl<T: Debug, E: Debug> RailsSyslog<Result<T, E>> for Result<T, E> {
    fn log<'a>(self, log: Syslog<'a, Result<(), ()>>) -> Self {
        match (log.state, &self) {
            (Ok(_), Ok(t)) => match log.level {
                Severity::Emergency => {
                    emergency!("{}{:?}", log.msg, t);
                }
                Severity::Alert => {
                    alert!("{}{:?}", log.msg, t);
                }
                Severity::Critical => {
                    critical!("{}{:?}", log.msg, t);
                }
                Severity::Error => {
                    error!("{}{:?}", log.msg, t);
                }
                Severity::Warning => {
                    warning!("{}{:?}", log.msg, t);
                }
                Severity::Notice => {
                    notice!("{}{:?}", log.msg, t);
                }
                Severity::Info => {
                    info!("{}{:?}", log.msg, t);
                }
                Severity::Debug => {
                    debug!("{}{:?}", log.msg, t);
                }
            },
            (Err(_), Err(e)) => match log.level {
                Severity::Emergency => {
                    emergency!("{}{:?}", log.msg, e);
                }
                Severity::Alert => {
                    alert!("{}{:?}", log.msg, e);
                }
                Severity::Critical => {
                    critical!("{}{:?}", log.msg, e);
                }
                Severity::Error => {
                    error!("{}{:?}", log.msg, e);
                }
                Severity::Warning => {
                    warning!("{}{:?}", log.msg, e);
                }
                Severity::Notice => {
                    notice!("{}{:?}", log.msg, e);
                }
                Severity::Info => {
                    info!("{}{:?}", log.msg, e);
                }
                Severity::Debug => {
                    debug!("{}{:?}", log.msg, e);
                }
            },
            (_, _) => {}
        };
        self
    }
}

pub struct Syslog<'a, T> {
    msg: &'a str,
    state: T,
    level: Severity,
    facility: Facility,
}

impl<'a, T> Syslog<'a, T> {
    fn kernel(mut self) -> Self {
        self.facility = Facility::Kernel;
        self
    }
    fn user(mut self) -> Self {
        self.facility = Facility::User;
        self
    }
    fn mail(mut self) -> Self {
        self.facility = Facility::Mail;
        self
    }
    fn system(mut self) -> Self {
        self.facility = Facility::System;
        self
    }
    fn security(mut self) -> Self {
        self.facility = Facility::Security;
        self
    }
    fn syslog(mut self) -> Self {
        self.facility = Facility::Syslog;
        self
    }
    fn printer(mut self) -> Self {
        self.facility = Facility::Printer;
        self
    }
    fn news(mut self) -> Self {
        self.facility = Facility::News;
        self
    }
    fn uucp(mut self) -> Self {
        self.facility = Facility::Uucp;
        self
    }
    fn clock(mut self) -> Self {
        self.facility = Facility::Clock;
        self
    }
    fn auth(mut self) -> Self {
        self.facility = Facility::Auth;
        self
    }
    fn ftp(mut self) -> Self {
        self.facility = Facility::Ftp;
        self
    }
    fn ntp(mut self) -> Self {
        self.facility = Facility::Ntp;
        self
    }
    fn audit(mut self) -> Self {
        self.facility = Facility::Audit;
        self
    }
    fn alert(mut self) -> Self {
        self.facility = Facility::Alert;
        self
    }
    fn clock2(mut self) -> Self {
        self.facility = Facility::Clock2;
        self
    }
    fn local0(mut self) -> Self {
        self.facility = Facility::Local0;
        self
    }
    fn local1(mut self) -> Self {
        self.facility = Facility::Local1;
        self
    }
    fn local2(mut self) -> Self {
        self.facility = Facility::Local2;
        self
    }
    fn local3(mut self) -> Self {
        self.facility = Facility::Local3;
        self
    }
    fn local4(mut self) -> Self {
        self.facility = Facility::Local4;
        self
    }
    fn kerne5(mut self) -> Self {
        self.facility = Facility::Local5;
        self
    }
    fn local6(mut self) -> Self {
        self.facility = Facility::Local6;
        self
    }
    fn local7(mut self) -> Self {
        self.facility = Facility::Local7;
        self
    }
}

pub trait RailsSyslogState<'a, O> {
    fn event(self, level: Severity, facility: Facility) -> Syslog<'a, O>;
    fn emergency(self) -> Syslog<'a, O>;
}

impl<'a, O> RailsSyslogState<'a, Result<(), ()>> for O
where
    O: FnOnce(()) -> Result<(), ()>,
{
    fn event(self, level: Severity, facility: Facility) -> Syslog<'a, Result<(), ()>> {
        Syslog {
            msg: "",
            state: self(()),
            level,
            facility,
        }
    }
    fn emergency(self) -> Syslog<'a, Result<(), ()>> {
        self.event(Severity::Emergency, Facility::User)
    }
}

pub trait RailsSyslogMsgState<'a, O> {
    fn event_msg(
        self,
        level: Severity,
        facility: Facility,
        msg: &'a str,
    ) -> Syslog<'a, Result<(), ()>>;
    fn emergency_msg(self, msg: &'a str) -> Syslog<'a, O>;
}

impl<'a, O> RailsSyslogMsgState<'a, Result<(), ()>> for O
where
    O: FnOnce(()) -> Result<(), ()>,
{
    fn event_msg(
        self,
        level: Severity,
        facility: Facility,
        msg: &'a str,
    ) -> Syslog<'a, Result<(), ()>> {
        Syslog {
            msg,
            state: self(()),
            level,
            facility,
        }
    }
    fn emergency_msg(self, msg: &'a str) -> Syslog<'a, Result<(), ()>> {
        self.event_msg(Severity::Emergency, Facility::User, msg)
    }
}

#[cfg(test)]
mod test {
    use super::{RailsSyslog, RailsSyslogState};

    #[test]
    fn test_logging_implementation() {
        let res: Result<&str, &str> = Ok("Hello");
        res.log(Ok.emergency()).log(Err.emergency());
    }
}
