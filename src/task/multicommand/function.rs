use crate::error::tracer::DynTracerError;
use crate::task::multicommand::command_response::CommandResponse;
use crate::task::multicommand::ExecutableCommand;
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::{
    fmt::{self, *},
    future::Future,
    pin::Pin,
    result::Result,
};
use tokio::task;
use tokio::task::JoinHandle;

pub struct Function(Box<dyn FnOnce(&mut CommandResponse) + Send>);

impl Function {
    pub fn new<F: FnOnce(&mut CommandResponse) + Send + 'static>(f: F) -> Self {
        Self(Box::new(f))
    }
}

impl ExecutableCommand for Function {
    fn exec(
        self,
        id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Vec<JoinHandle<Result<CommandResponse, DynTracerError>>>>>>
    {
        Box::pin(async {
            vec![task::spawn(async move {
                let mut resp = CommandResponse::new();
                resp.with_task_id(id);
                (self.0)(&mut resp);
                Ok(resp)
            })]
        })
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function Command")
    }
}
