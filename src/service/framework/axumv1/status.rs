use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServerRuntimeError {
    UnknownRuntimeError,
}
impl Display for ServerRuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownRuntimeError => write!(f, "Unknown Runtime Error"),
        }
    }
}

#[derive(Debug)]
pub enum ServerRuntimeStatus {
    GracefulShutdown,
}

impl Display for ServerRuntimeStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GracefulShutdown => write!(f, "Graceful Shutdown of server"),
        }
    }
}
