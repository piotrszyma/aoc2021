use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day15.txt";

type Row = Vec<i8>;

struct Cave {
    rows: Vec<Row>
}

impl Cave {
    fn in_range(&self, row_idx: usize, col_idx: usize) -> bool {
        match self.rows.get(row_idx) {
            Some(row) => {
                match row.get(col_idx) {
                    Some(_) => true,
                    _ => false,
                }
            },
            _ => false
        }
    }
    
    fn is_bottom_right_position(&self, row_idx: usize, col_idx: usize) -> bool {
        let is_bottom_row = self.rows.len() - 1 == row_idx;
        if !is_bottom_row {
            return false;
        }
        
        let row = self.rows.first().unwrap();
        
        let is_rightmost_col = row.len() - 1 == col_idx;

        return is_rightmost_col;
    }

    fn get_risk(&self, row_idx: usize, col_idx: usize) -> Option<i8> {
        match self.rows.get(row_idx) {
            Some(row) => {
                match row.get(col_idx) {
                    Some(value) => Some(*value),
                    _ => None,
                }
            },
            _ => None
        }
    }
}

fn read_data(reader: BufReader<&File>) -> Cave {
    let mut rows = Vec::new();
    for line in reader.lines() {
        let mut row = Row::new();
        for c in line.unwrap().chars() {
            let risk: i8 = c.to_string().parse().unwrap();
            row.push(risk);
        }
        rows.push(row);
    };
    Cave {
        rows
    }
}

fn find_least_risky(row_idx: usize, col_idx: usize, visited: &HashSet::<(usize, usize)>, cave: &Cave) -> Option<i64> {
    if visited.contains(&(row_idx, col_idx)) {
        return None
    }

    if !cave.in_range(row_idx, col_idx) {
        return None
    }

    if cave.is_bottom_right_position(row_idx, col_idx) {
        let risk = cave.get_risk(row_idx, col_idx);
        return Some(risk.expect("Expected position to be valid.").into());
    }

    let current_risk = cave.get_risk(row_idx, col_idx).expect("Expected position to be valid.");
    let mut options: Vec<i64> = Vec::new();
    let mut next_visited = visited.clone();
    next_visited.insert((row_idx, col_idx));

    // Either move top.
    // if row_idx > 0 {

    //     let next_row_idx = row_idx - 1;
    //     let next_col_idx = col_idx;

    //     match find_least_risky(next_row_idx, next_col_idx, &next_visited, cave) {
    //         Some(value) => {
    //             options.push(value);
    //         },
    //         _ => (),
    //     }
    // }

    // Or move right.
    {
        let next_row_idx = row_idx;
        let next_col_idx = col_idx + 1;

        match find_least_risky(next_row_idx, next_col_idx, &next_visited, cave) {
            Some(value) => {
                options.push(value);
            },
            _ => (),
        }
    }

    // Or move bottom.
    {
        let next_row_idx = row_idx + 1;
        let next_col_idx = col_idx;

        match find_least_risky(next_row_idx, next_col_idx, &next_visited, cave) {
            Some(value) => {
                options.push(value);
            },
            _ => (),
        }
    }

    // Or move left.
    // if col_idx > 0 {
    //     let next_row_idx = row_idx;
    //     let next_col_idx = col_idx - 1;

    //     match find_least_risky(next_row_idx, next_col_idx, &next_visited, cave) {
    //         Some(value) => {
    //             options.push(value);
    //         },
    //         _ => (),
    //     }
    // }
    
    options.sort();
    let lowest_next_risk = options.first();
    match lowest_next_risk {
        Some(lowest_next_risk) => Some(lowest_next_risk + current_risk as i64),
        None => None,
    }
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let cave = read_data(BufReader::new(&file));
    let least_risky = find_least_risky(0, 0, &HashSet::new(), &cave);

    match least_risky {
        Some(value) => value - cave.get_risk(0, 0).unwrap() as i64,
        None => panic!("Failed to find least risky route.")
    }
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
    static TEST_DATA_FILEPATH: &str = "data/day15_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(40, task1_run(TEST_DATA_FILEPATH))
    }

    // #[test]
    // fn task1() {
    //     assert_eq!(0, task1_run(DATA_FILEPATH));
    // }

    // #[test]
    // fn task2_test_data() {
    //     assert_eq!(0, task2_run(TEST_DATA_FILEPATH))
    // }

    // #[test]
    // fn task2() {
    //     assert_eq!(0, task2_run(DATA_FILEPATH))
    // }
}
