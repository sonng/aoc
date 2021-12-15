use std::cmp::min;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_15.in", Box::new(Day15 { }))
}

struct Day15;

type Input = Vec<Vec<u32>>;
type Output = u32;

impl Puzzle<Input, Output> for Day15 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        Ok(contents.iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>())
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let height = input.len();
        let width = input[0].len();
        let mut dp = vec![vec![0 as u32; width]; height];

        let mut y = 0;
        while y < height as i32 {
            let yu = y as usize;
            let mut x = if y == 0 { 1 } else { 0 };
            while x < width as i32 {
                let xu = x as usize;

                if y == 0 {
                    dp[yu][xu] = dp[yu][xu - 1] + input[yu][xu];
                } else if x == 0 {
                    dp[yu][xu] = dp[yu - 1][xu] + input[yu][xu];
                } else {
                    dp[yu][xu] = min(dp[yu - 1][xu], dp[yu][xu - 1]) + input[yu][xu];
                }

                x += 1;
            }

            y += 1;
        }

        dp[height - 1][width - 1]
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
        assert_eq!(40, run_part_one("./inputs/day_15_test.in", Box::new(Day15 {}))?);
        Ok(())
    }

    #[test]
    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_15_test.in", Box::new(Day15 {}))?);
        Ok(())
    }
}