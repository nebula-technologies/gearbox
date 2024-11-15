#[derive(Clone)]
pub enum LogFormatterBackend {
    Bunyan,
    DeepLog,
    Syslog,
}
