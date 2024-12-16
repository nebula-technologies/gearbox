#[derive(Debug)]
pub enum SocketAddrsError {
    /// Failed to capture IP addresses with additional context
    FailedToCaptureIp(String),
    /// Missing bind addresses in the SocketAddrs configuration
    MissingBindAddresses,
    /// Failed to determine broadcast addresses
    FailedToDetermineBroadcast(String),
}

impl std::fmt::Display for SocketAddrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketAddrsError::FailedToCaptureIp(reason) => {
                write!(f, "Failed to capture IP addresses: {}", reason)
            }
            SocketAddrsError::MissingBindAddresses => {
                write!(f, "No bind addresses provided in the configuration")
            }
            SocketAddrsError::FailedToDetermineBroadcast(reason) => {
                write!(f, "Failed to determine broadcast addresses: {}", reason)
            }
        }
    }
}
