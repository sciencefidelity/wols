use crate::task::{self, Task};
use crate::Queue;

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use uuid::Uuid;

/// Keeps track of all the jobs in the system and is responsible for
/// making good scheduling decisions, as well as providing answers to
/// user queries about job and worker statuses.
///
/// Keeps track of all worker metrics, such as the number of jos a worker
/// is currently running, how much memory it has available, how much load
/// the CPU is under, and how much disk space is free.
///
/// # Requirements
///
/// - Accept requests from users to start and stop tasks
/// - Schedule tasks onto worker machines
/// - Keep track of tasks, their states, and the machine on which they run
#[derive(Debug)]
pub struct Manager {
    /// Tasks pending, newly submitted tasks will be added here
    pending: Queue,
    /// DB of active tasks
    task_db: HashMap<String, Rc<Task>>,
    /// DB of active events
    event_db: HashMap<String, Rc<task::Event>>,
    /// Holds names of all workers currently in the cluster
    workers: Vec<String>,
    /// Map of jobs assigned to each worker
    worker_task_map: HashMap<String, Uuid>,
    /// Map of tasks assigned to each worker
    task_worker_map: HashMap<Uuid, String>,
}

impl Manager {
    #[must_use]
    pub fn new(worker_name: String) -> Self {
        Self {
            pending: VecDeque::new(),
            task_db: HashMap::new(),
            event_db: HashMap::new(),
            workers: vec![worker_name],
            worker_task_map: HashMap::new(),
            task_worker_map: HashMap::new(),
        }
    }

    /// Assesses requirements specified in a Task and evaluates to resources
    /// available in the pool of workers to see which worker is best suited
    /// to run the task
    pub fn select_worker(&self) {
        println!("I will select and appropriate worker");
    }

    /// Keeps track of tasks, their states and the machine on which they run
    /// by calling `collect_stats()`
    pub fn update_tasks(&self) {
        println!("I will update tasks");
    }

    /// Sends tasks to workers
    pub fn send_work(&self) {
        println!("I will send work to workers");
    }
}
