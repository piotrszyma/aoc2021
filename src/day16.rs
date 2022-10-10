use std::fmt;

// packet
// first three bits packet version <-- most sign bits first
// next 3 bits packet type ID  <-- most significant bits first
// i.e. version 100 -> 4
// packet with ID:
//   4 -> literal value
//
// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC

#[derive(Debug)]
struct Packet{
    raw_version: String,
    raw_type: String,
    raw_content: String,
}

fn hex_str_to_bin(hex: String) -> String {
    let n: u32 = u32::from_str_radix(&hex, 16).unwrap();
    format!("{n:b}")
}

impl Packet {
    fn from_string(hex_input: String) -> Self {
        let raw_version = &hex_input[..3];
        let raw_type = &hex_input[3..6];
        let raw_content = &hex_input[6..];
        Self {
            raw_version: raw_version.to_string(),
            raw_type: raw_type.to_string(),
            raw_content: raw_content.to_string(),
        }
    }
}

impl<'a> fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.raw_version, self.raw_type, self.raw_content)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // static TEST_DATA_FILEPATH: &str = "data/day16_test.txt";

    #[test]
    fn task1_test1() {
        let input = String::from("D2FE28");
        let bin_input = hex_str_to_bin(input);
        let packet = Packet::from_string(input);

        println!("{}", packet)
    }

}
