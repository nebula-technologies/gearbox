pub mod command_response;
pub mod function;

#[cfg(feature = "std")]
pub mod shellcommand;

use crate::error::tracer::DynTracerError;
use crate::task::multicommand::command_response::CommandResponse;
use crate::task::multicommand::function::Function;
#[cfg(feature = "std")]
use crate::task::multicommand::shellcommand::ShellCommand;
use alloc::{boxed::Box, string::String, vec::Vec};
use core::future::Future;
use core::pin::Pin;
use tokio::task::JoinHandle;

pub trait ExecutableCommand {
    fn exec(
        self,
        id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Vec<JoinHandle<Result<CommandResponse, DynTracerError>>>>>>;
}
#[derive(Debug)]
pub enum MultiCommand {
    #[cfg(feature = "std")]
    ShellCommand(ShellCommand),
    Function(Function),
}

impl ExecutableCommand for MultiCommand {
    fn exec(
        self,
        id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Vec<JoinHandle<Result<CommandResponse, DynTracerError>>>>>>
    {
        Box::pin(async {
            match self {
                #[cfg(feature = "std")]
                MultiCommand::ShellCommand(shell_command) => shell_command.exec(id).await,
                MultiCommand::Function(function) => function.exec(id).await,
            }
        })
    }
}
