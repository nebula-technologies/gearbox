use crate::collections::HashMap;
use crate::service::framework::axum::probe::status_response::StatusResponse;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponses(HashMap<String, Vec<StatusResponse>>);
