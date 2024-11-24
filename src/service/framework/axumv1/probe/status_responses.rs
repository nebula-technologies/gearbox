#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponses(HashMap<String, Vec<StatusResponse>>);
