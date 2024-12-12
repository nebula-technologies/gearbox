pub mod caller;
pub mod device;
pub mod process_info;
pub mod service;
pub mod system_info;
pub mod timestamp;
pub mod user;

use crate::log::tracing::entity::syslog::{Facility, Severity};
use crate::prelude::serde::derive::{Deserialize, Serialize};
pub use caller::Caller;
pub use device::Device;
pub use process_info::ProcessInfo;
pub use service::Service;
pub use system_info::SystemInfo;
pub use timestamp::Timestamps;
pub use user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeepLog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<Caller>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aid: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub payload_data: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stacktrace: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Timestamps>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facility: Option<Facility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}
impl Default for DeepLog {
    fn default() -> Self {
        Self {
            version: None,
            caller: None,
            correlation_id: None,
            device: None,
            duration: None,
            environment: None,
            id: None,
            aid: None,
            local_id: None,
            message: None,
            payload_data: Default::default(),
            service: None,
            severity: None,
            span_id: None,
            stacktrace: vec![],
            timestamps: Default::default(),
            trace_id: None,
            facility: None,
            process: None,
            system_info: None,
        }
    }
}
