use crate::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day8.txt";

#[derive(Debug)]
struct NotesEntry {
    segments: Vec<String>,
    output: Vec<String>,
}

fn read_notes_entries(reader: BufReader<&std::fs::File>) -> Vec<NotesEntry> {
    let mut entries = Vec::<NotesEntry>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut segment_and_output = line.split(" | ");
        let raw_segments = segment_and_output.next().unwrap();
        let raw_output = segment_and_output.next().unwrap();

        let segments: Vec<_> = raw_segments.split(' ').map(|s| s.to_string()).collect();
        let output: Vec<_> = raw_output.split(' ').map(|s| s.to_string()).collect();
        entries.push(NotesEntry { segments, output })
    }
    entries
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let entries = read_notes_entries(BufReader::new(&file));
    let mut counts = HashMap::<i8, i64>::new();
    for entry in entries {
        for element in &entry.output {
            match element.len() {
                2 => *counts.entry(1).or_insert(0) += 1,
                4 => *counts.entry(4).or_insert(0) += 1,
                3 => *counts.entry(7).or_insert(0) += 1,
                7 => *counts.entry(8).or_insert(0) += 1,
                _ => (),
            }
        }
    }
    let result = counts.values().sum();
    result
}

type NumsToOptions = HashMap<String, Vec<String>>;
type CharToResolvedChar = HashMap<String, String>;

fn substract(a: &String, b: &String) -> String {
    let mut result = String::new();
    for c in a.chars() {
        if !b.contains(c) {
            result.push(c);
        }
    }
    result
}

fn filter_out_char(nums: NumsToOptions, char_to_filter: String) -> NumsToOptions {
let mut nums_less_char = NumsToOptions::new();
    for (k, v) in nums {
        let v_less_a: Vec<String> = v
            .iter()
            .map(|w| {
                let chars_less_a = w.chars().filter(|c| c.to_string() != char_to_filter).collect();
                chars_less_a
            })
            .collect();
        nums_less_char.insert(k, v_less_a);
    };
    nums_less_char
}

fn find_char_mapped_to_a(nums: NumsToOptions) -> (NumsToOptions, CharToResolvedChar) {
    let one = nums.get("1").unwrap().first().unwrap();
    let seven = nums.get("7").unwrap().first().unwrap();
    let char_mapped_to_a = substract(seven, one);
    let mut char_to_resolved = CharToResolvedChar::new();
    char_to_resolved.insert(char_mapped_to_a.to_string(), "a".to_string());
    let nums_less_char_mapped_to_a = filter_out_char(nums, char_mapped_to_a);
    (nums_less_char_mapped_to_a, char_to_resolved)
}

fn find_char_mapped_to_g_and_nine(
    mut nums: NumsToOptions,
    mut resolved: CharToResolvedChar,
) -> (NumsToOptions, CharToResolvedChar) {
    let four = nums.get("4").unwrap().first().unwrap();
    let seven = nums.get("7").unwrap().first().unwrap();
    let possible_nines = nums.get("069").expect("Expect 069 to be in nums.");

    let mut nine = Vec::<String>::new();
    let mut others = Vec::<String>::new();

    let mut maybe_char_mapped_to_g: Option<String> = None;

    for possible in possible_nines {
        let result = substract(&substract(possible, four), seven);
        if result.len() == 1 {
            nine.push(possible.to_string());
            maybe_char_mapped_to_g = Some(result);
        } else {
            others.push(possible.to_string());
        }
    }
    let char_mapped_to_g = maybe_char_mapped_to_g.expect("Failed to find char mapped to g");
    resolved.insert(char_mapped_to_g.to_string(), "g".to_string());
    nums.insert("06".to_string(), others);
    nums.insert("9".to_string(), nine);
    nums.remove("069");

    let nums = filter_out_char(nums, char_mapped_to_g);
    (nums, resolved)
}

fn find_char_mapped_to_by_substraction(
    nums: NumsToOptions,
    mut resolved: CharToResolvedChar,
    searched_char: &str,
    minuend: &str,
    subtrahend: &str,
) -> (NumsToOptions, CharToResolvedChar) {
    let minued = nums.get(minuend).unwrap().first().unwrap();
    let subtrahend = nums.get(subtrahend).unwrap().first().unwrap();

    let result = substract(minued, subtrahend);
    if result.len() != 1 {
        panic!("Failed to find {}", searched_char)
    };

    let char_mapped_to = result;

    resolved.insert(char_mapped_to.to_string(), searched_char.to_string());
    let nums = filter_out_char(nums, char_mapped_to);
    (nums, resolved)

}

