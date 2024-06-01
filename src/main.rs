use wols::{task, Manager, Node, Task, Worker};

fn main() {
    let task_name = "Task-1".to_string();
    let image_name = "Image-1".to_string();
    let worker_name = "Worker-1".to_string();
    let node_name = "Node-1".to_string();

    let task = Task::new(task_name, image_name, 1024, 1);
    let task_event = task::Event::new(task.clone());

    println!("task: {task:?}");
    println!("task: {task_event:?}");

    let worker = Worker::new(worker_name);
    println!("worker: {worker:?}");
    worker.collect_stats();
    worker.run_task();
    worker.start_task();
    worker.stop_task();

    let manager = Manager::new(worker.name().clone());

    println!("manager: {manager:?}");
    manager.select_worker();
    manager.update_tasks();
    manager.send_work();

    let node = Node::new(node_name);
    println!("node: {node:?}");
}
