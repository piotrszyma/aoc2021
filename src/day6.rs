use crate::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day6.txt";

type Cache = HashMap<(i64, i64), i64>;

fn read_initial_fishes(reader: BufReader<&std::fs::File>) -> Vec<i64> {
    let mut fishes: Vec<i64> = Vec::new();
    for line in reader.lines() {
        for number in line.unwrap().split(',') {
            fishes.push(number.parse().unwrap())
        }
    }
    fishes
}

/// Recursively counts fish children population.
fn single_fish_after(fish_state: i64, days_left: i64, cache: &mut Cache) -> i64 {
    let cached_value = cache.get(&(fish_state, days_left));
    match cached_value {
        Some(value) => *value,
        None => {
            let result = if days_left <= 0 {
                0
            }  else if fish_state == 0 {
                1 + single_fish_after(6, days_left - 1, cache)
                    + single_fish_after(8, days_left - 1, cache)
            } else {
                single_fish_after(0, days_left - fish_state, cache)
            };
            cache.insert((fish_state, days_left), result);
            result
        }
    }
}

fn fishes_count_after(state: Vec<i64>, days_left: i64) -> i64 {
    let mut cache: Cache = Cache::new();
    let initial_sum = state.len() as i64;
    let children_sum: i64 = state.into_iter().map(|f| single_fish_after(f, days_left, &mut cache)).sum();
    initial_sum + children_sum
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let initial = read_initial_fishes(BufReader::new(&file));
    let result = fishes_count_after(initial, 80);
    result
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let initial = read_initial_fishes(BufReader::new(&file));
    let result = fishes_count_after(initial, 256);
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
    static TEST_DATA_FILEPATH: &str = "data/day6_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(5934, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(358214, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(26984457539, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(1622533344325, task2_run(DATA_FILEPATH))
    }
}
