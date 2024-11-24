#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProbeResult {
    Success,
    Failure,
    SuccessWith(String),
    FailureWith(String),
}
