use std::cell::Cell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day11.txt";

#[derive(Debug)]
struct Octopus {
    energy: Cell<i8>,
    flashed: Cell<bool>,
}

type OctopusRow = Vec<Octopus>;

#[derive(Debug)]
struct Grid {
    rows: Vec<OctopusRow>,
}

impl Grid {
    fn get_octopus_at(&self, row_idx: usize, col_idx: usize) -> Option<&Octopus> {
        let row = self.rows.get(row_idx);
        if row.is_none() {
            return None;
        }
        let o = row.unwrap().get(col_idx);
        if o.is_none() {
            return None;
        }

        return Some(o.unwrap());
    }
}

fn read_grid(reader: BufReader<&File>) -> Grid {
    let mut rows: Vec<OctopusRow> = Vec::new();
    for line in reader.lines() {
        let mut row: OctopusRow = Vec::new();
        for c in line.unwrap().chars() {
            row.push(Octopus {
                energy: Cell::new(c.to_string().parse().unwrap()),
                flashed: Cell::new(false),
            })
        }
        rows.push(row);
    }
    Grid { rows }
}

fn inc_and_flash_if_needed(row_idx: usize, col_idx: usize, grid: &Grid) -> i64 {
    let o = grid.get_octopus_at(row_idx, col_idx);
    if o.is_none() {
        return 0;
    }

    let o = o.unwrap();

    if o.flashed.get() {
        return 0;
    }

    if o.energy.get() < 9 {
        o.energy.set(o.energy.get() + 1);
        return 0;
    }

    o.flashed.set(true);
    o.energy.set(0);

    let mut flashes = 1;

    if row_idx > 0 {
        if col_idx > 0 {
            flashes += inc_and_flash_if_needed(row_idx - 1, col_idx - 1, grid);
        }
        flashes += inc_and_flash_if_needed(row_idx - 1, col_idx, grid);
        flashes += inc_and_flash_if_needed(row_idx - 1, col_idx + 1, grid);
    }

    if col_idx > 0 {
        flashes += inc_and_flash_if_needed(row_idx, col_idx - 1, grid);
    }
    flashes += inc_and_flash_if_needed(row_idx, col_idx + 1, grid);

    if col_idx > 0 {
        flashes += inc_and_flash_if_needed(row_idx + 1, col_idx - 1, grid);
    }
    flashes += inc_and_flash_if_needed(row_idx + 1, col_idx, grid);
    flashes += inc_and_flash_if_needed(row_idx + 1, col_idx + 1, grid);

    flashes
}

fn next_step(grid: &Grid) -> i64 {
    let mut flashes = 0;

    for (row_idx, row) in grid.rows.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            flashes += inc_and_flash_if_needed(row_idx, col_idx, grid)
        }
    }

    // Clear flashed flag.
    for row in grid.rows.iter() {
        for o in row {
            o.flashed.set(false);
        }
    }

    flashes
}

fn all_energy_equal_zero(grid: &Grid) -> bool {
    let expected_energy = 0;

    for row in &grid.rows {
        for cell in row.iter() {
            if cell.energy.get() != expected_energy {
                return false;
            }
        }
    }

    return true;
}

fn flashes_after_steps(grid: Grid, n: i64) -> i64 {
    let mut flashes = 0;
    for _ in 0..n {
        flashes += next_step(&grid);
    }
    flashes
}

fn flashes_simultanous_after(grid: Grid) -> i64 {
    let mut step = 0;
    loop {
        next_step(&grid);
        step += 1;

        if all_energy_equal_zero(&grid) {
            return step;
        }
    }
}
pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let grid = read_grid(BufReader::new(&file));
    let result = flashes_after_steps(grid, 100);
    result
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let grid = read_grid(BufReader::new(&file));
    let result = flashes_simultanous_after(grid);
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
    static TEST_DATA_FILEPATH: &str = "data/day11_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(1656, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(1571, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(195, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(387, task2_run(DATA_FILEPATH))
    }
}
