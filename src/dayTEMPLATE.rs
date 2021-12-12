use std::cell::Cell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/dayTEMPLATE.txt";


fn read_data(reader: BufReader<&File>) -> Vec<String> {
    let data: Vec<String> = Vec::new();
    data
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let data = read_data(BufReader::new(&file));
    0
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let data = read_data(BufReader::new(&file));
    0
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
    static TEST_DATA_FILEPATH: &str = "data/dayTEMPLATE_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(0, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(0, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(0, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(0, task2_run(DATA_FILEPATH))
    }
}
