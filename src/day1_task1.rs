use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn count_increasing(nums: Vec<i32>) -> i32 {
    nums.windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum()
}

fn read_nums(reader: BufReader<&std::fs::File>) -> Vec<i32> {
    reader.lines().map(|l| l.unwrap().parse().unwrap()).collect()
}

pub fn run() {
    let file = File::open("data/day1_task1.txt").unwrap();
    let nums = read_nums(BufReader::new(&file));
    println!("result={}", count_increasing(nums))
}
