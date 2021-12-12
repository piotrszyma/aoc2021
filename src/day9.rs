use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day9.txt";

type Row = Vec<i8>;

#[derive(Debug)]
struct HeightMap {
    rows: Vec<Row>,
}

impl HeightMap {
    fn get_point_value(&self, coords: (i8, i8)) -> Option<&i8> {
        let (row_idx, col_idx) = coords;
        match self.rows.get(row_idx as usize) {
            Some(row) => row.get(col_idx as usize),
            _ => None
        }
    }
}

fn read_height_map(reader: BufReader<&std::fs::File>) -> HeightMap {
    let mut rows: Vec<Row> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split("").collect();
        println!("line={:?}", line);
        let line: Row = line
            .iter()
            .filter(|&&c| c != "")
            .map(|c| {
                let value: i8 = c.parse().unwrap();
                value
            })
            .collect();
        rows.push(line)
    }
    HeightMap { rows }
}

fn get_low_points(map: &HeightMap) -> Vec<(u8, u8)> {
    let mut low_points: Vec<(u8, u8)> = Vec::new();
    for row_idx in 0..map.rows.len() {
        for col_idx in 0..map.rows.first().unwrap().len() {
            let value = map
                .rows
                .get(row_idx)
                .expect(&format!(
                    "Expected to find a row with id row_idx: {}",
                    row_idx
                ))
                .get(col_idx)
                .expect(&format!(
                    "Expected to find a col with id col_idx: {}",
                    col_idx
                ));
            let mut neighbour_values: Vec<i8> = Vec::new();

            if col_idx > 0 {
                let value_before = map.rows.get(row_idx).unwrap().get(col_idx - 1);
                match value_before {
                    Some(value) => neighbour_values.push(*value),
                    _ => (),
                }
            }

            let value_after = map.rows.get(row_idx).unwrap().get(col_idx + 1);
            match value_after {
                Some(value) => neighbour_values.push(*value),
                _ => (),
            }

            if row_idx > 0 {
                let row_above = map.rows.get(row_idx - 1);
                match row_above {
                    Some(row) => neighbour_values.push(*row.get(col_idx).unwrap()),
                    _ => (),
                };
            }

            let row_below = map.rows.get(row_idx + 1);
            match row_below {
                Some(row) => neighbour_values.push(*row.get(col_idx).unwrap()),
                _ => (),
            };

            let is_low_point = neighbour_values.iter().all(|neighbour| neighbour > value);
            if is_low_point {
                low_points.push((row_idx.try_into().unwrap(), col_idx.try_into().unwrap()));
            }
        }
    }
    low_points
}



fn get_low_points_values(map: HeightMap) -> Vec<i8> {
    get_low_points(&map).into_iter().map(|(row_idx, col_idx)| {
        let value = map.rows.get(row_idx as usize).unwrap().get(col_idx as usize).unwrap();
        *value
    }).collect()
}

fn get_size_of_basin_internal(starting_point: (i8, i8), map: &HeightMap, visited: &mut HashSet::<(i8, i8)>) -> i64 {

    // up
    // right
    // bottom
    // left

    if visited.contains(&starting_point) {
        return 0
    }

    match map.get_point_value(starting_point) {
        Some(point_value) => {
            visited.insert(starting_point);

            if *point_value == 9 {
                return 0
            }

            let mut value: i64 = 1;

            let (x, y) = starting_point;
            value += get_size_of_basin_internal((x, y + 1), map, visited);
            value += get_size_of_basin_internal((x + 1, y), map, visited);
            value += get_size_of_basin_internal((x, y - 1), map, visited);
            value += get_size_of_basin_internal((x - 1, y), map, visited);
            value
        }
        None => 0,
    }
}

fn get_size_of_basin(basin_start: (i8, i8), map: &HeightMap) -> i64 {
    let mut visited = HashSet::<(i8, i8)>::new();
    get_size_of_basin_internal(basin_start, map, &mut visited)
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let map = read_height_map(BufReader::new(&file));
    let low_points = get_low_points_values(map);
    low_points.into_iter().map(|p| {
        let value: i64 = i64::from(p) + 1;
        value
    }).sum()
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let map = read_height_map(BufReader::new(&file));
    let low_points = get_low_points(&map);
    let mut low_points_sizes: Vec<i64> = low_points.into_iter().map(|p| {
        let (x, y) = p;
        get_size_of_basin((x as i8, y as i8), &map)
    }).collect();
    low_points_sizes.sort();
    low_points_sizes.reverse();
    let top_three_sizes = &low_points_sizes[..=2];
    top_three_sizes.iter().product()
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
    static TEST_DATA_FILEPATH: &str = "data/day9_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(15, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(423, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(1134, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(1198704, task2_run(DATA_FILEPATH))
    }
}
