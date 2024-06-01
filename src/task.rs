use std::time::{self, Instant};
use std::{collections::HashSet, rc::Rc};
use uuid::Uuid;

type PortSet = HashSet<u16>;

/// States representing the life-cycle of a task.
#[derive(Debug)]
pub enum State {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
}

/// A scheduled task in the form of a Docker container deployed on a cluster
#[derive(Debug)]
pub struct Task {
    id: Uuid,
    /// User supplied task name
    name: String,
    /// The Task's life-cycle
    state: State,
    /// The Docker image used by the Task
    image: String,
    /// Tracks the memory allocated to the container
    memory: u16,
    disk: u16,
    exposed_ports: PortSet,
    port_bindings: Vec<String>,
    /// Tells the system what to do in the event a task stops or fails unexpectedly
    restart_policy: Option<String>,
    start_time: Option<Instant>,
    finish_time: Option<Instant>,
}

impl Task {
    pub fn new(name: String, image: String, memory: u16, disk: u16) -> Rc<Task> {
        Rc::new(Task {
            id: Uuid::new_v4(),
            name,
            state: State::Pending,
            image,
            memory,
            disk,
            exposed_ports: HashSet::new(),
            port_bindings: Vec::new(),
            restart_policy: None,
            start_time: None,
            finish_time: None,
        })
    }
}

/// Allows the user to perform actions on a running Task such as stopping the Task
#[derive(Debug)]
pub struct TaskEvent {
    id: Uuid,
    /// The state the task should transition to
    state: State,
    /// The time the event was requested
    timestamp: time::Instant,
    task: Rc<Task>,
}

impl TaskEvent {
    pub fn new(task: Rc<Task>) -> Rc<TaskEvent> {
        Rc::new(TaskEvent {
            id: Uuid::new_v4(),
            state: State::Pending,
            timestamp: time::Instant::now(),
            task,
        })
    }
}
