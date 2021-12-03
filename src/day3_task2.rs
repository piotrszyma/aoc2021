use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_bit_strings(reader: BufReader<&std::fs::File>) -> Vec<String> {
    reader.lines().map(|l| l.unwrap()).collect()
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

pub fn run() {
    let file = File::open("data/day3_task1.txt").unwrap();
    let bit_strings = read_bit_strings(BufReader::new(&file));
    let nums: Vec<Vec<char>> = bit_strings
        .iter()
        .map(|bit_string| bit_string.chars().collect())
        .collect();

    let more_result = bitstring_to_i32(find_more(nums.to_owned())); //TODO: Can I just borrow read only? 
    let less_result = bitstring_to_i32(find_less(nums));

    println!("more_result={:?}", more_result);
    println!("less_result={:?}", less_result);
    let result = more_result * less_result;

    println!("result={:?}", result);
}
