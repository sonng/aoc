use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./rust-2021/inputs/day_2_input")?;
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

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn parse(input: Vec<String>) -> Vec<Command> {
    input.into_iter()
        .map(|line| {
            let l = line.split(" ").collect::<Vec<&str>>();
            let cmd = l[0];
            let amount = l[1].parse::<i32>().expect("can't parse");

            match (cmd, amount) {
                ("forward", l) => Some(Command::Forward(l)),
                ("down", l) => Some(Command::Down(l)),
                ("up", l) => Some(Command::Up(l)),
                _ => None
            }
        })
        .filter_map(|e| e)
        .collect()
}

fn calculate_part_1(input: &Vec<Command>) {
    let calc = input.iter()
        .fold((0, 0), |(hoz, dep), command | {
            match &command {
                Command::Forward(u) => (hoz + u, dep),
                Command::Down(u) => (hoz, dep + u),
                Command::Up(u) => (hoz, dep - u),
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
            }
        });

    println!("{:?}", calc.0 * calc.1);
}