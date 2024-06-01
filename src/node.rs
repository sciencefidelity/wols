/// Represents a machine in the cluster
#[derive(Debug)]
pub struct Node {
    /// Name of the node
    name: String,
    /// Address of the node for the manager to send tasks to
    ip: String,
    cores: u16,
    memory: u16,
    memory_allocated: Option<u8>,
    disk: u8,
    disk_allocated: Option<u8>,
    role: String,
    task_count: Option<u8>,
}

impl Node {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            ip: "192.168.1.1".to_string(),
            cores: 4,
            memory: 1024,
            memory_allocated: None,
            disk: 25,
            disk_allocated: None,
            role: "worker".to_string(),
            task_count: None,
        }
    }
}
