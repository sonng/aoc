use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./rust-2021/inputs/day_3_input")?;
    let buf_reader = BufReader::new(file);

    let mut contents = vec![];
    for line in buf_reader.lines() {
        contents.push(line?);
    }

    let parsed_input = parse(contents);

    calculate_part_1(&parsed_input);
    calculate_part_2(&parsed_input);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Reading {
    val: isize,
    size: usize
}

fn parse(input: Vec<String>) -> Vec<Reading> {
    input.iter()
        .map(|l| {
            Reading {
                val: isize::from_str_radix(&l[..], 2).unwrap(),
                size: l.len()
            }

        })
        .collect()
}

fn calculate_part_1(input: &Vec<Reading>) {
    let total = input.len();
    let mut readings = vec![0; input[0].size];

    // Count bits
    for reading in input {
        let mut g = reading.val.clone();
        for i in 0..reading.size {
            if g & 1 == 1 {
                readings[i] += 1;
            }

            let l = g >> 1;
            g = g >> 1;
        }

    }

    // Reduce down
    let mut g = String::new();
    let mut e = String::new();
    for n in readings.into_iter().rev() {
        if (total - n) >= (total / 2) {
            g.push("1".parse().unwrap());
            e.push("0".parse().unwrap())
        } else {
            g.push("0".parse().unwrap());
            e.push("1".parse().unwrap());
        }
    }


    let g = isize::from_str_radix(&g, 2).unwrap();
    let e = isize::from_str_radix(&e, 2).unwrap();
    println!("{:?}", g * e);
}

fn find_oxygen(input: &Vec<Reading>, cursor: usize) -> Vec<Reading> {
    if input.len() == 1 {
        return input.clone();
    }

    let total = input.len();
    let mut count = 0;
    for reading in input.into_iter() {
        let mut g = reading.val.clone();
        if (g >> cursor) & 1 == 1 {
            count += 1;
        }
    }

    let most = if count >= (total - count) { 1 } else { 0 };
    let mut oxygen = vec![];
    for reading in input.into_iter() {
        if ((reading.val >> cursor) & 1) ^ most == 0 {
            oxygen.push(reading.clone());
        }
    }

    return oxygen;
}

fn find_co2(input: &Vec<Reading>, cursor: usize) -> Vec<Reading> {
    if input.len() == 1 {
        return input.clone();
    }

    let total = input.len();

    let mut count = 0;
    for reading in input.into_iter() {
        let mut g = reading.val.clone();
        if (g >> cursor) & 1 == 1 {
            count += 1;
        }
    }

    let least = if count >= total - count { 0 } else { 1 };
    let mut co2 = vec![];
    for reading in input.into_iter() {
        if ((reading.val >> cursor) & 1) ^ least == 0 {
            co2.push(reading.clone());
        }
    }

    return co2;
}

fn calculate_part_2(input: &Vec<Reading>) {
    let mut oxygen = input.clone();
    for i in (0..input[0].size).rev() {
        oxygen = find_oxygen(&oxygen, i);
    }

    let mut co2 = input.clone();
    for i in (0..input[0].size).rev() {
        co2 = find_co2(&co2, i);
    }

    println!("{:?}", co2[0].val * oxygen[0].val);
}