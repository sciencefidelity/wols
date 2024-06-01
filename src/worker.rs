use crate::{task::Task, Queue};
use std::collections::{HashMap, VecDeque};

use uuid::Uuid;

#[derive(Debug)]
pub struct Worker {
    pub name: String,
    queue: Queue,
    db: Option<HashMap<Uuid, Task>>,
    task_count: Option<u32>,
}

impl Worker {
    pub fn new(name: String) -> Self {
        Worker {
            name,
            queue: VecDeque::new(),
            db: None,
            task_count: None,
        }
    }

    pub fn collect_stats() {
        println!("I will collect stats");
    }

    pub fn run_task() {
        println!("I will start or stop a task");
    }

    pub fn start_task() {
        println!("I will start a task");
    }

    pub fn stop_task() {
        println!("I will stop a task");
    }
}
