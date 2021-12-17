use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_17.in", Box::new(Day17))
}

struct Day17;

type Input = Vec<i32>;
type Output = i32;

impl Puzzle<Input, Output> for Day17 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        todo!()
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        todo!()
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        todo!()
    }
}

#[test]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_calculate_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_one("./inputs/day_17_test.in", Box::new(Day17))?);
        Ok(())
    }

    #[test]
    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_17_test.in", Box::new(Day17))?);
        Ok(())
    }
}