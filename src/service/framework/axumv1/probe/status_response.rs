#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponse {
    status: ProbeResult,
    name: String,
}
