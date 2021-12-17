use std::cmp::{max, min};
use std::error::Error;
use std::ops::Range;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_17.in", Box::new(Day17))
}

struct Day17;

type Input = TargetArea;
type Output = i64;

struct TargetArea(Range<i64>, Range<i64>);

fn parse_into_range(s: &str) -> Result<Range<i64>, Box<dyn Error>> {
    let split = s.split('=').collect::<Vec<_>>();
    let range_split = split[1].split("..").collect::<Vec<_>>();
    let left = range_split[0].parse()?;
    let right = range_split[1].parse::<i64>()?;

    Ok(Range { start: min(left, right), end: max(left, right) + 1 })
}

fn check(x_v: i64, y_v: i64, target_area: &TargetArea) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut x_v = x_v;
    let mut y_v = y_v;
    let mut height = 0;
    loop {
        if target_area.0.contains(&x) && target_area.1.contains(&y) {
            return Some(height);
        }

        if target_area.0.end < x || target_area.1.start > y {
            return None;
        }

        x += x_v;
        y += y_v;

        height = max(height, y);

        x_v = if x_v > 0 {
            x_v - 1
        } else if x_v < 0 {
            x_v + 1
        } else {
            0
        };

        y_v -= 1;
    }
}

impl Puzzle<Input, Output> for Day17 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let target_split = contents[0].split(':')
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        let ranges= target_split[1].split(',')
            .map(|s| s.trim())
            .map(|s| parse_into_range(s))
            .collect::<Result<Vec<Range<i64>>, Box<dyn Error>>>()?;

        Ok(TargetArea(ranges[0].clone(), ranges[1].clone()))
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut max_height = 0;
        for x in 0..input.0.end {
            for y in 0..input.0.end {
                if let Some(h) = check(x, y, input) {
                    max_height = max(max_height, h);
                }
            }
        }

        max_height
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let mut count = 0;
        for x in 0..input.0.end*2 {
            for y in input.1.start*2..input.0.end {
                if let Some(h) = check(x, y, input) {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_calculate_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(45, run_part_one("./inputs/day_17_test.in", Box::new(Day17))?);
        Ok(())
    }

    #[test]
    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(112, run_part_two("./inputs/day_17_test.in", Box::new(Day17))?);
        Ok(())
    }
}