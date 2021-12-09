use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_9.in", Box::new(Day9 { }))
}

struct Day9 {}

type Input = Vec<Vec<u8>>;
type Output = u32;

impl Puzzle<Input, Output> for Day9 {

    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        Ok(contents.iter()
            .map(|s| s.bytes().collect::<Vec<_>>())
            .collect())
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut results = 0;

        let mut y: i32 = 0;
        while y < input.len() as i32 {
            let mut x: i32 = 0;
            while x < input[y as usize].len() as i32 {
                let digit = input[y as usize][x as usize];
                if ((y - 1 >= 0) && input[(y - 1) as usize][x as usize] <= digit) ||
                    ((x - 1 >= 0) && input[y as usize][(x - 1) as usize] <= digit) ||
                    ((x + 1 < input[y as usize].len() as i32) && input[y as usize][(x + 1) as usize] <= digit) ||
                    ((y + 1 < (input.len() as i32)) && input[(y + 1) as usize][x as usize] <= digit)
                    {
                    x += 1;
                    continue;
                }

                let digit = (digit as char).to_digit(10).unwrap();
                results += 1 + digit;
                x += 1;
            }

            y += 1;
        }

        results
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;


    #[test]
    fn test_calculate_part_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(15, run_part_one("./inputs/day_9_test.in", Box::new(Day9 { }))?);
        Ok(())
    }

    #[test]
    fn test_calculate_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_9_test.in", Box::new(Day9 { }))?);
        Ok(())
    }
}