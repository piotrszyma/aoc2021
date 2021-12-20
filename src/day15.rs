use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day15.txt";

type Row = Vec<i64>;

#[derive(Debug)]
struct Cave {
    rows: Vec<Row>,
}

impl Cave {
    fn get_risk(&self, row_idx: usize, col_idx: usize) -> Option<i64> {
        match self.rows.get(row_idx) {
            Some(row) => match row.get(col_idx) {
                Some(value) => Some(*value),
                _ => None,
            },
            _ => None,
        }
    }

    fn rows_len(&self) -> usize {
        self.rows.len()
    }

    fn cols_len(&self) -> usize {
        self.rows.first().unwrap().len()
    }
}

fn read_data(reader: BufReader<&File>) -> Cave {
    let mut rows = Vec::new();
    for line in reader.lines() {
        let mut row = Row::new();
        for c in line.unwrap().chars() {
            let risk: i64 = c.to_string().parse().unwrap();
            row.push(risk);
        }
        rows.push(row);
    }
    Cave { rows }
}

fn read_data_expanded(reader: BufReader<&File>) -> Cave {
    let mut rows = Vec::new();
    let mut chunk_size: i8 = -1;
    for line in reader.lines() {
        let mut row = Row::new();
        for c in line.unwrap().chars() {
            let risk: i64 = c.to_string().parse().unwrap();
            row.push(risk);
        }
        chunk_size = row.len() as i8;
        let mut row_mul_5 = Vec::new();
        for _ in 0..=4 {
            row_mul_5.extend(row.clone())
        }

        rows.push(row_mul_5);
    }

    let mut rows_mul_5 = Vec::new();
    for _ in 0..=4 {
        rows_mul_5.extend(rows.clone());
    }

    let rows = rows_mul_5;
    let mut updated_rows: Vec<Row> = Vec::new();
    for (row_idx, row) in rows.iter().enumerate() {
        let mut updated_row: Row = Row::new();
        for (col_idx, cell) in row.iter().enumerate() {
            let big_row_idx = (row_idx / chunk_size as usize) as i64;
            let big_col_idx = (col_idx / chunk_size as usize) as i64;

            if big_row_idx == 0 && big_col_idx == 0 {
                updated_row.push(*cell);
                continue
            }

            if big_col_idx == 0 {
                let prev_value = updated_rows[row_idx - 10][col_idx];
                let new_value = if prev_value == 9 { 1 } else { prev_value + 1 };
                updated_row.push(new_value);
                continue
            }
        }

        for _ in 0..40 {
            let idx = updated_row.len() - 10;
            let element = updated_row.get(idx).unwrap();
            let new_element = if element == &9 {
                1
            } else {
                element + 1
            };
            updated_row.push(new_element)
        }

        updated_rows.push(updated_row);
    }
    // println!("rows={:?}", rows);
    // for row in updated_rows.iter() {
    //     println!("row={:?}", row);
    // }
    // println!("chunk_size={}", chunk_size);
    // println!("rows.len()={:?}", updated_rows.len());
    // println!("rows.first().len()={:?}", updated_rows.first().unwrap().len());
    Cave { rows: updated_rows }
    // let cave = Cave { rows };

    // let mut vec_of_vec_of_caves: Vec<Vec<Cave>> = Vec::new();

    // let mut vec_of_caves: Vec<Cave> = Vec::new();
    // vec_of_caves.push(cave);

    // for _ in 1..=4 {
    //     vec_of_caves.push(copy_inc(vec_of_caves.last().unwrap()));
    // };

    // vec_of_vec_of_caves.push(vec_of_caves);

    // for _ in 1..=4 {
    //     let last_vec = vec_of_vec_of_caves.last().unwrap();
    //     let mut new_vec_of_caves: Vec<Cave> = Vec::new();

    //     for item in last_vec {
    //         new_vec_of_caves.push(copy_inc(item));
    //     }

    //     vec_of_vec_of_caves.push(new_vec_of_caves)
    // }

    // println!("vec_of_vec_of_caves={:?}", vec_of_vec_of_caves);

    // let final_cave_rows: Vec<Row> = Vec::new();

    // for vec_of_caves in vec_of_vec_of_caves {
    //     let row =
    // }
}

fn find_least_risky(cave: &Cave) -> i64 {
    let mut rows: Vec<Row> = Vec::new();
    for _ in 0..cave.rows_len() {
        let mut row = Row::new();
        for _ in 0..cave.cols_len() {
            row.push(0);
        }
        rows.push(row);
    }

    for row_idx in 0..cave.rows_len() {
        for col_idx in 0..cave.cols_len() {
            let mut options: Vec<i64> = Vec::new();
            if col_idx > 0 {
                let risk_to_leave_left = cave.get_risk(row_idx, col_idx - 1).unwrap();
                let risk_to_get_to_the_left = rows[row_idx][col_idx - 1];
                options.push(risk_to_get_to_the_left + risk_to_leave_left);
            }

            if row_idx > 0 {
                let risk_to_leave_top = cave.get_risk(row_idx - 1, col_idx).unwrap();
                let risk_to_get_to_top = rows[row_idx - 1][col_idx];
                options.push(risk_to_get_to_top + risk_to_leave_top);
            }

            options.sort();

            // if (options.len() == 2 && options.first().unwrap() == options.last().unwrap()) {
            //     options.reverse();
            // }

            let risk_to_get_to_this = options.first();

            match risk_to_get_to_this {
                Some(risk_to_get_to_this) => {
                    rows[row_idx][col_idx] = *risk_to_get_to_this;
                }
                None => (),
            }
        }
    }


    println!("risk rows={:?}", rows);

    let last_row = rows.last().unwrap();

    let risk_to_get_to_last = last_row.last().unwrap();

    return *risk_to_get_to_last;
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let cave = read_data(BufReader::new(&file));
    find_least_risky(&cave)
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let cave = read_data_expanded(BufReader::new(&file));
    find_least_risky(&cave)
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

    #[test]
    fn task1() {
        assert_eq!(363, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(315, task2_run(TEST_DATA_FILEPATH))
    }

    // #[test]
    // fn task2() {
    //     assert_eq!(0, task2_run(DATA_FILEPATH))
    // }
}
