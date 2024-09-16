use crate::common::process;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProcessInfo {
    pub process_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
}

impl Default for ProcessInfo {
    fn default() -> Self {
        Self {
            process_id: process::id(),
            application: Some(env!("CARGO_PKG_NAME").to_string()),
        }
    }
}

impl From<&ProcessInfo> for ProcessInfo {
    fn from(value: &ProcessInfo) -> Self {
        value.clone()
    }
}
