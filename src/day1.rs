use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn count_increasing(nums: Vec<i32>) -> i32 {
    nums.windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum()
}

fn sum_windows_of_three(nums: Vec<i32>) -> Vec<i32> {
    nums.windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect()
}

fn count_increasing_of_three(nums: Vec<i32>) -> i32 {
    sum_windows_of_three(nums)
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum()
}

fn read_nums(reader: BufReader<&std::fs::File>) -> Vec<i32> {
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn task1_run(input_path: &str) -> i32 {
    let file = File::open(input_path).unwrap();
    let nums = read_nums(BufReader::new(&file));
    count_increasing(nums)
}

fn task2_run(input_path: &str) -> i32 {
    let file = File::open(input_path).unwrap();
    let nums = read_nums(BufReader::new(&file));
    count_increasing_of_three(nums)
}

pub fn task1() -> i32 {
    task1_run("data/day1_task1.txt")
}

pub fn task2() -> i32 {
    task2_run("data/day1_task1.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1() {
        assert_eq!(1226, task1_run("data/day1_task1.txt"))
    }

    #[test]
    fn task2() {
        assert_eq!(1252, task2_run("data/day1_task1.txt"))
    }
}
