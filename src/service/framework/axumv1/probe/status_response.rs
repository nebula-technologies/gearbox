#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponse {
    pub(crate) status: ProbeResult,
    pub(crate) name: String,
}
