use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day10.txt";

fn read_lines(reader: BufReader<&File>) -> Vec<String> {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    lines
}

#[derive(Debug)]
enum LineStatus {
    Incomplete,
    IllegalChar(String),
    Valid,
}

type LineUnclosedParens = Vec<String>;

fn get_line_status(
    line: &String,
    paren_open_to_close: &HashMap<String, String>,
) -> (LineStatus, LineUnclosedParens) {
    let mut unclosed_parens = LineUnclosedParens::new();
    let open_parens: HashSet<String> = paren_open_to_close.keys().map(|k| k.to_string()).collect();

    for (_, paren) in line.chars().enumerate() {
        if open_parens.contains(&paren.to_string()) {
            unclosed_parens.push(paren.to_string())
        } else {
            // close_paren
            let last_seen = unclosed_parens.last();

            if last_seen.is_none() {
                return (LineStatus::IllegalChar(paren.to_string()), unclosed_parens)
            }

            let last_seen = last_seen.unwrap();

            let expected_close = paren_open_to_close.get(last_seen);
            match expected_close {
                Some(expected_close) => {
                    if paren.to_string() != *expected_close {
                        return (LineStatus::IllegalChar(paren.to_string()), unclosed_parens)
                    } else {
                        unclosed_parens.pop();
                        continue;
                    }
                }
                _ => {
                    return (LineStatus::IllegalChar(paren.to_string()), unclosed_parens)
                }
            };
        }
    }

    if unclosed_parens.len() > 0 {
        (LineStatus::Incomplete, unclosed_parens)
    } else {
        (LineStatus::Valid, unclosed_parens)
    }
}


pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let lines = read_lines(BufReader::new(&file));
    println!("lines={:?}", lines);
    let paren_close_to_open: HashMap<String, String> = HashMap::from([
        ("{".to_string(), "}".to_string()),
        ("[".to_string(), "]".to_string()),
        ("<".to_string(), ">".to_string()),
        ("(".to_string(), ")".to_string()),
    ]);
    let invalid_scores: HashMap<String, i64> = HashMap::from([
        (")".to_string(), 3),
        ("]".to_string(), 57),
        ("}".to_string(), 1197),
        (">".to_string(), 25137),
    ]);

    lines
        .into_iter()
        .map(|l| {
            let (status, _) = get_line_status(&l, &paren_close_to_open);
            status
        })
        .map(|line_result| match line_result {
            LineStatus::Valid => 0,
            LineStatus::Incomplete => 0,
            LineStatus::IllegalChar(c) => {
                *(invalid_scores.get(&c).expect("Unexpected invalid char"))
            }
        })
        .sum()
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let lines = read_lines(BufReader::new(&file));
    println!("lines={:?}", lines);
    let paren_close_to_open: HashMap<String, String> = HashMap::from([
        ("{".to_string(), "}".to_string()),
        ("[".to_string(), "]".to_string()),
        ("<".to_string(), ">".to_string()),
        ("(".to_string(), ")".to_string()),
    ]);
    let autocomplete_scores: HashMap<String, i64> = HashMap::from([
        ("(".to_string(), 1),
        ("[".to_string(), 2),
        ("{".to_string(), 3),
        ("<".to_string(), 4),
    ]);

    let unclosed_chars: Vec<Vec<String>> = lines
        .into_iter()
        .map(|l| {
            let (result, unclosed) = get_line_status(&l, &paren_close_to_open);
            (result, unclosed)
        })
        .filter(|(result, _)| match result {
            LineStatus::Incomplete => true,
            _ => false,
        })
        .map(|(_, unclosed)| unclosed)
        .collect();
    
    let mut results: Vec<i64> = Vec::new();
    for unclosed in unclosed_chars {
        let mut total = 0;
        let mut unclosed = unclosed;
        unclosed.reverse();
        for char_score in unclosed.iter().map(|c| autocomplete_scores.get(c).unwrap()) {
            total = total * 5 + char_score;
        }
        results.push(
            total
        );
        
    };
    results.sort();
    let middle_result = results[results.len() / 2];  // Results are expected to always be of odd length.
    middle_result

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
    static TEST_DATA_FILEPATH: &str = "data/day10_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(26397, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(423, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(288957, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        assert_eq!(3646451424, task2_run(DATA_FILEPATH))
    }
}
