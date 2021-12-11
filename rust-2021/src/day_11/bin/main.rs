use std::collections::{HashSet};
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_11.in", Box::new(Day11 { max_steps: 100 }))
}

static FLASH_POINT: u32 = 9;
static RESET_POINT: u32 = 0;

type Coord = (usize, usize);
type Grid = Vec<Vec<u32>>;
type Output = i64;

struct Day11 {
    max_steps: i32
}

impl Day11 {

    fn take_step(&self, grid: &mut Grid) {
        for row in grid.iter_mut() {
            for col in row.iter_mut() {
                *col += 1;
            }
        }
    }

    fn still_more_reaction(&self, grid: &Grid) -> bool {
        grid.iter()
            .any(|cols| cols.iter().any(|i| i > &FLASH_POINT))
    }

    fn react(&self, grid: &mut Grid, flashed: &mut HashSet<Coord>) {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        for y in 0..height {
            for x in 0..width {
                if let Some((x_usize, y_usize)) = self.will_explode(x, y, grid, flashed) {
                    self.try_move(x + 1, y, grid, flashed);
                    self.try_move(x - 1, y, grid, flashed);
                    self.try_move(x, y - 1, grid, flashed);
                    self.try_move(x, y + 1, grid, flashed);

                    self.try_move(x - 1, y - 1, grid, flashed);
                    self.try_move(x + 1, y - 1, grid, flashed);
                    self.try_move(x - 1, y + 1, grid, flashed);
                    self.try_move(x + 1, y + 1, grid, flashed);

                    grid[y_usize][x_usize] = RESET_POINT;
                    flashed.insert((x_usize, y_usize));
                }
            }
        }
    }

    fn will_explode(&self, x: i32, y: i32, grid: &Grid, flashed: &HashSet<Coord>) -> Option<Coord> {
        let x = x as usize;
        let y = y as usize;

        if flashed.contains(&(x, y)) { return None; }

        return if grid[y][x] > FLASH_POINT {
            Some((x, y))
        } else {
            None
        }
    }

    fn try_move(&self, x: i32, y: i32, grid: &mut Grid, flashed: &mut HashSet<Coord>) {
        if let Some((x, y)) = self.can_move(x, y, grid, flashed) {
            grid[y][x] += 1;
        }
    }

    fn can_move(&self, x: i32, y: i32, grid: &Grid, flashed: &HashSet<Coord>) -> Option<Coord> {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        if y >= height || y < 0 || x >= width || x < 0 { return None; }

        let x = x as usize;
        let y = y as usize;
        return if flashed.contains(&(x, y)) {
            None
        } else {
            Some((x, y))
        }
    }
}

impl Puzzle<Grid, Output> for Day11 {
    fn parse(&self, contents: Vec<String>) -> Result<Grid, Box<dyn Error>> {
        Ok(contents.iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>())
    }

    fn calculate_part_1(&self, input: &Grid) -> Output {
        let mut grid = input.clone();
        let mut results = 0;


        for _ in 0..self.max_steps {
            self.take_step(&mut grid);

            let mut flashes = HashSet::<Coord>::new();
            while self.still_more_reaction(&grid) {
                self.react(&mut grid, &mut flashes);
            }
            results += flashes.len() as i64;
        }

        results
    }

    fn calculate_part_2(&self, input: &Grid) -> Output {
        let mut grid = input.clone();

        for i in 1..i32::MAX {
            self.take_step(&mut grid);

            let mut flashes = HashSet::<Coord>::new();
            while self.still_more_reaction(&grid) {
                self.react(&mut grid, &mut flashes);
            }

            if flashes.len() == 100 {
                return i as i64;
            }
        }

        return 0;
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{ run_part_one, run_part_two };
    use super::*;

    #[test]
    fn test_calculate_part_1_100() -> Result<(), Box<dyn Error>> {
        assert_eq!(1656, run_part_one("./inputs/day_11_test.in", Box::new(Day11 { max_steps: 100 }))?);
        Ok(())
    }

    #[test]
    fn test_calculate_part_1_10() -> Result<(), Box<dyn Error>> {
        assert_eq!(204, run_part_one("./inputs/day_11_test.in", Box::new(Day11 { max_steps: 10 }))?);
        Ok(())
    }

    #[test]
    fn test_calculate_part_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(195, run_part_two("./inputs/day_11_test.in", Box::new(Day11 { max_steps: 10 }))?);
        Ok(())
    }
}