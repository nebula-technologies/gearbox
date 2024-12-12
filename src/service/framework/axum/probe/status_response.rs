use crate::service::framework::axum::probe::probe_result::ProbeResult;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponse {
    pub(crate) status: ProbeResult,
    pub(crate) name: String,
}
