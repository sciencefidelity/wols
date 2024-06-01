#![allow(unused)]

use std::collections::VecDeque;

mod manager;
pub use manager::Manager;

mod node;
pub use node::Node;

mod scheduler;

mod task;
pub use task::{State, Task, TaskEvent};

mod worker;
pub use worker::Worker;

type Queue = VecDeque<Task>;
