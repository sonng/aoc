use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("./rust-2021/inputs/day_1_1_input")?;
    let mut buf_reader = BufReader::new(input);
    let mut numbers = vec![];

    for line in buf_reader.lines() {
        numbers.push(line?.parse::<i32>()?);
    }

    part_1(&numbers);
    part_2(&numbers);

    Ok(())
}

fn part_1(input: &Vec<i32>) {
    let counter = input.windows(2)
        .fold(0, |acc, window| if window[0] < window[1] {
            acc + 1
        } else { acc });

    println!("{:?}", counter);
}

fn part_2(input: &Vec<i32>) {
    let sums = input.windows(3)
        .map(|window| window.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    let counter = sums.windows(2)
        .fold(0, |acc, window| if window[0] < window[1] {
            acc + 1
        } else { acc });

    println!("{:?}", counter);
}
