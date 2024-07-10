use crate::collections::simple_linked_list::SimpleLinkedList;
use crate::collections::VecDeque;
use crate::error::tracer::DynTracerError;
use crate::rails::tracing::syslog::RailsSyslog;
use crate::task::multicommand::command_response::CommandResponse;
use crate::task::multicommand::{ExecutableCommand, MultiCommand};
use crate::{error, tracer_dyn_err};
use alloc::{format, string::String, vec::Vec};
use futures::future::join_all;
use hashbrown::HashMap;
use semver::{Version, VersionReq};

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub async fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_task(&self, id: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }
}

pub struct Task {
    id: String,
    version: Version,
    command: Option<MultiCommand>,
    dependencies: Vec<Dependency>,
}

impl Task {
    pub fn new(
        id: String,
        version: Version,
        command: Option<MultiCommand>,
        dependencies: Vec<Dependency>,
    ) -> Task {
        Task {
            id,
            version,
            command,
            dependencies,
        }
    }
}

pub struct Dependency {
    require_task: String,
    version_req: VersionReq,
}

impl Dependency {
    pub fn new(require_task: String, version_req: VersionReq) -> Dependency {
        Dependency {
            require_task,
            version_req,
        }
    }

    pub fn matches(&self, task: &Task) -> bool {
        self.version_req.matches(&task.version)
    }
}

#[derive(Debug)]
pub struct TaskRunner {
    #[allow(unused)]
    task_order: SimpleLinkedList<Vec<TaskRunable>>,
}

impl TaskRunner {
    pub async fn run(&mut self) -> Result<Vec<CommandResponse>, DynTracerError> {
        let mut resp = Vec::new();
        while !self.task_order.is_empty() {
            let level = self.task_order.pop_front().unwrap();
            let mut tasks = Vec::new();
            for task in level {
                if let Some(command) = task.command {
                    let c = command.exec(Option::from(task.id)).await;
                    tasks.extend(c);
                }
            }
            join_all(tasks)
                .await
                .into_iter()
                .map(|t| {
                    t.map_err(|e| tracer_dyn_err!(e.to_string()))
                        .and_then(|t| t)
                        .log(error!(Err))
                })
                .collect::<Vec<Result<CommandResponse, DynTracerError>>>()
                .into_iter()
                .collect::<Result<Vec<CommandResponse>, DynTracerError>>()
                .map(|t| resp.extend(t))?;
        }
        Ok(resp)
    }
}

impl TryFrom<TaskManager> for TaskRunner {
    type Error = DynTracerError;

    fn try_from(mut task_manager: TaskManager) -> Result<Self, DynTracerError> {
        // Step 1: Build the graph
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        for task in &task_manager.tasks {
            let id = task.id.clone();
            in_degree.entry(id.clone()).or_insert(0);
            for dep in &task.dependencies {
                // Check if the version requirement is met
                if let Some(dep_task) = task_manager.get_task(&dep.require_task) {
                    if !dep.version_req.matches(&dep_task.version) {
                        return Err(tracer_dyn_err!(format!(
                            "Version requirement {} for task {} is not satisfied by version {}",
                            dep.version_req, dep.require_task, dep_task.version
                        )));
                    }
                } else {
                    return Err(tracer_dyn_err!(format!(
                        "Required task {} not found",
                        dep.require_task
                    )));
                }

                graph
                    .entry(dep.require_task.clone())
                    .or_insert_with(Vec::new)
                    .push(id.clone());
                *in_degree.entry(id.clone()).or_insert(0) += 1;
            }
        }

        // Step 2: Topological sort using Kahn's algorithm
        let mut queue: VecDeque<String> = VecDeque::new();
        for (id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(id.clone());
            }
        }

        let mut levels: Vec<Vec<String>> = Vec::new();
        while !queue.is_empty() {
            let mut level: Vec<String> = Vec::new();
            let mut size = queue.len();
            while size > 0 {
                let id = queue.pop_front().unwrap();
                level.push(id.clone());
                if let Some(deps) = graph.get(&id) {
                    for dep in deps {
                        if let Some(degree) = in_degree.get_mut(dep) {
                            *degree -= 1;
                            if *degree == 0 {
                                queue.push_back(dep.clone());
                            }
                        }
                    }
                }
                size -= 1;
            }
            levels.push(level);
        }

        // Step 3: Ensure there are no cycles
        if levels.iter().flatten().count() != task_manager.tasks.len() {
            return Err(tracer_dyn_err!("Cycle detected in task dependencies"));
        }

        // Step 4: Create TaskRunner
        let mut task_order = SimpleLinkedList::new();
        for level in levels {
            let mut runable_level: Vec<TaskRunable> = Vec::new();
            for task_id in level {
                if let Some(task) = task_manager.get_task_mut(&task_id) {
                    runable_level.push(TaskRunable {
                        id: task.id.clone(),
                        command: task.command.take(),
                    });
                }
            }
            task_order.push_front(runable_level);
        }

        Ok(TaskRunner { task_order })
    }
}

#[derive(Debug)]
pub struct TaskRunable {
    #[allow(unused)]
    id: String,
    #[allow(unused)]
    command: Option<MultiCommand>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::multicommand::command_response::{CommandResponse, Output};
    use crate::task::multicommand::function::Function;
    use crate::task::multicommand::shellcommand::ShellCommand;
    use crate::task::multicommand::{ExecutableCommand, MultiCommand};
    use futures::future::join_all;
    use semver::Version;

    use tokio::task::JoinHandle;

