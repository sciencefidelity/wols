use std::{
    collections::HashSet,
    time::{self, SystemTime},
};
use uuid::Uuid;

type PortSet = HashSet<u16>;

#[derive(Debug)]
pub enum State {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct Task {
    id: Uuid,
    name: String,
    state: State,
    image: String,
    memory: u16,
    disk: u16,
    exposed_ports: Option<PortSet>,
    port_bindings: Option<Vec<String>>,
    restart_policy: Option<String>,
    start_time: Option<SystemTime>,
    finish_time: Option<SystemTime>,
}

impl Task {
    pub fn new(name: String, image: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            name,
            state: State::Pending,
            image,
            memory: 1024,
            disk: 1,
            exposed_ports: None,
            port_bindings: None,
            restart_policy: None,
            start_time: None,
            finish_time: None,
        }
    }
}

#[derive(Debug)]
pub struct TaskEvent {
    id: Uuid,
    state: State,
    timestamp: time::Instant,
    task: Task,
}

impl TaskEvent {
    pub fn new(task: Task) -> Self {
        TaskEvent {
            id: Uuid::new_v4(),
            state: State::Pending,
            timestamp: time::Instant::now(),
            task,
        }
    }
}
