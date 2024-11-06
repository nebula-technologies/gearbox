#[derive(Clone)]
pub enum LogFormatter {
    Bunyan,
    DeepLog,
    Syslog,
}
