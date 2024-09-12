pub mod facility;
pub mod severity;

use crate::collections::HashMap;
use crate::time::DateTime;
pub use facility::Facility;
pub use severity::Severity;

#[derive(Debug, Default)]
pub struct Syslog {
    facility: Option<Facility>,
    severity: Option<Severity>,
    version: Option<i32>,
    timestamp: Option<DateTime>,
    hostname: Option<String>,
    application: Option<String>,
    proc_id: Option<u32>,
    message_id: Option<Vec<String>>,
    message: Option<String>,
    data: Option<StructuredData>,
    file: Option<String>,
    line: Option<u32>,
}

#[derive(Debug)]
pub struct StructuredData(HashMap<String, Value>);
