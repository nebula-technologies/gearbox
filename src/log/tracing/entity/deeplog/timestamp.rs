use crate::time::DateTime;
use serde_derive::{Deserialize, Serialize};
use toml::value::Time;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Timestamps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_timestamp: Option<DateTime>,
    pub timestamp: Option<DateTime>,
}

impl Default for Timestamps {
    fn default() -> Self {
        Self {
            timestamp: Some(DateTime::now()),
            received_timestamp: None,
        }
    }
}

impl From<&Timestamps> for Timestamps {
    fn from(value: &Timestamps) -> Self {
        value.clone()
    }
}
