use crate::time::DateTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Timestamps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_timestamp: Option<DateTime>,
    pub timestamp: Option<DateTime>,
}

impl From<&Timestamps> for Timestamps {
    fn from(value: &Timestamps) -> Self {
        value.clone()
    }
}
