use std::error::Error;
use aoc_utils::{Puzzle, run_all};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_10.in", Box::new(Day10 {}))
}

type Input = Vec<Vec<u8>>;
type Output = i64;

struct Day10;



impl Puzzle<Input, Output> for Day10 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        Ok(contents.iter()
            .map(|s| s.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let score_mapping  = HashMap::<u8, (u8, i64)>::from([
            (')' as u8, ('(' as u8, 3)),
            (']' as u8, ('[' as u8, 57)),
            ('}' as u8, ('{' as u8, 1197)),
            ('>' as u8, ('<' as u8, 25137))
        ]);

        let mut results = 0;

        for line in input {
            let mut stack = vec![];
            for c in line {
                if score_mapping.contains_key(c) {
                    let (open, score) = score_mapping[c];
                    if let Some(last) = stack.last() {
                        if !open.eq(last) {
                            results += score;
                            break;
                        } else {
                            stack.pop();
                        }
                    }
                } else {
                    stack.push(c.clone());
                }
            }
        }

        results
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let score_mapping = HashMap::<u8, (u8, i64)>::from([
            (')' as u8, ('(' as u8, 1)),
            (']' as u8, ('[' as u8, 2)),
            ('}' as u8, ('{' as u8, 3)),
            ('>' as u8, ('<' as u8, 4))
        ]);
        let mapping = HashMap::<u8, u8>::from([
            ('(' as u8, ')' as u8),
            ('[' as u8, ']' as u8),
            ('{' as u8, '}' as u8),
            ('<' as u8, '>' as u8),
        ]);

        let mut scores = vec![];
        'outer: for line in input {
            let mut stack = vec![];
            for c in line {
                if score_mapping.contains_key(c) {
                    let (open, _) = score_mapping[c];
                    if let Some(last) = stack.last() {
                        if open.eq(last) {
                            stack.pop();
                        } else {
                            continue 'outer;
                        }
                    }
                } else {
                    stack.push(c.clone());
                }
            }

            let mut calc_score = 0;
            while let Some(item) = stack.pop() {
                let op = mapping[&item];
                let (_, score) = score_mapping[&op];
                calc_score = (calc_score * 5) + score;
            }

            scores.push(calc_score);
        }

        scores.sort();
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(26397, run_part_one("./inputs/day_10_test.in", Box::new(Day10 {}))?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(288957, run_part_two("./inputs/day_10_test.in", Box::new(Day10 {}))?);
        Ok(())
    }
}
