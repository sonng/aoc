
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), String> {
    let input = File::open("./rust-2021/inputs/day_1_1_input").expect("Can't load file");
    let mut buf_reader = BufReader::new(input);

    let numbers: Vec<i32> = buf_reader.lines()
        .map(|line| line.expect("Couldn't convert to string"))
        .map(|line| line.parse().expect("couldn't convert to i32"))
        .collect();

    part_1(&numbers);
    part_2(&numbers);

    Ok(())
}

fn part_1(input: &Vec<i32>) {
    let mut counter = 0;
    let mut prev = input[0];
    for &n in input {
        if n > prev {
            counter += 1;
        }

        prev = n;
    }

    println!("{:?}", counter);
}

fn part_2(input: &Vec<i32>) {
    let mut counter = 0;
    let mut cur_sum = 0;

    for i in 0..input.len() {
        if i < 3 {
            cur_sum += input[i];
            continue;
        }

        let new_sum = cur_sum - input[i - 3] + input[i];

        if new_sum > cur_sum {
            counter += 1;
        }

        cur_sum = new_sum;
    }

    println!("{:?}", counter);
}