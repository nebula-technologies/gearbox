#[derive(Debug)]
pub enum SocketAddrError {
    /// Failed to capture IP addresses with additional context
    FailedToCaptureIp(String),
    /// Missing bind addresses in the SocketAddrs configuration
    MissingBindAddresses,
    /// Failed to determine broadcast addresses
    FailedToDetermineBroadcast(String),
}

impl std::fmt::Display for SocketAddrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketAddrError::FailedToCaptureIp(reason) => {
                write!(f, "Failed to capture IP addresses: {}", reason)
            }
            SocketAddrError::MissingBindAddresses => {
                write!(f, "No bind addresses provided in the configuration")
            }
            SocketAddrError::FailedToDetermineBroadcast(reason) => {
                write!(f, "Failed to determine broadcast addresses: {}", reason)
            }
        }
    }
}
