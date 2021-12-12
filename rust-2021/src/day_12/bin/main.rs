use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_12.in", Box::new(Day12 { }))
}

struct Day12;

type Input = Vec<i32>;
type Output = i32;

impl Puzzle<Input, Output> for Day12 {
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
    fn test_part_one_example_1() -> Result<(), Box<dyn Error>> {
        assert_eq!(10, run_part_one("./inputs/day_12_test.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_one_example_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(19, run_part_one("./inputs/day_12_test_1.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_one_example_3() -> Result<(), Box<dyn Error>> {
        assert_eq!(226, run_part_one("./inputs/day_12_test_2.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_12_test.in", Box::new(Day12 { }))?);
        Ok(())
    }

}