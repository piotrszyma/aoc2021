use core::ops::Range;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_bit_strings(reader: BufReader<&std::fs::File>) -> Vec<String> {
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn run() {
    let file = File::open("data/day3_task1.txt").unwrap();
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
    }

    println!("gamma_rate={:?}", gamma_rate);
    println!("epsilon_rate={:?}", epsilon_rate);
    let result = epsilon_rate * gamma_rate;
    println!("result={:?}", result)
}
