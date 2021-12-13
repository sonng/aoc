use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_13.in", Box::new(Day13 { }))
}

struct Day13;

type Input = Board;
type Output = usize;

#[derive(Debug, Clone)]
struct Board {
    width: i32, height: i32,
    walls: HashSet<Coord>,
    instructions: Vec<Fold>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord(i32, i32);

#[derive(Debug, Clone)]
enum Fold {
    Horizontal(i32),
    Vertical(i32)
}

impl Board {
    fn fold(&mut self, n: usize) {
        let mut walls = self.walls.clone();
        let mut to_add = vec![];
        let mut to_remove = vec![];

        match self.instructions[n] {
            Fold::Horizontal(axis) => {
                for point in walls {
                    if point.1 < axis { continue; }

                    let new_y = axis - (point.1 - axis);
                    if new_y >= 0 {
                        to_add.push(Coord(point.0, new_y));
                    }
                    to_remove.push(point);
                }
                self.height = axis;
            },
            Fold::Vertical(axis) => {
                for point in walls {
                    if point.0 < axis { continue; }

                    let new_x = axis - (point.0 - axis);
                    if new_x >= 0 {
                        to_add.push(Coord(new_x, point.1));
                    }
                    to_remove.push(point);
                }
                self.width = axis;
            }
        }

        for point in to_add { self.walls.insert(point); }
        for point in to_remove { self.walls.remove(&point); }
    }

    fn print_walls(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.walls.contains(&Coord(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Puzzle<Input, Output> for Day13 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();

        let mut i = 0;
        while i < contents.len() {
            if contents[i].is_empty() {
                i += 1;
                break;
            }
            let content = &contents[i];

            let coord_string = content.split(",").collect::<Vec<&str>>();
            let x = coord_string[0].parse::<i32>()?;
            let y = coord_string[1].parse::<i32>()?;
            width = max(width, x);
            height = max(height, y);

            walls.insert(Coord(x, y));
            i += 1;
        }

        let mut instructions = vec![];
        while i < contents.len() {
            let content = &contents[i];

            let instruction_string = content.split(" ").collect::<Vec<&str>>();
            let axis_string = instruction_string[2].split('=').collect::<Vec<&str>>();
            let axis_value = axis_string[1].parse::<i32>()?;

            let instruction = if axis_string[0] == "y" {
                Fold::Horizontal(axis_value)
            } else {
                Fold::Vertical(axis_value)
            };

            instructions.push(instruction);

            i += 1;
        }

        Ok(Board { width, height, walls, instructions })
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut input = input.clone();

        input.fold(0);

        input.walls.len()
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let mut input = input.clone();

        for i in 0..input.instructions.len() {
            input.fold(i);
        }

        input.print_walls();
        0
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(17, run_part_one("./inputs/day_13_test.in", Box::new(Day13 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_13_test.in", Box::new(Day13 { }))?);
        Ok(())
    }
}