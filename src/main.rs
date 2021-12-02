use std::collections::HashMap;
use std::env;

mod day1_task1;
mod day1_task2;

type Task = fn() -> ();

fn main() {
    let mut tasks_registry = HashMap::<&str, Task>::new();
    tasks_registry.insert("day1_task1", day1_task1::run);
    tasks_registry.insert("day1_task2", day1_task2::run);

    let task_id = match env::args().nth(1) {
        Some(filepath) => filepath,
        _ => panic!("First arg must be task id."),
    };

    match tasks_registry.get(&*task_id) {
        Some(func) => func(),
        _ => panic!("Invalid task_id={}", task_id)
    }

}
