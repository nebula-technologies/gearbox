use crate::service::framework::axum::{
    HyperConfig, LogFormatterBackend, LogOutput, ModuleManager, ServerBuilder,
};
use std::net::IpAddr;

#[derive(Clone)]
pub struct ServerFrameworkConfig {
    pub address: IpAddr,
    pub port: u16,
    pub worker_pool: Option<usize>,
    pub logger: LogFormatterBackend,
    pub logger_output: LogOutput,
    pub logger_discovery: bool,
    pub trace_layer: bool,
    pub use_http2: bool,
    pub certificates: Option<(String, String)>,
    pub hyper_config: HyperConfig,
    pub include_subtasks_in_worker_pool: bool,
    pub module_manager: ModuleManager,
}

impl From<&ServerBuilder> for ServerFrameworkConfig {
    fn from(builder: &ServerBuilder) -> Self {
        Self {
            address: builder.address,
            port: builder.port,
            worker_pool: builder.worker_pool,
            logger: builder.logger.clone(),
            logger_output: builder.logger_output.clone(),
            logger_discovery: builder.logger_discovery,
            trace_layer: builder.trace_layer,
            use_http2: builder.use_http2,
            certificates: builder.certificates.clone(),
            hyper_config: builder.hyper_config.clone(),
            include_subtasks_in_worker_pool: builder.include_subtasks_in_worker_pool,
            module_manager: builder.module_manager.clone(),
        }
    }
}
