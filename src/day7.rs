use crate::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day7.txt";

type FuelCalcFn = fn(crabs: &Vec<i64>, target_position: i64) -> i64;

fn read_initial_crabs_positions(reader: BufReader<&std::fs::File>) -> Vec<i64> {
    let mut crabs: Vec<i64> = Vec::new();
    for line in reader.lines() {
        for number in line.unwrap().split(',') {
            crabs.push(number.parse().unwrap())
        }
    }
    crabs
}

fn find_most_common(crabs: &Vec<i64>) -> i64 {
    let mut counts = HashMap::new();
    for crab in crabs {
        *counts.entry(crab).or_insert(0) += 1;
    }
    let mut most_common = (crabs.first().unwrap(), 0);
    for (k, v) in counts {
        if v > most_common.1 {
            most_common = (k, v)
        }
    }
    most_common.1
}

fn calc_fuel_needed(crabs: &Vec<i64>, target_position: i64) -> i64 {
    crabs.iter().map(|c| (target_position - c).abs()).sum()
}

fn calc_fuel_needed_more_expensive(crabs: &Vec<i64>, target_position: i64) -> i64 {
    crabs
        .iter()
        .map(|c| {
            let dist = (target_position - c).abs();
            let range = 1..(dist + 1);
            let sum: i64 = range.sum();
            sum
        })
        .sum()
}

fn find_best_position(
    crabs: &Vec<i64>,
    start_position: i64,
    fuel_needed: i64,
    fuel_calc: FuelCalcFn,
) -> i64 {
    let fuel_needed_one_up = fuel_calc(crabs, start_position + 1);
    let fuel_needed_one_down = fuel_calc(crabs, start_position - 1);

    if fuel_needed_one_down < fuel_needed {
        find_best_position(crabs, start_position - 1, fuel_needed_one_down, fuel_calc)
    } else if fuel_needed_one_up < fuel_needed {
        find_best_position(crabs, start_position + 1, fuel_needed_one_up, fuel_calc)
    } else {
        fuel_needed
    }
}

fn find_least_fuel_to_align(crabs: Vec<i64>, fuel_calc: FuelCalcFn) -> i64 {
    let start_position = find_most_common(&crabs);
    let fuel_needed = fuel_calc(&crabs, start_position);
    println!("start_position={}, fuel_needed={}", start_position, fuel_needed);
    find_best_position(&crabs, start_position, fuel_needed, fuel_calc)
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let initial = read_initial_crabs_positions(BufReader::new(&file));
    let result = find_least_fuel_to_align(initial, calc_fuel_needed);
    result
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let initial = read_initial_crabs_positions(BufReader::new(&file));
    let result = find_least_fuel_to_align(initial, calc_fuel_needed_more_expensive);
    result
}

pub fn task1() -> i64 {
    task1_run(DATA_FILEPATH)
}

pub fn task2() -> i64 {
    task2_run(DATA_FILEPATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA_FILEPATH: &str = "data/day7_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(37, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(352331, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(168, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(99266250, task2_run(DATA_FILEPATH))
    }

    #[test]
    fn calc_fuel_more_expensive() {
        assert_eq!(calc_fuel_needed_more_expensive(&Vec::from([1]), 5), 10)
    }
}
