fn hex_str_to_u32(hex_input: String) -> u32 {
    u32::from_str_radix(&hex_input, 16).unwrap()
}

pub fn bin_str_to_u32(bin_input: String) -> u32 {
    u32::from_str_radix(&bin_input, 2).unwrap()
}


pub fn hex_str_to_bin(hex_input: String) -> String {
    let mut output = String::new();
    for c in hex_input.chars() {
        let n = hex_str_to_u32(c.to_string());
        output.push_str(&format!("{n:0>4b}"))
    };
    output
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

    #[test]
    fn text_hex_to_bin_0() {
        let hex = "0000";
        let bin = hex_str_to_bin(hex.to_string());
        assert_eq!(String::from("0000000000000000"), bin)
    }



    #[test]
    fn text_hex_to_bin_38006f45291200() {
        let hex = "38006F45291200";
        let bin = hex_str_to_bin(hex.to_string());
        assert_eq!(String::from("00111000000000000110111101000101001010010001001000000000"), bin)
    }

}