    #[tokio::test]
    async fn test_add_and_get_task() {
        let mut manager = TaskManager::new();
        let task = Task::new(
            "task1".to_string(),
            Version::parse("1.0.0").unwrap(),
            None,
            Vec::new(),
        );
        manager.add_task(task).await;
        let fetched_task = manager.get_task("task1");
        assert!(fetched_task.is_some());
        assert_eq!(fetched_task.unwrap().id, "task1");
    }

    #[test]
    fn test_dependency_matches() {
        let task = Task::new(
            "task1".to_string(),
            Version::parse("1.0.0").unwrap(),
            None,
            Vec::new(),
        );
        let dependency =
            Dependency::new("task1".to_string(), VersionReq::parse(">=1.0.0").unwrap());
        assert!(dependency.matches(&task));
    }

    #[tokio::test]
    async fn test_task_runner_no_cycles() {
        let mut manager = TaskManager::new();
        let task1 = Task::new(
            "task1".to_string(),
            Version::parse("1.0.0").unwrap(),
            None,
            Vec::new(),
        );
        let task2 = Task::new(
            "task2".to_string(),
            Version::parse("1.0.0").unwrap(),
            None,
            vec![Dependency::new(
                "task1".to_string(),
                VersionReq::parse(">=1.0.0").unwrap(),
            )],
        );
        manager.add_task(task1).await;
        manager.add_task(task2).await;

        let runner = TaskRunner::try_from(manager);
        assert!(runner.is_ok());
    }

    #[tokio::test]
    async fn test_task_runner_with_cycles() {
        let mut manager = TaskManager::new();
        let task1 = Task::new(
            "task1".to_string(),
            Version::parse("1.0.0").unwrap(),
            None,
            vec![Dependency::new(
                "task2".to_string(),
                VersionReq::parse(">=1.0.0").unwrap(),
            )],
        );
        let task2 = Task::new(
            "task2".to_string(),
            Version::parse("1.0.0").unwrap(),
            None,
            vec![Dependency::new(
                "task1".to_string(),
                VersionReq::parse(">=1.0.0").unwrap(),
            )],
        );
        manager.add_task(task1).await;
        manager.add_task(task2).await;

        let runner = TaskRunner::try_from(manager);
        assert!(runner.is_err());
    }

    #[tokio::test]
    async fn test_run_tasks() {
        let mut manager = TaskManager::new();
        let task1 = Task::new(
            "task1".to_string(),
            Version::parse("1.0.0").unwrap(),
            Some(MultiCommand::Function(Function::new(|_cmd_resp| {
                println!("Executing task1");
            }))),
            Vec::new(),
        );
        let task2 = Task::new(
            "task2".to_string(),
            Version::parse("1.0.0").unwrap(),
            Some(MultiCommand::Function(Function::new(|_cmd_resp| {
                println!("Executing task2");
            }))),
            vec![Dependency::new(
                "task1".to_string(),
                VersionReq::parse(">=1.0.0").unwrap(),
            )],
        );
        manager.add_task(task1).await;
        manager.add_task(task2).await;

        let mut runner = TaskRunner::try_from(manager).unwrap();
        let result = runner.run().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_shell_command() {
        let mut command = ShellCommand::new("echo");
        command.arg("Hello, world!");

        let handles: Vec<JoinHandle<Result<CommandResponse, DynTracerError>>> =
            command.exec(Some("Hello world test".to_string())).await;
        let responses = join_all(handles).await;

        for response in responses {
            let response = response.unwrap().unwrap();
            assert_eq!(response.status, 0);
            assert!(response.output.iter().any(|output| match output {
                Output::Out(s) => s.contains("Hello, world!"),
                _ => false,
            }));
        }
    }

    #[tokio::test]
    async fn test_dependency_ordering() {
        let mut manager = TaskManager::new();

        let task1 = Task::new(
            "task1".to_string(),
            Version::parse("1.0.0").unwrap(),
            Some(MultiCommand::Function(Function::new(|cmd_resp| {
                println!("Executing task1");
                cmd_resp.out("Task 1 output");
            }))),
            Vec::new(),
        );

        let task2 = Task::new(
            "task2".to_string(),
            Version::parse("1.0.0").unwrap(),
            Some(MultiCommand::Function(Function::new(|cmd_resp| {
                println!("Executing task2");
                cmd_resp.out("Task 2 output");
            }))),
            vec![Dependency::new(
                "task1".to_string(),
                VersionReq::parse(">=1.0.0").unwrap(),
            )],
        );

        let task3 = Task::new(
            "task3".to_string(),
            Version::parse("1.0.0").unwrap(),
            Some(MultiCommand::Function(Function::new(|cmd_resp| {
                println!("Executing task3");
                cmd_resp.out("Task 3 output");
            }))),
            vec![Dependency::new(
                "task2".to_string(),
                VersionReq::parse(">=1.0.0").unwrap(),
            )],
        );

        manager.add_task(task1).await;
        manager.add_task(task2).await;
        manager.add_task(task3).await;

        let mut runner = TaskRunner::try_from(manager).unwrap();

        println!("{:?}", runner);

        let res = runner.run().await;
        // Capturing the output to check the order of execution

        assert!(res.is_ok());

        // Ensure the order of execution
        let output = res
            .unwrap()
            .into_iter()
            .map(|t| t.to_vec().join(""))
            .collect::<Vec<String>>();

        println!("{:?}", output);
        let expected_output = ["Task 3 output", "Task 2 output", "Task 1 output"];

        for expected in expected_output.iter().enumerate() {
            assert_eq!(output[expected.0], expected.1.to_string());
        }
    }
}
