use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_19.in", Box::new(Day19))
}

struct Day19;

type Input = Vec<Scanner>;
type Output = i32;

#[derive(Clone)]
struct Coord { x: i32, y: i32, z: i32 }

struct Scanner {
    beacons: Vec<Coord>
}

impl Puzzle<Input, Output> for Day19 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let mut scanners = vec![];
        let mut coords = vec![];

        for line in contents {
            if line.is_empty() {
                scanners.push(Scanner { beacons: coords.clone() });
            } else if line.starts_with("---") {
                coords = vec![];
            } else {
                let split = line.split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                coords.push(Coord { x: split[0], y: split[1], z: split[2] });
            }
        }

        if !coords.is_empty() {
            scanners.push(Scanner { beacons: coords.clone() });
        }

        Ok(scanners)
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

    use super::*;


}