use crate::error::tracer::DynTracerError;
use crate::task::multicommand::command_response::{CommandResponse, Output};
use crate::task::multicommand::ExecutableCommand;
use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt::Display;
use core::future::Future;
use core::pin::Pin;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use tokio::task;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub workdir: Option<String>,
}

impl ShellCommand {
    pub fn new(command: &str) -> ShellCommand {
        ShellCommand {
            command: command.to_string(),
            args: Vec::new(),
            env: Vec::new(),
            workdir: None,
        }
    }

    pub fn workdir(&mut self, workdir: &str) -> &mut Self {
        self.workdir = Some(workdir.to_owned());
        self
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }
    pub fn args(&mut self, arg: &mut Vec<String>) -> &mut Self {
        self.args.append(arg);
        self
    }

    pub fn env(&mut self, name: &str, value: &str) {
        self.env.push((name.to_string(), value.to_string()));
    }
}

impl ExecutableCommand for ShellCommand {
    fn exec(
        self,
        id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Vec<JoinHandle<Result<CommandResponse, DynTracerError>>>>>>
    {
        Box::pin(async {
            println!("Spawning thread");
            vec![task::spawn(async move {
                let mut response = CommandResponse::new();
                response.with_task_id(id);
                let mut command = std::process::Command::new(&self.command);
                command.args(&self.args);
                for (name, value) in &self.env {
                    command.env(name, value);
                }
                if let Some(workdir) = &self.workdir {
                    command.current_dir(workdir);
                }

                command.stdout(Stdio::piped());
                command.stderr(Stdio::piped());
                command.stdin(Stdio::piped());

                match command.spawn() {
                    Ok(mut child) => {
                        let stdout = child.stdout.take().unwrap();
                        let stderr = child.stderr.take().unwrap();

                        let stdout_reader = BufReader::new(stdout);
                        let stderr_reader = BufReader::new(stderr);

                        for line in stdout_reader.lines() {
                            match line {
                                Ok(line) => response.output.push(Output::out(&line)),
                                Err(e) => response
                                    .output
                                    .push(Output::err(&format!("Error reading stdout: {}", e))),
                            }
                        }

                        for line in stderr_reader.lines() {
                            match line {
                                Ok(line) => response.output.push(Output::err(&line)),
                                Err(e) => response
                                    .output
                                    .push(Output::err(&format!("Error reading stderr: {}", e))),
                            }
                        }

                        let status = child.wait().expect("Command wasn't running");

                        response.status = status.code().unwrap_or(1) as u8;
                    }
                    Err(e) => {
                        response
                            .output
                            .push(Output::err(&format!("Failed to execute command: {}", e)));
                    }
                }

                Ok(response)
            })]
        })
    }
}

impl Display for ShellCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut command = self.command.clone();
        command = format!("{} {}", command, &self.args.join(" "));

        if let Some(workdir) = &self.workdir {
            command = format!("cd {} && {}", workdir, command);
        }

        for (name, value) in &self.env {
            command = format!("{}={} {}", name, value, command);
        }
        write!(f, "{}", command)
    }
}
