use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(point_str: &str) -> Self {
        let mut coords = point_str.split(',');
        let x = coords.next().expect("point_str should be 'x,y'");
        let y = coords.next().expect("point_str should be 'x,y'");
        let x: i32 = x.parse().expect("point_str x should be int");
        let y: i32 = y.parse().expect("point_str y should be int");
        return Point { x, y };
    }
}

#[derive(Debug)]
struct LinesRange {
    start: Point,
    end: Point,
}

fn read_lines_ranges(reader: BufReader<&std::fs::File>) -> Vec<LinesRange> {
    let mut ranges: Vec<LinesRange> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        // TODO: Should points be "mut"?
        let mut points = line.split(" -> ");
        let start_point = points.next().unwrap();
        let end_point = points.next().unwrap();
        ranges.push(LinesRange {
            start: Point::new(start_point),
            end: Point::new(end_point),
        })
    }
    ranges
}

fn calculate_overlaps(ranges: Vec<LinesRange>) -> i32 {
    let mut counts = HashMap::<(i32, i32), i32>::new();
    for range in &ranges {
        if range.start.x == range.end.x {
            let x = range.start.x;
            let y_start = cmp::min(range.start.y, range.end.y);
            let y_end = cmp::max(range.start.y, range.end.y);
            let range = Range {
                start: y_start,
                end: y_end + 1,
            };

            for y in range {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        } else if range.start.y == range.end.y {
            let y = range.start.y;
            let x_start = cmp::min(range.start.x, range.end.x);
            let x_end = cmp::max(range.start.x, range.end.x);

            let range = Range {
                start: x_start,
                end: x_end + 1,
            };

            for x in range {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        } else {
            let x_diff = if range.start.x > range.end.x { -1 } else { 1 };
            let y_diff = if range.start.y > range.end.y { -1 } else { 1 };

            let mut x = range.start.x;
            let mut y = range.start.y;

            loop {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;

                if x == range.end.x && y == range.end.y {
                    break;
                }

                x += x_diff;
                y += y_diff;
            }
        }
    }
    let mut overlaps = 0;
    for (_, count) in &counts {
        if count > &1 {
            overlaps += 1
        }
    }
    overlaps
}

pub fn run() {
    let file = File::open("data/day5_task1.txt").unwrap();
    let lines_ranges = read_lines_ranges(BufReader::new(&file));

    let result = calculate_overlaps(lines_ranges);

    println!("result={:?}", result)
}
