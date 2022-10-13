use std::{fmt, ops::Add};
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
    Other,
}

impl From<u32> for PacketType {
    fn from(v: u32) -> Self {
        match v {
            6 => PacketType::Literal,
            _ => PacketType::Other,
        }
    }
}

enum PacketValue {
    Literal(u32)
}

enum Operator {
    TypeZero,
    TypeOne,
}

fn value_literal_to_u32(value: String) -> u32 {
    let mut start = 0;
    let mut bin_val = String::new();
    loop {
        let chunk = &value[start..start+5];
        let chunk_bin_value = &chunk[1..];
        bin_val.push_str(chunk_bin_value.into());

        if chunk.starts_with("0") {
            break
        }

        start += 5;
    };
    bin_str_to_u32(bin_val)
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
        let packet_type = self.packet_type();
        assert_eq!(PacketType::Literal, packet_type);
        value_literal_to_u32(self.raw_content.to_string())
    }

    fn packet_type(&self) -> PacketType {
        PacketType::from(self.type_)
    }

    fn value(self) -> Result<u32, &'static str> {
        if self.packet_type() == PacketType::Literal {
            Ok(self.value_literal())
        } else {
            Err("Packet that is not literal cannot have value.")
        }
    }

    fn length_type_id(self) -> Result<String, &'static str> {
        if self.packet_type() == PacketType::Literal {
            Err("packet that is literal has no length type id")
        } else {
            Ok(self.raw_content[..0].into())
        }
    }

    fn packets(self) -> Result<Vec<Packet>, &'static str> {
        if self.packet_type() == PacketType::Literal {
            return Err("packet that is literal cannot have subpackets")
        };
        // Parse packets.
        Ok(Vec::new())
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


    #[test]
    fn test_value_literal_0() {
        let input = String::from("00000");
        let value = value_literal_to_u32(input);
        assert_eq!(0, value)
    }


    #[test]
    fn test_value_literal_2021() {
        let input = String::from("101111111000101000");
        let value = value_literal_to_u32(input);
        assert_eq!(2021, value)
    }

    #[test]
    fn test_length_type_id() {
        let raw_input = String::from("38006F45291200");
        let packet = Packet::from_string(raw_input);

        assert_eq!("0", packet.length_type_id().unwrap())
    }

}
