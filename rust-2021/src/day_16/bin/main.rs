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
    parse_packet(bits.as_str(), 0).0
}

fn parse_packet(bits: &str, cur: usize) -> (Packet, usize) {
    let header = convert_bits_to_header(&bits[cur..cur+6]);

    let version = header >> 3;
    let packet_type = header & 0b111;
    let (body, read_up_to) = convert_bits_to_body(packet_type, &bits, cur+6);

    (Packet::from(version, packet_type, body), read_up_to)
}

fn convert_bits_to_header(bits: &str) -> u8 {
    u8::from_str_radix(&bits[..], 2).unwrap()
}

fn convert_bits_to_u16(bits: &str) -> u16 {
    u16::from_str_radix(&bits[..], 2).unwrap()
}

fn parse_literal(bits: &str, cur: usize) -> (PacketBody, usize) {
    let mut bit_string = String::new();
    let mut i = cur;
    while i < bits.len() {
        bit_string.push_str(&bits[i+1..i+5]);

        let should_break = &bits[i..i+1] == "0";
        i += 5;

        if should_break { break; }
    }

    (PacketBody::Literal(convert_bits_to_u16(bit_string.as_str())), i)
}

fn convert_bits_to_operations(bits: &str, cur: usize) -> (PacketBody, usize) {
    let length_type = &bits[cur..cur+1];

    if length_type == "0" {
        let length = convert_bits_to_u16(&bits[cur+1..cur+16]) as usize;
        let mut cur = cur+16; // we've read the length
        let end =  cur + length;
        let mut packets = vec![];
        while cur < end {
            let (packet, read) = parse_packet(bits, cur);
            packets.push(packet);
            cur = read;
        }
        (PacketBody::Operation(packets), cur)
    } else {
        let length = convert_bits_to_u16(&bits[cur+1..cur+12]) as usize;
        let mut cur = cur+12;
        let mut packets = vec![];
        for i in 0..length {
            let (packet, read) = parse_packet(bits, cur);
            packets.push(packet);
            cur = read;
        }
        (PacketBody::Operation(packets), cur)
    }
}

fn convert_bits_to_body(packet_type: u8, bits: &str, cur: usize) -> (PacketBody, usize) {
    match packet_type {
        4 => parse_literal(bits, cur),
        _ => convert_bits_to_operations(bits, cur)
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