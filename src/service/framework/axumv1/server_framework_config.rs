use crate::net::socket_addr::SocketAddrs;

use crate::service::framework::axumv1::logger::{LogFormatter, LogOutput};

#[derive(Debug, Clone)]
pub struct FrameworkConfig {
    pub socket: SocketAddrs,
    pub port_default: u16,
    pub worker_pool: Option<usize>,
    pub logger: LogFormatter,
    pub logger_output: LogOutput,
    pub logger_discovery: bool,
    pub trace_layer: bool,
    pub use_http2: bool,
    pub certificates: Option<(String, String)>,
    pub include_subtasks_in_worker_pool: bool,
}

impl Default for FrameworkConfig {
    fn default() -> Self {
        Self {
            socket: SocketAddrs::default(),
            port_default: 3000,
            worker_pool: None,
            logger: LogFormatter::default(),
            logger_output: <LogOutput as Default>::default(),
            logger_discovery: false,
            trace_layer: false,
            use_http2: false,
            certificates: None,
            include_subtasks_in_worker_pool: false,
        }
    }
}
