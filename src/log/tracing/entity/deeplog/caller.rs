use serde_derive::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Caller {
    pub file: Option<String>,
    pub function: Option<String>,
    pub line: Option<i64>,
}
impl From<&Caller> for Caller {
    fn from(value: &Caller) -> Self {
        value.clone()
    }
}
