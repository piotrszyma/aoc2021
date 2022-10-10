use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum BoatDirection {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct BoatCommand {
    direction: BoatDirection,
    distance: i64,
}

#[derive(Debug)]
struct BoatPosition {
    horizontal_position: i64,
    depth: i64,
}

fn read_commands(reader: BufReader<&std::fs::File>) -> Vec<BoatCommand> {
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let split: Vec<&str> = line.split(" ").collect();
            let direction = split.get(0).unwrap();
            let raw_distance: String = split.get(1).unwrap().to_string();
            let distance: i64 = raw_distance
                .parse()
                .expect(&format!("Expected {} to be a number.", &raw_distance));
            BoatCommand {
                direction: {
                    match direction {
                        &"up" => BoatDirection::Up,
                        &"down" => BoatDirection::Down,
                        &"forward" => BoatDirection::Forward,
                        _ => panic!("Unsupported boat direction={}", direction),
                    }
                },
                distance: distance,
            }
        })
        .collect()
}

fn final_position(commands: Vec<BoatCommand>) -> BoatPosition {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for command in commands {
        match command.direction {
            BoatDirection::Up => depth -= command.distance,
            BoatDirection::Down => depth += command.distance,
            BoatDirection::Forward => horizontal_position += command.distance,
        }
    }

    BoatPosition {
        horizontal_position,
        depth,
    }
}

fn final_position_with_aim(commands: Vec<BoatCommand>) -> BoatPosition {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command.direction {
            BoatDirection::Up => aim -= command.distance,
            BoatDirection::Down => aim += command.distance,
            BoatDirection::Forward => {
                horizontal_position += command.distance;
                depth += aim * command.distance;
            }
        }
    }

    BoatPosition {
        horizontal_position,
        depth,
    }
}

fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let commands = read_commands(BufReader::new(&file));
    let position = final_position(commands);
    position.depth * position.horizontal_position
}

pub fn task1() -> i64 {
    task1_run("data/day2.txt")
}

fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let commands = read_commands(BufReader::new(&file));
    let position = final_position_with_aim(commands);
    position.depth * position.horizontal_position
}

pub fn task2() -> i64 {
    task2_run("data/day2.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_test_data() {
        assert_eq!(150, task1_run("data/day2_test.txt"))
    }

    #[test]
    fn task1() {
        assert_eq!(2322630, task1_run("data/day2.txt"))
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(900, task2_run("data/day2_test.txt"))
    }

    #[test]
    fn task2() {
        assert_eq!(2105273490, task2_run("data/day2.txt"))
    }
}

