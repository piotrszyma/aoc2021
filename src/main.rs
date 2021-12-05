use std::collections::HashMap;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

type Task = fn() -> i32;

fn main() {
    let tasks: [(&str, Task); 10] = [
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
    ];
    let latest_task = tasks.last().unwrap();
    let tasks_registry = HashMap::from(tasks);

    let task_id = match env::args().nth(1) {
        Some(filepath) => filepath,
        _ => (*latest_task).0.to_string(),
    };

    let result = match tasks_registry.get(&*task_id) {
        Some(func) => {
            println!("Running task_id: {}", task_id);
            func()
        }
        _ => panic!("Invalid task_id: {}", task_id),
    };
    println!("result: {}", result)
}
