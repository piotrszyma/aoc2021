use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day14.txt";

type Polymer = HashMap<String, i64>;

type Insertions = HashMap<String, Vec<String>>;

#[derive(Debug)]
struct Input {
    template: String,
    insertions: Insertions,
}

fn read_data(reader: BufReader<&File>) -> Input {
    let mut lines = reader.lines();
    let template: String = lines
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect();
    lines.next(); // skip empty line
    let mut insertions: Insertions = HashMap::new();
    for line in lines {
        let template = line.unwrap();
        let mut split = template.split(" -> ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        let mut split = from.split("");
        split.next();
        let first_char = split.next().unwrap();
        let second_char = split.next().unwrap();
        insertions.insert(
            from.to_string(),
            vec![first_char.to_string() + to, to.to_string() + second_char],
        );
    }

    Input {
        template,
        insertions,
    }
}

fn group_in_pairs(value: &String) -> Vec<String> {
    let mut idx = 0;
    let max_idx = value.len() - 2;

    let mut result = Vec::new();

    while idx <= max_idx {
        result.push((&value[idx..=idx + 1]).to_string());
        idx += 1;
    }

    return result;
}

fn run_step(polymer: Polymer, insertions: &Insertions) -> Polymer {
    let mut new_polymer = Polymer::new();

    for (pair, count) in polymer.iter() {
        match insertions.get(pair) {
            Some(new_pairs) => {
                for new_pair in new_pairs {
                    let entry = new_polymer.entry(new_pair.to_string()).or_insert(0);
                    *entry += count
                }
            }
            _ => {
                let entry = new_polymer.entry(pair.to_string()).or_insert(0);
                *entry += count
            }
        }
    }

    new_polymer
}

fn polymer_from_template(template: &String) -> Polymer {
    let mut polymer = Polymer::new();
    for e in group_in_pairs(&template) {
        let entry = polymer.entry(e).or_insert(0);
        *entry += 1;
    }
    polymer
}

fn run_steps(path: &str, steps: i64) -> i64 {
    let file = File::open(path).unwrap();
    let data = read_data(BufReader::new(&file));

    let mut polymer = polymer_from_template(&data.template);

    for _ in 1..=steps {
        polymer = run_step(polymer, &data.insertions);
    }

    let mut counts = HashMap::<String, i64>::new();
    for (pair, count) in polymer.iter() {
        for c in pair.chars() {
            let entry = counts.entry(c.to_string()).or_insert(0);
            *entry += count;
        }
    }

    let mut sorted_counts: Vec<i64>  = counts.into_iter().map(|(_, count)| {
        (count + 1) / 2
    }).collect();
    sorted_counts.sort();

    let smallest_count = sorted_counts.first().unwrap();
    let biggest_count = sorted_counts.last().unwrap();

    biggest_count - smallest_count
}

pub fn task1_run(path: &str) -> i64 {
    run_steps(path, 10)
}

pub fn task2_run(path: &str) -> i64 {
    run_steps(path, 40)
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
    static TEST_DATA_FILEPATH: &str = "data/day14_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(1588, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(2587, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(2188189693529, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(3318837563123, task2_run(DATA_FILEPATH))
    }

    #[test]
    fn test_run_step() {
        assert_eq!(1/2, 0);
        let insertions = Insertions::from([
            (
                String::from("AB"),
                vec![String::from("AC"), String::from("CB")],
            ),
            (
                String::from("DE"),
                vec![String::from("DC"), String::from("CE")],
            ),
        ]);
        let polymer = Polymer::from([(String::from("AB"), 2), (String::from("AC"), 2)]);
        let expected = Polymer::from([(String::from("AC"), 4), (String::from("CB"), 2)]);

        assert_eq!(expected, run_step(polymer, &insertions));
    }
}
