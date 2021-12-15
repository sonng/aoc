use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_15.in", Box::new(Day15 { }))
}

struct Day15;

type Input = Vec<i32>;
type Output = i32;

impl Puzzle<Input, Output> for Day15 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        Ok(vec![])
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        0
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        0
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_calculate_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_one("./inputs/day_15_test.in", Box::new(Day15 {}))?);
        Ok(())
    }

    #[test]
    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_15_test.in", Box::new(Day15 {}))?);
        Ok(())
    }
}