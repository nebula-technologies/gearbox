use crate::time::DateTime;
use core::fmt::{Display, Formatter};
use core::net::IpAddr;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DiscoveryMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<DiscoveryMessageAdditionalInfo>,
    pub ip: Option<Vec<IpAddr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    pub port: Option<u16>,
    pub service_name: Option<String>,
    pub timestamp: Option<DateTime>,
    pub version: Option<String>,
    pub http: bool,
    pub http_api_schema_endpoint: Option<String>,
}

impl Display for DiscoveryMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} [{}]:{}",
            self.service_name.clone().unwrap_or("<Unknown>".to_string()),
            self.ip
                .clone()
                .map(|t| t
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","))
                .unwrap_or("<empty>".to_string()),
            self.port.clone().unwrap_or(0)
        )
    }
}

impl From<&DiscoveryMessage> for DiscoveryMessage {
    fn from(value: &DiscoveryMessage) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscoveryMessageAdditionalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl From<&DiscoveryMessageAdditionalInfo> for DiscoveryMessageAdditionalInfo {
    fn from(value: &DiscoveryMessageAdditionalInfo) -> Self {
        value.clone()
    }
}
