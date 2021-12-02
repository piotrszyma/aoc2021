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
    distance: i32,
}

#[derive(Debug)]
struct BoatPosition {
    horizontal_position: i32,
    depth: i32,
}

fn read_commands(reader: BufReader<&std::fs::File>) -> Vec<BoatCommand> {
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let split: Vec<&str> = line.split(" ").collect();
            let direction = split.get(0).unwrap();
            let raw_distance: String = split.get(1).unwrap().to_string();
            let distance: i32 = raw_distance
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
        horizontal_position: horizontal_position,
        depth: depth,
    }
}

pub fn run() {
    let file = File::open("data/day2_task1.txt").unwrap();
    let commands = read_commands(BufReader::new(&file));
    let position = final_position(commands);
    let result = position.depth * position.horizontal_position;
    println!("result: {:?}", result);
}
