use wols::{Manager, Node, Task, TaskEvent, Worker};

fn main() {
    let task_name = "Task-1".to_string();
    let image_name = "Image-1".to_string();
    let worker_name = "Worker-1".to_string();
    let node_name = "Node-1".to_string();

    let task = Task::new(task_name, image_name);
    println!("task: {:?}", task);

    let task_event = TaskEvent::new(task);

    println!("task: {:?}", task_event);

    let worker = Worker::new(worker_name);
    println!("worker: {:?}", worker);
    Worker::collect_stats();
    Worker::run_task();
    Worker::start_task();
    Worker::stop_task();

    let manager = Manager::new(worker.name);

    println!("manager: {:?}", manager);
    Manager::select_worker();
    Manager::update_tasks();
    Manager::send_work();

    let node = Node::new(node_name);
    println!("node: {:?}", node);
}
