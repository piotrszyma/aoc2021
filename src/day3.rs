use core::ops::Range;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_bit_strings(reader: BufReader<&std::fs::File>) -> Vec<String> {
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn task1_run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let bit_strings = read_bit_strings(BufReader::new(&file));

    let no_bits = bit_strings[0].len();
    let mut zeros = vec![0; no_bits];
    let mut ones = vec![0; no_bits];

    for bit_string in bit_strings {
        for (index, bit) in bit_string.chars().enumerate() {
            if bit == '0' {
                zeros[index] += 1;
            } else {
                ones[index] += 1;
            }
        }
    }

    let bits_range = Range {
        start: 0,
        end: no_bits,
    };

    let mut gamma_bits: Vec<i32> = vec![0; no_bits];
    for id in bits_range {
        if zeros[id] > ones[id] {
            gamma_bits[id] = 0
        } else {
            gamma_bits[id] = 1
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (idx, value) in gamma_bits.iter().rev().enumerate() {
        if value == &1 {
            gamma_rate += 1 << idx;
        } else {
            epsilon_rate += 1 << idx;
        }
    };
    epsilon_rate * gamma_rate
}

type TakeOnesWhenFn = fn(a: i32, b: i32) -> bool;

fn find_at(nums: Vec<Vec<char>>, idx: usize, take_ones_when_fn: TakeOnesWhenFn) -> Vec<char> {
    let mut ones_at_idx = 0;
    let mut zeroes_at_idx = 0;

    for num in nums.iter() {
        if num[idx] == '0' {
            zeroes_at_idx += 1;
        } else {
            ones_at_idx += 1;
        }
    }

    let more_at_idx = if take_ones_when_fn(zeroes_at_idx, ones_at_idx) {
        '1'
    } else {
        '0'
    };

    let filtered_nums: Vec<Vec<char>> =
        nums.into_iter().filter(|n| n[idx] != more_at_idx).collect();

    if filtered_nums.len() == 1 {
        filtered_nums.first().unwrap().to_owned()
    } else {
        find_at(filtered_nums, idx + 1, take_ones_when_fn)
    }
}

fn find_less(nums: Vec<Vec<char>>) -> Vec<char> {
    if nums.len() == 1 {
        return nums.first().unwrap().to_owned();
    } else {
        return find_at(nums, 0, |zero_count_at_idx, one_count_at_idx| {
            zero_count_at_idx > one_count_at_idx
        });
    }
}

fn find_more(nums: Vec<Vec<char>>) -> Vec<char> {
    if nums.len() == 1 {
        return nums.first().unwrap().to_owned();
    } else {
        return find_at(nums, 0, |zero_count_at_idx, one_count_at_idx| {
            zero_count_at_idx <= one_count_at_idx
        });
    }
}

fn bitstring_to_i32(bit_string: Vec<char>) -> i32 {
    let mut result = 0;
    for (idx, value) in bit_string.iter().rev().enumerate() {
        if value == &'1' {
            result += 1 << idx;
        }
    }
    result
}

pub fn task2_run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let bit_strings = read_bit_strings(BufReader::new(&file));
    let nums: Vec<Vec<char>> = bit_strings
        .iter()
        .map(|bit_string| bit_string.chars().collect())
        .collect();

    let more_result = bitstring_to_i32(find_more(nums.to_owned())); //TODO: Can I just borrow read only?
    let less_result = bitstring_to_i32(find_less(nums));
    more_result * less_result
}

pub fn task1() -> i32 {
    task1_run("data/day3_task1.txt")
}


pub fn task2() -> i32 {
    task2_run("data/day3_task1.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_test_data() {
        assert_eq!(198, task1_run("data/day3_task1_test.txt"))
    }

    #[test]
    fn task1() {
        assert_eq!(741950, task1_run("data/day3_task1.txt"))
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(230, task2_run("data/day3_task1_test.txt"))
    }

    #[test]
    fn task2() {
        assert_eq!(903810, task2_run("data/day3_task1.txt"))
    }
}


