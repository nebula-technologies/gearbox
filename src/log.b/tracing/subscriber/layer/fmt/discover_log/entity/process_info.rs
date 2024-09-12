use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProcessInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
}
impl From<&ProcessInfo> for ProcessInfo {
    fn from(value: &ProcessInfo) -> Self {
        value.clone()
    }
}
