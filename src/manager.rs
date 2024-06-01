use std::collections::{HashMap, VecDeque};

use uuid::Uuid;

use crate::{
    task::{Task, TaskEvent},
    Queue,
};

#[derive(Debug)]
pub struct Manager {
    pending: Queue,
    task_db: HashMap<String, Task>,
    event_db: HashMap<String, TaskEvent>,
    workers: Vec<String>,
    worker_task_map: Option<HashMap<String, Uuid>>,
    task_worker_map: Option<HashMap<Uuid, String>>,
}

impl Manager {
    pub fn new(worker_name: String) -> Self {
        Manager {
            pending: VecDeque::new(),
            task_db: HashMap::new(),
            event_db: HashMap::new(),
            workers: vec![worker_name],
            worker_task_map: None,
            task_worker_map: None,
        }
    }

    pub fn select_worker() {
        println!("I will select and appropriate worker");
    }

    pub fn update_tasks() {
        println!("I will update tasks");
    }

    pub fn send_work() {
        println!("I will send work to workers");
    }
}
