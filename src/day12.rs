use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day12.txt";

#[derive(PartialEq, Eq, Hash)]
enum CaveKind {
    Big,
    Small,
}

type CaveName = String;

type CaveConnections = HashMap<CaveName, Vec<CaveName>>;

static START_CAVE: &str = "start";
static END_CAVE: &str = "end";

fn cave_kind(cave_name: &str) -> CaveKind {
    if cave_name == cave_name.to_uppercase() {
        CaveKind::Big
    } else {
        CaveKind::Small
    }
}

fn read_data(reader: BufReader<&File>) -> CaveConnections {
    let mut connections = CaveConnections::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let caves: Vec<&str> = line.split("-").collect();
        let first_cave_name = caves.first().unwrap().to_string();
        let second_cave_name = caves.last().unwrap().to_string();

        // You can move from first cave to second cave.
        let entry = connections
            .entry(first_cave_name.to_string())
            .or_insert(Vec::new());
        entry.push(second_cave_name.to_string());

        // You can move from second cave to first cave.
        let entry = connections.entry(second_cave_name).or_insert(Vec::new());
        entry.push(first_cave_name);
    }

    connections
}

fn find_paths(
    start_cave: &str,
    path: Vec<String>,
    connections: &CaveConnections,
    can_visit_small_twice: bool,
) -> Vec<Vec<String>> {
    if start_cave == END_CAVE {
        let mut new_path = Vec::from(path);
        new_path.push(END_CAVE.to_string());
        return vec![new_path];
    } else {
        let adjacent_caves = connections.get(start_cave).unwrap();
        let mut paths: Vec<Vec<String>> = Vec::new();
        for adjacent_cave in adjacent_caves {
            let mut can_visit_small_twice = can_visit_small_twice;

            if cave_kind(adjacent_cave) == CaveKind::Small && path.contains(adjacent_cave) {
                if adjacent_cave == END_CAVE || adjacent_cave == START_CAVE || !can_visit_small_twice {
                    continue;
                } else {
                    can_visit_small_twice = false;
                }
            } 

            let mut new_path = path.clone();
            new_path.push(start_cave.to_string()); // Add cave we entered on this find_paths call.

            let found_paths = find_paths(adjacent_cave, new_path, connections, can_visit_small_twice);
            paths.extend(found_paths);

        };

        return paths;
    };
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let data: CaveConnections = read_data(BufReader::new(&file));
    let paths = find_paths(START_CAVE, vec![], &data, false);
    paths.len() as i64
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let data: CaveConnections = read_data(BufReader::new(&file));
    let paths = find_paths(START_CAVE, vec![], &data, true);
    paths.len() as i64
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
    static TEST_DATA_FILEPATH: &str = "data/day12_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(10, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(5228, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(36, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(131228, task2_run(DATA_FILEPATH))
    }
}
