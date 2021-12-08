use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let mut contents = vec![];
    for line in buf_reader.lines() {
        contents.push(line?);
    }

    Ok(contents)
}

pub trait Puzzle<Input, Output> {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>>;
    fn calculate_part_1(&self, input: &Input) -> Output;
    fn calculate_part_2(&self, input: &Input) -> Output;
}

pub fn run_all<Input, Output>(path: &str, puzzle: Box<dyn Puzzle<Input, Output>>) -> Result<(), Box<dyn Error>>
    where Output: Debug
{
    let contents = read_file(path)?;
    let input = puzzle.as_ref().parse(contents)?;

    println!("Part One");
    println!("{:?}", puzzle.as_ref().calculate_part_1(&input));
    println!("Part Two");
    println!("{:?}", puzzle.as_ref().calculate_part_2(&input));

    Ok(())
}

pub fn run_part_one<Input, Output>(path: &str, puzzle: Box<dyn Puzzle<Input, Output>>) -> Result<Output, Box<dyn Error>>
    where Output: Debug
{
    let contents = read_file(path)?;
    let input = puzzle.as_ref().parse(contents)?;

    Ok(puzzle.as_ref().calculate_part_1(&input))
}

pub fn run_part_two<Input, Output>(path: &str, puzzle: Box<dyn Puzzle<Input, Output>>) -> Result<Output, Box<dyn Error>>
    where Output: Debug
{
    let contents = read_file(path)?;
    let input = puzzle.as_ref().parse(contents)?;

    Ok(puzzle.as_ref().calculate_part_2(&input))
}