use std::net::IpAddr;

pub struct CommonMessage {
    pub additional_info: Option<DiscoveryMessageAdditionalInfo>,
    pub ip: Option<Vec<IpAddr>>,
    pub mac: Option<String>,
    pub port: Option<u16>,
    pub service_name: Option<String>,
    pub timestamp: Option<DateTime>,
    pub version: Option<String>,
}

pub struct DiscoveryMessageAdditionalInfo {
    pub service_type: Option<String>,
    pub status: Option<String>,
}
