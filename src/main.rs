use std::collections::HashMap;
use std::env;

mod day1_task1;
mod day1_task2;
mod day2_task1;
mod day2_task2;
mod day3_task1;
mod day3_task2;
mod day4_task1;
mod day4_task2;
mod day5_task1;
mod day5_task2;

type Task = fn() -> ();

fn main() {
    let tasks: [(&str, Task); 10] = [
        ("day1_task1", day1_task1::run),
        ("day1_task2", day1_task2::run),
        ("day2_task1", day2_task1::run),
        ("day2_task2", day2_task2::run),
        ("day3_task1", day3_task1::run),
        ("day3_task2", day3_task2::run),
        ("day4_task1", day4_task1::run),
        ("day4_task2", day4_task2::run),
        ("day5_task1", day5_task1::run),
        ("day5_task2", day5_task2::run),
    ];
    let latest_task = tasks.last().unwrap();
    let tasks_registry = HashMap::from(tasks);

    let task_id = match env::args().nth(1) {
        Some(filepath) => filepath,
        _ => (*latest_task).0.to_string(),
    };

    match tasks_registry.get(&*task_id) {
        Some(func) => {
            println!("Running task_id: {}", task_id);
            func();
        }
        _ => panic!("Invalid task_id: {}", task_id),
    }
}
