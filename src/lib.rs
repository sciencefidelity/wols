#![allow(unused)]

use std::collections::{HashMap, VecDeque};
use std::hash::RandomState;
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
#[allow(clippy::zero_sized_map_values)]
type PortSet = HashMap<String, HashMap<(), ()>>;
