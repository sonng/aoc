use std::error::Error;
use std::u8;
use aoc_utils::{Puzzle, run_all};
use crate::PacketBody::Literal;

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_16.in", Box::new(Day16))
}

struct Day16;

type Input = Packet;
type Output = u64;

#[derive(PartialEq, Debug)]
enum PacketBody {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equal(Vec<Packet>)
}

impl PacketBody {
    fn from(packet_type: u8, packets: Vec<Packet>) -> Self {
        match packet_type {
            0 => PacketBody::Sum(packets),
            1 => PacketBody::Product(packets),
            2 => PacketBody::Minimum(packets),
            3 => PacketBody::Maximum(packets),
            5 => PacketBody::GreaterThan(packets),
            6 => PacketBody::LessThan(packets),
            7 => PacketBody::Equal(packets),
            _ => panic!("unknown operator")
        }
    }

    fn value(&self) -> u64 {
        match self {
            PacketBody::Literal(v) => v.clone(),
            PacketBody::Equal(p) => if p[0].value() == p[1].value() { 1 } else { 0 },
            PacketBody::LessThan(p) => if p[0].value() < p[1].value() { 1 } else { 0 },
            PacketBody::GreaterThan(p) => if p[0].value() > p[1].value() { 1 } else { 0 },
            PacketBody::Maximum(p) => p.iter().map(|p| p.value()).max().unwrap_or_default(),
            PacketBody::Minimum(p) => p.iter().map(|p| p.value()).min().unwrap_or_default(),
            PacketBody::Product(p) => p.iter().map(|p| p.value()).product(),
            PacketBody::Sum(p) => p.iter().map(|p| p.value()).sum()
        }
    }

    fn sum_of_versions(&self) -> u64 {
        if let Literal(_) = self {
            return 0;
        }

        let packets = match &self {
            PacketBody::Sum(packets) => packets,
            PacketBody::Product(packets) => packets,
            PacketBody::Minimum(packets) => packets,
            PacketBody::Maximum(packets) => packets,
            PacketBody::GreaterThan(packets) => packets,
            PacketBody::LessThan(packets) => packets,
            PacketBody::Equal(packets) => packets,
            _ => panic!("it shouldn't go here")
        };

        packets.iter().map(|p| p.sum_of_versions()).sum()
    }
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u8,
    packet_type: u8,
    body: PacketBody
}

impl Packet {
    fn literal(header: u8, literal: u64) -> Self {
        Packet { version: header >> 3, packet_type: header & 0b111, body: PacketBody::Literal(literal) }
    }

    fn operation(header: u8, packets: Vec<Packet>) -> Self {
        Packet { version: header >> 3, packet_type: header & 0b111, body: PacketBody::from(header & 0b111, packets) }
    }

    fn from(version: u8, packet_type: u8, body: PacketBody) -> Self {
        Packet { version, packet_type, body }
    }

    fn sum_of_versions(&self) -> u64 {
        (self.version as u64) + &self.body.sum_of_versions()
    }

    fn value(&self) -> u64 {
        self.body.value()
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

fn convert_bits_to_literal_num(bits: &str) -> u64 {
    u64::from_str_radix(&bits[..], 2).unwrap()
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

    (PacketBody::Literal(convert_bits_to_literal_num(bit_string.as_str())), i)
}

fn convert_bits_to_operations(bits: &str, packet_type: u8, cur: usize) -> (PacketBody, usize) {
    let length_type = &bits[cur..cur+1];

    if length_type == "0" {
        let length = convert_bits_to_literal_num(&bits[cur+1..cur+16]) as usize;
        let mut cur = cur+16; // we've read the length
        let end =  cur + length;
        let mut packets = vec![];
        while cur < end {
            let (packet, read) = parse_packet(bits, cur);
            packets.push(packet);
            cur = read;
        }
        (PacketBody::from(packet_type, packets), cur)
    } else {
        let length = convert_bits_to_literal_num(&bits[cur+1..cur+12]) as usize;
        let mut cur = cur+12;
        let mut packets = vec![];
        for _ in 0..length {
            let (packet, read) = parse_packet(bits, cur);
            packets.push(packet);
            cur = read;
        }
        (PacketBody::from(packet_type, packets), cur)
    }
}

fn convert_bits_to_body(packet_type: u8, bits: &str, cur: usize) -> (PacketBody, usize) {
    match packet_type {
        4 => parse_literal(bits, cur),
        _ => convert_bits_to_operations(bits, packet_type, cur)
    }
}

impl Puzzle<Input, Output> for Day16 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let bits = convert_hex_to_bits(contents[0].as_str()).unwrap();
        let packet = convert_bit_string_to_packets(bits);
        Ok(packet)
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        input.sum_of_versions()
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        input.value()
    }
}

#[cfg(test)]
mod test {
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
    fn test_sum_of_versions_1() {
        let bits = convert_hex_to_bits("8A004A801A8002F478").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(16, packet.sum_of_versions())
    }

    #[test]
    fn test_sum_of_versions_2() {
        let bits = convert_hex_to_bits("620080001611562C8802118E34").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(12, packet.sum_of_versions())
    }

    #[test]
    fn test_sum_of_versions_3() {
        let bits = convert_hex_to_bits("C0015000016115A2E0802F182340").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(23, packet.sum_of_versions())
    }

    #[test]
    fn test_sum_of_versions_4() {
        let bits = convert_hex_to_bits("A0016C880162017C3686B18A3D4780").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(31, packet.sum_of_versions())
    }

    #[test]
    fn test_process_example_sum() {
        let bits = convert_hex_to_bits("C200B40A82").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(3, packet.value())
    }

    #[test]
    fn test_process_example_product() {
        let bits = convert_hex_to_bits("04005AC33890").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(54, packet.value())
    }

    #[test]
    fn test_process_example_minimum() {
        let bits = convert_hex_to_bits("880086C3E88112").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(7, packet.value())
    }

    #[test]
    fn test_process_example_maximum() {
        let bits = convert_hex_to_bits("CE00C43D881120").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(9, packet.value())
    }

    #[test]
    fn test_process_example_less_than() {
        let bits = convert_hex_to_bits("D8005AC2A8F0").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(1, packet.value())
    }

    #[test]
    fn test_process_example_greater_than() {
        let bits = convert_hex_to_bits("F600BC2D8F").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(0, packet.value())
    }

    #[test]
    fn test_process_example_equal() {
        let bits = convert_hex_to_bits("9C005AC2F8F0").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(0, packet.value())
    }

    #[test]
    fn test_process_example_equal_combine() {
        let bits = convert_hex_to_bits("9C0141080250320F1802104A08").unwrap();
        let packet = convert_bit_string_to_packets(bits);
        assert_eq!(1, packet.value())
    }

    #[test]
    fn test_conversion_1() {
        assert_eq!(convert_hex_to_bits("D2FE28").unwrap(), "110100101111111000101000");
    }

    #[test]
    fn test_conversion_2() {
        assert_eq!(convert_hex_to_bits("38006F45291200").unwrap(), "00111000000000000110111101000101001010010001001000000000");
    }
}