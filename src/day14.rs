use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day14.txt";

type Insertions = HashMap<String, String>;

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
    let mut insertions = HashMap::new();
    for line in lines {
        let template = line.unwrap();
        let mut split = template.split(" -> ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        // let mut split = from.split("");
        // split.next();
        // let first_char = split.next().unwrap();
        // let second_char = split.next().unwrap();
        insertions.insert(from.to_string(), to.to_string());
    }

    Input {
        template,
        insertions,
    }
}

#[derive(Debug)]
struct Count {
    symbol: String,
    value: u64,
}

#[derive(Debug)]
struct TopCounts {
    most_common: Count,
    least_common: Count,
}

fn most_least_common(template: String) -> TopCounts {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut most_common = template.chars().nth(0).unwrap().to_string();
    let mut least_common = template.chars().nth(0).unwrap().to_string();

    for c in template.chars() {
        let entry = counts.get(&c.to_string()).unwrap_or(&0);
        let entry = entry + 1;

        let most_common_count = counts.get(&most_common);
        if most_common_count.is_none() || most_common_count.unwrap() < &entry {
            most_common = c.to_string();
        }

        let least_common_count = counts.get(&least_common);
        if least_common_count.is_none() || least_common_count.unwrap() > &entry {
            least_common = c.to_string();
        }

        counts.insert(c.to_string(), entry);
    }

    let most_common_count = counts.get(&most_common).unwrap();
    let least_common_count = counts.get(&least_common).unwrap();

    TopCounts {
        most_common: Count {
            symbol: most_common.to_string(),
            value: *most_common_count,
        },
        least_common: Count {
            symbol: least_common.to_string(),
            value: *least_common_count,
        },
    }
}

// fn perform_step(template: Vec<String>, insertions: &Insertions) -> Vec<String> {
//     let mut new_template: Vec<String> = Vec::new();

//     let first_char = template.first().unwrap().to_string();

//     new_template.push(first_char);

//     for window in template.windows(2) {
//         let prev = &window[0];
//         let next = &window[1];
//         let key = (prev.to_string(), next.to_string());
//         if insertions.contains_key(&key) {
//             let value = insertions.get(&key).unwrap();
//             new_template.push(value.to_string());
//         }
//         new_template.push(next.to_string());
//     }

//     new_template
// }

fn insert_between(insert_into: &String, value: &String) -> String {
    if insert_into.len() != 2 {
        panic!("Can insert only into string of len 2")
    }

    let mut with_inserted = String::new();
    let mut insert_into = insert_into.chars();
    let first_char = insert_into.next().unwrap();
    let second_char = insert_into.next().unwrap();
    let new_char = value.chars().next().unwrap();

    with_inserted.push(first_char);
    with_inserted.push(new_char);
    with_inserted.push(second_char);

    return with_inserted;
}

fn drop_last_char(value: String) -> String {
    let len = value.len();
    (&value[0..len - 1]).to_string()
}

fn get_last_char(value: String) -> String {
    let len = value.len();
    (&value[len - 1..len]).to_string()
}

fn find_after(
    template: String,
    after_steps: i64,
    insertions: &HashMap<String, String>,
    cache: &mut HashMap<(String, i64), String>,
) -> String {
    if after_steps == 0 {
        return template;
    }

    match cache.get(&(template.to_string(), after_steps)) {
        Some(value) => return value.to_string(),
        _ => (),
    }

    if template.len() == 2 {
        let result = match insertions.get(&template) {
            Some(new_key) => {
                let new_template = insert_between(&template, new_key);
                find_after(new_template, after_steps - 1, insertions, cache)
                // let after_step_less_one = after_steps - 1;
                // // TODO: after_steps / 2
                // if after_step_less_one % 2 == 0 {
                //     let intermediate_template =
                //         find_after(new_template, after_step_less_one / 2, insertions, cache);
                //     find_after(intermediate_template, after_step_less_one / 2, insertions, cache)
                // } else {
                //     find_after(new_template, after_step_less_one - 1, insertions, cache)
                // }
            }
            _ => template.to_string(),
        };
        // cache.insert((template.to_string(), after_steps), result);
        cache.insert((template.to_string(), after_steps), result.to_string());
        return result;
    } else if template.len() > 2 {
        let mut new_template = String::new();
        for chunk in split_into_doubles(&template) {
            let result = find_after(chunk, after_steps, insertions, cache);
            new_template += &drop_last_char(result);
        }
        new_template += &get_last_char(template);
        new_template
    } else {
        panic!("Called with template len < 2.")
    }
}

fn split_into_doubles(template: &String) -> Vec<String> {
    let mut idx = 0;
    let max_idx = template.len() - 2; // element last - 1

    let mut template_windows = Vec::new();

    while idx <= max_idx {
        template_windows.push((&template[idx..=idx + 1]).to_string());

        idx += 1;
    }

    return template_windows;
}

fn run_steps(path: &str, steps: i64) -> i64 {
    let file = File::open(path).unwrap();
    let data = read_data(BufReader::new(&file));

    let final_template = find_after(data.template, steps, &data.insertions, &mut HashMap::new());

    println!("final_template={:?}", final_template);
    let top_counts = most_least_common(final_template);
    println!("top_counts={:?}", top_counts);
    let result = top_counts.most_common.value - top_counts.least_common.value;
    result as i64
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

    // #[test]
    // fn task2_test_data() {
    //     assert_eq!(0, task2_run(TEST_DATA_FILEPATH))
    // }

    // #[test]
    // fn task2() {
    //     assert_eq!(0, task2_run(DATA_FILEPATH))
    // }

    // #[test]
    // fn test_split_into_doubles() {
    //     assert_eq!(vec!["ab".to_string(), "bc".to_string(), "cd".to_string()], split_into_doubles("abcd".to_string()))
    // }
}
