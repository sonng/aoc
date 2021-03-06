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

    let parsed_input = parse(contents)?;

    calculate_part_1(&parsed_input);
    calculate_part_2(&parsed_input);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Reading {
    val: isize,
    size: usize
}

fn parse(input: Vec<String>) -> Result<Vec<Reading>, Box<dyn Error>> {
    let mut results = vec![];
    for l in input {
        results.push(Reading {
            val: isize::from_str_radix(&l[..], 2)?,
            size: l.len()
        })
    }
    Ok(results)
}

fn calculate_part_1(input: &Vec<Reading>) {
    let mut readings = vec![0; input[0].size];

    // Count bits
    for reading in input {
        let mut g = reading.val.clone();
        for i in 0..reading.size {
            if g & 1 == 1 {
                readings[i] += 1;
            }

            g = g >> 1;
        }
    }

    let total = input.len();
    let (g, e) = readings.into_iter().enumerate()
        .fold((0, 0), |(g, e), (i, n) | {
            if n >= total - n {
                (g + (1 << i), e)
            } else {
                (g, e + (1 << i))
            }
        });

    println!("{:?}", g * e);
}

fn find(input: &Vec<Reading>, cursor: usize, common: bool) -> Vec<Reading> {
    if input.len() == 1 { return input.clone(); }

    let count = input.into_iter()
        .filter(|reading| (reading.val >> cursor) & 1 == 1)
        .count();

    let keep = if count >= (input.len() - count) {
        if common { 1 } else { 0 }
    } else {
        if common { 0 } else { 1 }
    };

    input.into_iter().cloned()
        .filter(|reading| ((reading.val >> cursor) & 1) ^ keep == 0)
        .collect()
}

fn calculate_part_2(input: &Vec<Reading>) {
    let mut oxygen = input.clone();
    for i in (0..input[0].size).rev() {
        oxygen = find(&oxygen, i, true);
    }

    let mut co2 = input.clone();
    for i in (0..input[0].size).rev() {
        co2 = find(&co2, i, false);
    }

    println!("{:?}", co2[0].val * oxygen[0].val);
}