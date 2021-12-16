use std::error::Error;
use std::u8;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_16.in", Box::new(Day16))
}

struct Day16;

type Input = Packet;
type Output = i32;

#[derive(PartialEq, Debug)]
enum PacketBody {
    Literal(u16),
    Operation(Vec<Packet>)
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u8,
    packet_type: u8,
    body: PacketBody
}

impl Packet {
    fn literal(header: u8, literal: u16) -> Self {
        Packet { version: header >> 3, packet_type: header & 0b111, body: PacketBody::Literal(literal) }
    }

    fn operation(header: u8, packets: Vec<Packet>) -> Self {
        Packet { version: header >> 3, packet_type: header & 0b111, body: PacketBody::Operation(packets) }
    }

    fn from(version: u8, packet_type: u8, body: PacketBody) -> Self {
        Packet { version, packet_type, body }
    }
}

fn convert_hex_to_bits(str: &str) -> Result<String, Box<dyn Error>> {
    let mut results = String::with_capacity(str.len() * 4);
    for i in 0..str.len() {
        let s = u8::from_str_radix(&str[i..i+1], 16)?;
        results.extend(format!("{:0>4b}", s).chars());
    }
    Ok(results)
}

fn convert_bit_string_to_packets(bits: String) -> Packet {
    convert_bits_to_packet(bits.as_str())
}

fn convert_bits_to_packet(bits: &str) -> Packet {
    let header = convert_bits_to_header(&bits[0..6]);

    let version = header >> 3;
    let packet_type = header & 0b111;
    let body = convert_bits_to_body(packet_type, &bits[6..]);

    Packet::from(version, packet_type, body)
}

fn convert_bits_to_header(bits: &str) -> u8 {
    u8::from_str_radix(&bits[..], 2).unwrap()
}

fn convert_bits_to_u16(bits: &str) -> u16 {
    u16::from_str_radix(&bits[..], 2).unwrap()
}

fn convert_bits_to_literal(bits: &str) -> PacketBody {
    let mut bit_string = String::new();
    let mut i = 0;
    while i < bits.len() {
        bit_string.push_str(&bits[i+1..i+5]);

        if &bits[i..i+1] == "0" { break; }

        i += 5;
    }

    PacketBody::Literal(convert_bits_to_u16(bit_string.as_str()))
}

fn convert_bits_to_operations(bits: &str) -> PacketBody {
    todo!()
}

fn convert_bits_to_body(packet_type: u8, bits: &str) -> PacketBody {
    match packet_type {
        4 => convert_bits_to_literal(bits),
        _ => convert_bits_to_operations(bits)
    }
}

impl Puzzle<Input, Output> for Day16 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        todo!()
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        todo!()
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_example_1() {
        let bits = convert_hex_to_bits("38006F45291200").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(packet, Packet::operation(0b001110,
                vec![
                    Packet::literal(0b110100, 10),
                    Packet::literal(0b010100, 20)
                ]
            ));
    }

    #[test]
    fn test_example_2() {
        let bits = convert_hex_to_bits("EE00D40C823060").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(packet, Packet::operation(0b111011,
                vec![
                    Packet::literal(0b010100, 1),
                    Packet::literal(0b100100, 2),
                    Packet::literal(0b001100, 3)
                ]
            ));
    }

    #[test]
    fn test_example_3() {
        let bits = convert_hex_to_bits("D2FE28").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(packet, Packet::literal(0b110100, 2021));
    }

    #[test]
    fn test_conversion_1() {
        assert_eq!(convert_hex_to_bits("D2FE28").unwrap(), "110100101111111000101000");
    }

    #[test]
    fn test_conversion_2() {
        assert_eq!(convert_hex_to_bits("38006F45291200").unwrap(), "00111000000000000110111101000101001010010001001000000000");
    }

    #[test]
    fn test_calculate_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_one("./inputs/day_16_test.in", Box::new(Day16))?);
        Ok(())
    }

    #[test]
    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_16_test.in", Box::new(Day16))?);
        Ok(())
    }
}