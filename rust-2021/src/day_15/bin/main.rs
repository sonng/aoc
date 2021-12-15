use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_15.in", Box::new(Day15 { }))
}

struct Day15;

type Input = Vec<Vec<u32>>;
type Output = u32;

#[derive(Eq, PartialEq, Clone, Copy)]
struct State {
    cost: u32,
    position: (i32, i32)
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl Day15 {
    fn find_shortest(&self, input: &Input) -> Output {
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

    fn find_shortest_any_movement(&self, input: &Input) -> Output {
        let height = input.len() as i32;
        let width = input[0].len() as i32;
        let end = (width - 1, height - 1);
        let mut seen_costs = vec![vec![u32::MAX; width as usize]; height as usize];

        let mut heap = BinaryHeap::new();
        heap.push(State { cost: 0, position: (0, 0) });

        while let Some(State { cost, position: (x, y) }) = heap.pop() {
            if (x, y) == end { return cost; }
            if cost > seen_costs[y as usize][x as usize] { continue; }

            let moves = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (move_x, move_y) in moves {
                if move_x < 0 || move_y < 0 || move_x >= width || move_y >= height { continue; }

                let next = State { cost: cost + input[move_y as usize][move_x as usize],
                                   position: (move_x, move_y) };

                if next.cost < seen_costs[move_y as usize][move_x as usize] {
                    heap.push(next);
                    seen_costs[move_y as usize][move_x as usize] = next.cost;
                }
            }
        }

        panic!("For some reason we can't find it");
    }
}

impl Puzzle<Input, Output> for Day15 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        Ok(contents.iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>())
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        self.find_shortest(input)
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let height = input.len();
        let width = input[0].len();

        let new_height = height * 5;
        let new_width = width * 5;

        let mut new_input = vec![vec![0 as u32; new_width]; new_height];

        for y in 0..new_height {
            for x in 0..new_width {
                let ref_y = y % height;
                let ref_x = x % width;

                let win_y = y / height;
                let win_x = x / width;

                if win_x == 0 && win_y == 0 {
                    new_input[y][x] = input[y][x];
                } else if win_x == 0 {
                    let value = (new_input[ref_y][x] + (1 * win_y as u32) - 1) % 9;
                    new_input[y][x] = value + 1;
                } else {
                    let value = (new_input[y][ref_x] + (1 * win_x as u32) - 1) % 9;
                    new_input[y][x] = value + 1;
                }
            }
        }

        self.find_shortest_any_movement(&new_input)
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
        assert_eq!(315, run_part_two("./inputs/day_15_test.in", Box::new(Day15 {}))?);
        Ok(())
    }
}