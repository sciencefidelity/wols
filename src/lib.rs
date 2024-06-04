#![allow(unused)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

mod connection;
pub use connection::{Client, Config, Docker, DockerResult};

mod manager;
pub use manager::Manager;

mod node;
pub use node::Node;

mod scheduler;

pub mod task;
pub use task::{State, Task};

mod worker;
pub use worker::Worker;

type Queue = VecDeque<Rc<Task>>;
type PortSet = HashMap<String, HashMap<(), ()>>;
