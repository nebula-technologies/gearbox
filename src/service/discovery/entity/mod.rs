use crate::time::DateTime;
use bytes::Bytes;
use core::fmt::{Display, Formatter};
use core::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Advertisement {
    pub ip: Option<Vec<IpAddr>>,
    pub mac: Option<String>,
    pub port: Option<u16>,
    pub service_id: Option<String>,
    pub hostname: Option<String>,
    pub timestamp: Option<DateTime>,
    pub version: Option<String>,
}

impl From<Advertisement> for Bytes {
    fn from(value: Advertisement) -> Self {
        Bytes::from(String::from(value))
    }
}
impl From<Advertisement> for String {
    fn from(value: Advertisement) -> Self {
        format!(
            "{};;{};;{};;{};;{};;{};;{}",
            value
                .ip
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(","),
            value.mac.clone().unwrap_or_default(),
            value.port.map(|t| t.to_string()).unwrap_or("".to_string()),
            value.service_id.clone().unwrap_or_default(),
            value.hostname.clone().unwrap_or_default(),
            value
                .timestamp
                .map(|t| t.to_rfc3339())
                .unwrap_or("".to_string()),
            value.version.clone().unwrap_or_default(),
        )
    }
}

#[derive(Debug)]
pub enum ParseAdvertisementError {
    InvalidIp(std::net::AddrParseError),
    InvalidTimestamp(crate::time::Error),
    InvalidPort(std::num::ParseIntError),
    MissingField,
}

impl Display for ParseAdvertisementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseAdvertisementError::InvalidIp(e) => write!(f, "Invalid IP address: {}", e),
            ParseAdvertisementError::InvalidTimestamp(e) => write!(f, "Invalid timestamp: {}", e),
            ParseAdvertisementError::InvalidPort(e) => write!(f, "Invalid port: {}", e),
            ParseAdvertisementError::MissingField => write!(f, "Missing field"),
        }
    }
}

impl TryFrom<Bytes> for Advertisement {
    type Error = ParseAdvertisementError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Bytes> for Advertisement {
    type Error = ParseAdvertisementError;

    fn try_from(value: &Bytes) -> Result<Self, Self::Error> {
        let data = String::from_utf8_lossy(&value);
        let parts: Vec<&str> = data.split(";;").collect();

        if parts.len() != 7 {
            return Err(ParseAdvertisementError::MissingField);
        }

        // Parse the IP addresses
        let ip: Option<Vec<IpAddr>> = if !parts[0].is_empty() {
            let ip_result: Result<Vec<IpAddr>, _> =
                parts[0].split(',').map(IpAddr::from_str).collect();
            match ip_result {
                Ok(ip_vec) => Some(ip_vec),
                Err(e) => return Err(ParseAdvertisementError::InvalidIp(e)),
            }
        } else {
            None
        };

        // Parse the MAC address
        let mac = if !parts[1].is_empty() {
            Some(parts[1].to_string())
        } else {
            None
        };

        // Parse the port
        let port = if !parts[2].is_empty() {
            match parts[2].parse::<u16>() {
                Ok(port) => Some(port),
                Err(e) => return Err(ParseAdvertisementError::InvalidPort(e)),
            }
        } else {
            None
        };

        // Parse the service ID
        let service_id = if !parts[3].is_empty() {
            Some(parts[3].to_string())
        } else {
            None
        };

        // Parse the hostname
        let hostname = if !parts[4].is_empty() {
            Some(parts[4].to_string())
        } else {
            None
        };

        // Parse the timestamp using gearbox::time::DateTime
        let timestamp = if !parts[5].is_empty() {
            match DateTime::from_str(parts[5]) {
                Ok(timestamp) => Some(timestamp),
                Err(e) => return Err(ParseAdvertisementError::InvalidTimestamp(e)),
            }
        } else {
            None
        };

        // Parse the version
        let version = if !parts[6].is_empty() {
            Some(parts[6].to_string())
        } else {
            None
        };

        Ok(Advertisement {
            ip,
            mac,
            port,
            service_id,
            hostname,
            timestamp,
            version,
        })
    }
}

impl TryFrom<Arc<Bytes>> for Advertisement {
    type Error = ParseAdvertisementError;

    fn try_from(value: Arc<Bytes>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<&Arc<Bytes>> for Advertisement {
    type Error = ParseAdvertisementError;

    fn try_from(value: &Arc<Bytes>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl Display for Advertisement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} [{}]:{}",
            self.service_id.clone().unwrap_or("<Unknown>".to_string()),
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
