use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug)]
pub struct CommandResponse {
    pub id: Option<String>,
    pub status: u8,
    pub output: Vec<Output>,
}

impl CommandResponse {
    pub fn new() -> CommandResponse {
        CommandResponse {
            id: None,
            status: 0,
            output: Vec::new(),
        }
    }

    pub fn out(&mut self, out: &str) {
        self.output.push(Output::out(out));
    }

    pub fn err(&mut self, err: &str) {
        self.output.push(Output::err(err));
    }

    pub fn with_task_id(&mut self, id: Option<String>) -> &mut Self {
        self.id = id;
        self
    }

    pub fn merge(&mut self, mut cmdrsp: CommandResponse) {
        self.status = cmdrsp.status;
        self.output.append(&mut cmdrsp.output);
    }

    pub fn merge_multiple(&mut self, cmdrsps: Vec<CommandResponse>) {
        for cmdrsp in cmdrsps {
            self.merge(cmdrsp);
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.output.iter().map(|o| o.to_string()).collect()
    }
}

impl Default for CommandResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum Output {
    Out(String),
    Err(String),
}

impl Output {
    pub fn out(out: &str) -> Output {
        Output::Out(out.to_string())
    }

    pub fn err(err: &str) -> Output {
        Output::Err(err.to_string())
    }

    #[cfg(feature = "std")]
    pub fn print(&self) {
        match self {
            Output::Out(out) => println!("{}", out),
            Output::Err(err) => eprintln!("{}", err),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Output::Out(out) => out.to_string(),
            Output::Err(err) => err.to_string(),
        }
    }
}
