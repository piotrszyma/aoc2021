fn hex_str_to_u32(hex_input: String) -> u32 {
    u32::from_str_radix(&hex_input, 16).unwrap()
}

pub fn bin_str_to_u32(bin_input: String) -> u32 {
    u32::from_str_radix(&bin_input, 2).unwrap()
}


pub fn hex_str_to_bin(hex_input: String) -> String {
    let n = hex_str_to_u32(hex_input);
    format!("{n:b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_str_to_u32() {
        let result = hex_str_to_u32(String::from("A"));
        assert_eq!(10, result)
    }

    #[test]
    fn test_bin_str_to_u32() {
        let result = bin_str_to_u32(String::from("001101"));
        assert_eq!(1 + 4 + 8, result)
    }

    #[test]
    fn test_hex_to_bin() {
        let hex = String::from("A");
        let bin = hex_str_to_bin(hex);
        assert_eq!(String::from("1010"), bin)
    }

}
