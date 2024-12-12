#[derive(Debug)]
pub enum DiscovererError {
    AdvertParsingError(String),
    InvalidDiscoveryData(String),
    ServiceNameMismatch,
    VersionParsingError(String),
    VersionMismatch,
    NoDataAvailable,
}
