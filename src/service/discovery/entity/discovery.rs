use crate::time::DateTime;
use bytes::Bytes;
use core::fmt::{Display, Formatter};
use core::net::IpAddr;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde()]
pub struct Advertisement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdvertisementExtra>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<Vec<IpAddr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "crate::common::serde_checks::is_false")]
    pub http: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_api_schema_endpoint: Option<String>,
}

#[cfg(feature = "with_json")]
impl From<Advertisement> for Bytes {
    fn from(value: Advertisement) -> Self {
        serde_json::to_string(&value)
            .map(Bytes::from)
            .unwrap_or_default()
    }
}

impl Display for Advertisement {
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

impl From<&Advertisement> for Advertisement {
    fn from(value: &Advertisement) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdvertisementExtra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl From<&AdvertisementExtra> for AdvertisementExtra {
    fn from(value: &AdvertisementExtra) -> Self {
        value.clone()
    }
}
