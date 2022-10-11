use std::fmt;
use std::convert::TryFrom;

use crate::hex::{hex_str_to_bin, bin_str_to_u32};

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

    version: u32,
    type_: u32,
}

#[derive(PartialEq, Debug)]
enum PacketType {
    Literal = 6,
}

impl TryFrom<u32> for PacketType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            6 => Ok(PacketType::Literal),
            _ => Err(()),
        }
    }
}

enum PacketValue {
    Literal(u32)
}

fn value_literal_to_u32(value: String) -> u32 {
    let chars: Vec<_> = value.chars().collect();
    chars.windows(4).map(|w| )
    1
}

impl Packet {
    fn from_string(hex_input: String) -> Self {
        let bin_input = hex_str_to_bin(hex_input);

        let raw_version = &bin_input[..3];
        let raw_type = &bin_input[3..6];
        let raw_content = &bin_input[6..];

        let type_ = bin_str_to_u32(raw_type.to_string());
        let version = bin_str_to_u32(raw_version.to_string());

        Self {
            raw_version: raw_version.to_string(),
            raw_type: raw_type.to_string(),
            raw_content: raw_content.to_string(),
            version,
            type_,
        }
    }

    fn value_literal(&self) -> u32 {
        let packet_type = PacketType::try_from(self.type_).unwrap();
        assert_eq!(PacketType::Literal, packet_type);
        value_literal_to_u32(self.raw_content.to_string())
    }

    fn value(self) -> u32 {
        self.value_literal()
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

    #[test]
    fn test_packet_from_string() {
        let raw_input = String::from("D2FE28");
        let packet = Packet::from_string(raw_input);

        assert_eq!("110", packet.raw_version);
        assert_eq!("100", packet.raw_type);
        assert_eq!("101111111000101000", packet.raw_content);

        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_);
    }

}
