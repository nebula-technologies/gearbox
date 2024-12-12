pub mod log_formatter_wrapper;
pub mod output;

pub use self::{log_formatter_wrapper::LogFormatter, output::LogOutput};
use crate::common::ArcFn;
use crate::log::tracing::formatter::deeplog::LogStyleOutput;
use crate::log::tracing::formatter::DeepLogFormatter;
use crate::log::tracing::layer::LogLayer;
use crate::net::socket_addr::SocketAddrs;
use crate::prelude::tracing::Level;
use crate::service::discovery::service_discovery::{
    Broadcaster, Discoverer, ServiceDiscoveryState,
};
use crate::service::framework::axum::framework_manager::FrameworkManager;
use crate::service::framework::axum::module::definition::ModuleDefinition;
use crate::sync::CommonContainerTrait;
use bytes::Bytes;
use spin::rwlock::RwLock;
use std::sync::Arc;
use tracing::event;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

static DEFAULT_LOG_BACKEND: RwLock<Option<LogFormatter>> = RwLock::new(None);

pub struct LoggerModule;

impl<S> ModuleDefinition<S> for LoggerModule
where
    S: CommonContainerTrait + Clone + Send + Sync + 'static,
{
    const NAME: &'static str = "Logger";
    const BROADCAST: fn(&FrameworkManager<S>) -> Vec<(Broadcaster<Bytes>, Option<SocketAddrs>)> =
        |_| Vec::new();
    const DISCOVERY: fn(
        &FrameworkManager<S>,
    ) -> Vec<(
        Discoverer<Arc<ServiceDiscoveryState>, Bytes>,
        Option<SocketAddrs>,
    )> = |_| Vec::new();
    const PRE_INIT: fn(&FrameworkManager<S>) -> Vec<ArcFn<()>> = |m| -> Vec<ArcFn<()>> {
        let logger = m.config().logger.clone();
        let output = m.config().logger_output.clone();
        vec![Arc::new(move || {
            let mut formatter: LogFormatter = match logger.clone() {
                LogFormatter::Bunyan(t) => {
                    panic!("Bunyan not currently supported")
                }
                LogFormatter::DeepLog(t) => {
                    let formatter = DeepLogFormatter::default();
                    match output {
                        LogOutput::Minimal => formatter.set_output_style(LogStyleOutput::Minimal),
                        LogOutput::Full => formatter.set_output_style(LogStyleOutput::Full),
                        LogOutput::Human => formatter.set_output_style(LogStyleOutput::Human),
                        LogOutput::Default => formatter,
                    }
                    .into()
                }
                LogFormatter::Syslog(_) => {
                    panic!("Syslog not currently supported")
                }
            };

            let log_layer = LogLayer::new(None, std::io::stdout, formatter);
            let subscriber = Registry::default().with(log_layer);
            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to attach log subscriber");
            event!(
                Level::DEBUG,
                level = "emergency",
                "Testing subscriber with level override"
            );
        })]
    };
}
