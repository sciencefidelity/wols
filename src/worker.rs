use crate::{task::Task, Queue};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use uuid::Uuid;

/// Providing an API primarily used by the Manager for the purpose of
/// sending tasks asking the Worker to stop tasks and retrieve metrics
/// about the Workers state.
///
/// # Requirements
///
/// - Run tasks as Docker containers
/// - Accept tasks to run from a manager
/// - Provide statistics to for the purpose of scheduling tasks
/// - Keep track of tasks and their state
#[derive(Debug)]
pub struct Worker {
    name: String,
    queue: Queue,
    db: HashMap<Uuid, Rc<Task>>,
    task_count: u32,
}

impl Worker {
    pub fn new(name: String) -> Self {
        Worker {
            name,
            queue: VecDeque::new(),
            db: HashMap::new(),
            task_count: 0,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn collect_stats(&self) {
        println!("I will collect stats");
    }

    pub fn run_task(&self) {
        println!("I will start or stop a task");
    }

    pub fn start_task(&self) {
        println!("I will start a task");
    }

    pub fn stop_task(&self) {
        println!("I will stop a task");
    }
}
