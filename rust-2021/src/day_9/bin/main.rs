use std::collections::{BinaryHeap, BTreeSet};
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_9.in", Box::new(Day9 { }))
}

struct Day9 {}

type Input = Vec<Vec<u8>>;
type Output = u32;

fn find_low_points(input: &Input) -> Vec<(usize,  usize)> {
    let mut results = vec![];

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

            results.push((x as usize, y as usize));
            x += 1;
        }

        y += 1;
    }

    results
}

fn crawl(x: i32, y: i32, input: &Input, seen: &mut BTreeSet<(i32, i32)>) -> Vec<u32> {
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    if y < 0 || y >= height || x < 0 || x >= width {
        return vec![];
    }

    if seen.contains(&(x, y)) {
        return vec![];
    }

    let digit = convert_u8(input[y as usize][x as usize]);

    if digit == 9 {
        return vec![];
    }

    seen.insert((x, y));

    let mut results = vec![];

    results.push(digit);

    results.append(&mut crawl(x - 1, y, input, seen));
    results.append(&mut crawl(x + 1, y, input, seen));
    results.append(&mut crawl(x, y - 1, input, seen));
    results.append(&mut crawl(x, y + 1, input, seen));

    return results;
}

fn calc_basin_size(x: usize, y: usize, input: &Input) -> u32 {
    crawl(x as i32, y as i32, input, &mut BTreeSet::new())
        .len() as u32
}

fn convert_u8(digit: u8) -> u32 {
    (digit as char).to_digit(10).unwrap()
}

impl Puzzle<Input, Output> for Day9 {

    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        Ok(contents.iter()
            .map(|s| s.bytes().collect::<Vec<_>>())
            .collect())
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let low_points = find_low_points(input);

        let mut results = 0;

        for (x, y) in low_points {
            results += 1 + convert_u8(input[y][x]);
        }

        results
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let low_points = find_low_points(input);
        let mut heap = BinaryHeap::new();

        for (x, y) in low_points {
            heap.push(calc_basin_size(x, y, input));
        }

        let mut results = 1;
        let mut counter = 0;

        while let Some(size) = heap.pop() {
            if counter == 3 { break; }

            results *= size;
            counter += 1;
        }

        results
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
        assert_eq!(1134, run_part_two("./inputs/day_9_test.in", Box::new(Day9 { }))?);
        Ok(())
    }
}