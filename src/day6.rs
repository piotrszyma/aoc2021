use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day6.txt";
static TEST_DATA_FILEPATH: &str = "data/day6_test.txt";

fn read_initial_fishes(reader: BufReader<&std::fs::File>) -> Vec<u64> {
    let mut fishes: Vec<u64> = Vec::new();
    for line in reader.lines() {
        for number in line.unwrap().split(',') {
            fishes.push(number.parse().unwrap())
        }
    }
    fishes
}

fn fishes_count_after(state: Vec<u64>, days: u64) -> u64 {
    if days == 0 {
        return state.len() as u64;
    }

    let mut next_day_state = Vec::new();
    for fish_state in state {
        if fish_state == 0 {
            next_day_state.push(8);
            next_day_state.push(6);
        } else {
            next_day_state.push(fish_state - 1);
        }
    };

    fishes_count_after(next_day_state, days - 1)
}

pub fn task1_run(path: &str) -> u64 {
    let file = File::open(path).unwrap();
    let initial = read_initial_fishes(BufReader::new(&file));
    let result = fishes_count_after(initial, 80);
    result
}

pub fn task2_run(path: &str) -> u64 {
    let file = File::open(path).unwrap();
    let initial = read_initial_fishes(BufReader::new(&file));
    let result = fishes_count_after(initial, 80);
    result
}

pub fn task1() -> u64 {
    task1_run(DATA_FILEPATH)
}

pub fn task2() -> u64 {
    task2_run(DATA_FILEPATH)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // fn task2() {
    //     assert_eq!(24164, task2_run("data/day5.txt"))
    // }
}