fn find_two(
    mut nums: NumsToOptions,
    resolved: CharToResolvedChar,
) -> (NumsToOptions, CharToResolvedChar) {
    let possible_two = nums.get("235").unwrap();

    let mut two = Vec::<String>::new();
    let mut others = Vec::<String>::new();

    for possible in possible_two {
        if possible.len() == 2 {
            two.push(possible.to_string());
        } else {
            others.push(possible.to_string())
        }
    }

    if two.len() != 1{
        panic!("Failed to exctract 2 from 235")
    }


    nums.insert("2".to_string(), two);
    nums.insert("35".to_string(), others);
    nums.remove("235");

    (nums, resolved)
}

fn find_char_mapped_to_c(
    nums: NumsToOptions,
    mut resolved: CharToResolvedChar,
)-> (NumsToOptions, CharToResolvedChar) {
    let two = nums.get("2").unwrap().first().unwrap();
    let char_mapped_to_c = two.to_string();

    resolved.insert(char_mapped_to_c.to_string(), "c".to_string());
    let nums = filter_out_char(nums, char_mapped_to_c.to_string());

    (nums, resolved)
}


fn find_char_mapped_to_f(
    nums: NumsToOptions,
    mut resolved: CharToResolvedChar,
)-> (NumsToOptions, CharToResolvedChar) {
    let one = nums.get("1").unwrap().first().unwrap();
    let char_mapped_to_f = one.to_string();

    resolved.insert(char_mapped_to_f.to_string(), "f".to_string());
    let nums = filter_out_char(nums, char_mapped_to_f.to_string());

    (nums, resolved)
}


fn decode_and_figure_value(entry: NotesEntry) -> i64 {
    let mut nums = HashMap::<String, Vec<String>>::new();
    for element in entry.segments {
        match element.len() {
            2 => {
                nums.insert("1".to_string(), Vec::from([element]));
            }
            4 => {
                nums.insert("4".to_string(), Vec::from([element]));
            }
            3 => {
                nums.insert("7".to_string(), Vec::from([element]));
            }
            7 => {
                nums.insert("8".to_string(), Vec::from([element]));
            }
            6 => {
                if !nums.contains_key("069") {
                    nums.insert("069".to_string(), Vec::new());
                }
                nums.get_mut("069").unwrap().push(element)
            }
            5 => {
                if !nums.contains_key("235") {
                    nums.insert("235".to_string(), Vec::new());
                }
                nums.get_mut("235").unwrap().push(element)
            }
            len => panic!("Unexpected element.len() = {}", len),
        }
    }
    let (nums, chars_to_resolved_chars) = find_char_mapped_to_a(nums);
    let (nums, chars_to_resolved_chars) = find_char_mapped_to_g_and_nine(nums, chars_to_resolved_chars);
    let (nums, chars_to_resolved_chars) = find_char_mapped_to_by_substraction(nums, chars_to_resolved_chars, "e", "8", "4");
    let (nums, chars_to_resolved_chars) = find_two(nums, chars_to_resolved_chars);
    let (nums, chars_to_resolved_chars) = find_char_mapped_to_by_substraction(nums, chars_to_resolved_chars, "d", "2", "1");
    let (nums, chars_to_resolved_chars) = find_char_mapped_to_by_substraction(nums, chars_to_resolved_chars, "b", "4", "1");
    let (nums, chars_to_resolved_chars) = find_char_mapped_to_c(nums, chars_to_resolved_chars);
    let (_, chars_to_resolved_chars) = find_char_mapped_to_f(nums, chars_to_resolved_chars);

    let mut digits = Vec::<&str>::new();

    for digit in entry.output {
        let mut digit: Vec<String> = digit.chars().map(|c| chars_to_resolved_chars.get(&c.to_string()).unwrap().to_string() ).collect();
        digit.sort();
        let digit = digit.join("");
        digits.push(match digit.as_ref() {
           "abcefg" => "0",
           "cf" => "1",
           "acdeg" => "2",
           "acdfg" => "3",
           "bcdf" => "4",
           "abdfg" => "5",
           "abdefg" => "6",
           "acf" => "7",
           "abcdefg" => "8",
           "abcdfg" => "9",
            _ => panic!("Unexpected number encoding"),
        })
    }

    let parsed_digits: i64 = digits.join("").parse().unwrap();

    parsed_digits
}

fn decode_and_sum_all(entries: Vec<NotesEntry>) -> i64 {
    entries
        .into_iter()
        .map(|e| decode_and_figure_value(e))
        .sum()
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let entries = read_notes_entries(BufReader::new(&file));

    let result = decode_and_sum_all(entries);
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
    static TEST_DATA_FILEPATH: &str = "data/day8_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(26, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(495, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(61229, task2_run(TEST_DATA_FILEPATH))
    }

    #[test] 
    fn task2() {
        assert_eq!(1055164, task2_run(DATA_FILEPATH))
    }

    // #[test]
    // fn calc_fuel_more_expensive() {
    //     assert_eq!(calc_fuel_needed_more_expensive(&Vec::from([1]), 5), 10)
    // }
}
