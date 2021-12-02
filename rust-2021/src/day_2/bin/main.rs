use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Command::Unknown;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./rust-2021/inputs/day_2_input")?;
    let buf_reader = BufReader::new(file);
    let mut contents = vec![];

    for line in buf_reader.lines() {
        let line = line?;
        let l = line.split(" ").collect::<Vec<&str>>();
        let cmd = l[0];
        let amount = l[1].parse::<i32>()?;
        contents.push(match (cmd, amount) {
            ("forward", l) => Command::Forward(l),
            ("down", l) => Command::Down(l),
            ("up", l) => Command::Up(l),
            _ => Unknown
        });
    }

    calculate_part_1(&contents);
    calculate_part_2(&contents);

    Ok(())
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Unknown
}

fn calculate_part_1(input: &Vec<Command>) {
    let calc = input.iter()
        .fold((0, 0), |(hoz, dep), command | {
            match &command {
                Command::Forward(u) => (hoz + u, dep),
                Command::Down(u) => (hoz, dep + u),
                Command::Up(u) => (hoz, dep - u),
                _ => (hoz, dep)
            }
        });

    println!("{:?}", calc.0 * calc.1);
}

fn calculate_part_2(input: &Vec<Command>) {
    let calc = input.iter()
        .fold((0, 0, 0), |(hoz, dep, aim), command | {
            match &command {
                Command::Down(u) => (hoz, dep, aim + u),
                Command::Up(u) => (hoz, dep, aim - u),
                Command::Forward(u) => (hoz + u, dep + aim * u, aim),
                _ => (hoz, dep, aim)
            }
        });

    println!("{:?}", calc.0 * calc.1);
}