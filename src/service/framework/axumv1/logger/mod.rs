pub mod log_formatter_wrapper;
pub mod output;

pub use self::{log_formatter_wrapper::LogFormatter, output::LogOutput};
use crate::common::ArcFn;
use crate::service::discovery::service_discovery::{Broadcaster, Discoverer};
use crate::service::framework::axum::LogFormatterBackend;
use crate::service::framework::axumv1::module::definition::ModuleDefinition;
use crate::service::framework::axumv1::{FrameworkConfig, RwStateController, StateController};
use axum::Router;
use bytes::Bytes;
use spin::rwlock::RwLock;
use std::sync::Arc;

static DEFAULT_LOG_BACKEND: RwLock<Option<LogFormatter>> = RwLock::new(None);

pub struct LoggerModule;

impl ModuleDefinition for LoggerModule {
    const NAME: &'static str = "Logger";
    const BROADCAST: fn(&FrameworkConfig) -> Vec<Broadcaster<Bytes>> = |_| Vec::new();
    const DISCOVERY: fn(&FrameworkConfig) -> Vec<Discoverer<(), Bytes>> = |_| Vec::new();
    const PRE_INIT: fn(&FrameworkConfig) -> Vec<ArcFn<()>> = |t| -> Vec<ArcFn<()>> {
        vec![
        //     ArcFn::new(move || {
        //     let mut formatter: LogFormatterWrapper = match logger {
        //         LogFormatterBackend::Bunyan => {
        //             panic!("Bunyan not currently supported")
        //         }
        //         LogFormatterBackend::DeepLog => {
        //             DEFAULT_LOG_BACKEND
        //                 .write()
        //                 .replace(LogFormatterBackend::DeepLog);
        //             let formatter = DeepLogFormatter::default();
        //             match output {
        //                 LogOutput::Minimal => {
        //                     formatter.set_output_style(deeplog::LogStyleOutput::Minimal)
        //                 }
        //                 LogOutput::Full => {
        //                     formatter.set_output_style(deeplog::LogStyleOutput::Full)
        //                 }
        //                 LogOutput::Human => {
        //                     formatter.set_output_style(deeplog::LogStyleOutput::Human)
        //                 }
        //                 LogOutput::Default => formatter,
        //             }
        //             .into()
        //         }
        //         LogFormatterBackend::Syslog => {
        //             panic!("Syslog not currently supported")
        //         }
        //     };
        //
        //     let log_layer = LogLayer::new(None, std::io::stdout, formatter);
        //     let subscriber = Registry::default().with(log_layer);
        //     tracing::subscriber::set_global_default(subscriber)
        //         .expect("Failed to attach log subscriber");
        //     event!(
        //         Level::DEBUG,
        //         level = "emergency",
        //         "Testing subscriber with level override"
        //     );
        // })
        ]
    };
}
