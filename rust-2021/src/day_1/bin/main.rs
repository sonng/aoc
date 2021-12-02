use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./rust-2021/inputs/day_1_1_input")?;
    let buf_reader = BufReader::new(file);
    let mut contents = vec![];

    for line in buf_reader.lines() {
        contents.push(line?);
    }

    let parsed_input = parse(contents);

    part_1(&parsed_input);
    part_2(&parsed_input);

    Ok(())
}

fn parse(input: Vec<String>) -> Vec<i32> {
    input.into_iter()
        .map(|l| l.parse().expect("can't parse into i32"))
        .collect()
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
