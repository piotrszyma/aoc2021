use std::collections::HashMap;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

type Task = fn() -> i32;

struct TaskRegistry {
    tasks: HashMap<String, Task>,
    latest_task_id: String,
}

impl TaskRegistry {
    fn new(tasks_init: &[(&str, Task)]) -> Self {
        let mut tasks = HashMap::<String, Task>::new();
        for (task_id, task_fn) in tasks_init {
            tasks.insert(task_id.to_string(), *task_fn);
        }
        let latest_task_id = tasks_init
            .last()
            .expect("tasks_init should have at least one task")
            .0
            .to_string();
        TaskRegistry {
            tasks,
            latest_task_id,
        }
    }

    fn get(&self, task_id: &str) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    fn latest_task_id(&self) -> &str {
        &self.latest_task_id
    }
}

fn main() {
    let tasks_registry = TaskRegistry::new(&[
        ("day1_task1", day1::task1),
        ("day1_task2", day1::task2),
        ("day2_task1", day2::task1),
        ("day2_task2", day2::task2),
        ("day3_task1", day3::task1),
        ("day3_task2", day3::task2),
        ("day4_task1", day4::task1),
        ("day4_task2", day4::task2),
        ("day5_task1", day5::task1),
        ("day5_task2", day5::task2),
    ]);

    let task_id = match env::args().nth(1) {
        Some(task_id) => task_id,
        _ => tasks_registry.latest_task_id().to_string()
    };

    let result = match tasks_registry.get(&task_id) {
        Some(func) => {
            println!("Running task_id: {}", task_id);
            func()
        }
        _ => panic!("Invalid task_id: {}", task_id),
    };

    println!("result: {}", result)
}
