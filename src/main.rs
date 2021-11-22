use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = args.get(1).expect("Failed to read first program argument.");

    let content = fs::read_to_string(filepath).expect("Something went wrong when reading file");

    let nums: Vec<i32> = content
        .split('\n')
        .map(|s| s.parse().expect("failed to parse"))
        .collect();

    for num in nums {
        println!("{}", num)
    }
}
