use std::collections::HashMap;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

struct Day8;

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_8_input", Box::new(Day8 {}))
}

type Input = Vec<Segments>;
type Output = i32;

#[derive(Debug)]
struct Segments {
    signal_patterns: Vec<String>,
    digit_output: Vec<String>
}

fn parse_patterns(line: &String, should_sort: bool) -> Vec<String> {
    let mut results = line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s.chars().collect::<Vec<char>>();
            s.sort_by(|a, b| b.cmp(a));
            String::from_iter(s)
        })
        .collect::<Vec<String>>();

    if should_sort {
        results.sort_by(|a, b| a.len().cmp(&b.len()));
    }
    results
}
/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg


1 has 2 chars
7 has 3 chars
4 has 4 chars
8 has 7 chars

5 chars => 2, 3, 5
6 chars => 0, 6, 9

7 is found in 3
2 contains only 2 parts of 4
5 is the other one

9 contains all of 4
6 contains all of 5
0 is the other one
 */
impl Segments {

    fn find_mapping(&self) -> HashMap<String, String> {
        let mut digits_mapping = HashMap::<i32, String>::new();

        for signal in &self.signal_patterns {
            let signal = signal.clone();
            match signal.len() {
                2 => { digits_mapping.insert(1, signal); },
                3 => { digits_mapping.insert(7, signal); },
                4 => { digits_mapping.insert(4, signal); },
                5 => {
                    if digits_mapping[&7].chars().all(|c| signal.contains(c)){
                        digits_mapping.insert(3, signal);
                    } else if digits_mapping[&4].chars().filter(|c| signal.contains(c.clone())).count() == 2 {
                        digits_mapping.insert(2, signal);
                    } else {
                        digits_mapping.insert(5, signal);
                    }
                },
                6 => {
                    if digits_mapping[&4].chars().all(|c| signal.contains(c)) {
                        digits_mapping.insert(9, signal);
                    } else if digits_mapping[&5].chars().all(|c| signal.contains(c)) {
                        digits_mapping.insert(6, signal);
                    } else {
                        digits_mapping.insert(0, signal);
                    }
                },
                _ => { digits_mapping.insert(8, signal); }
            }
        }

        let mut swapped = HashMap::new();
        for (key, value) in digits_mapping {
            swapped.insert(value, format!("{}", key));
        }

        swapped
    }

    fn solve(&self) -> i32 {
        let mapping = self.find_mapping();
        let digits = self.digit_output.iter()
            .map(|digit| {
                mapping[digit].clone()
            })
            .collect::<String>();

        digits.parse::<i32>().unwrap()
    }
}

impl Puzzle<Input, Output> for Day8 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let mut results = vec![];
        for line in contents {
            let line_split = line.split('|')
                .map(String::from)
                .collect::<Vec<String>>();
            let signal_patterns = parse_patterns(&line_split[0], true);
            let digit_output = parse_patterns(&line_split[1], false);

            results.push(Segments { signal_patterns, digit_output })
        }

        Ok(results)
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut count = 0;
        for segment in input {
            for digit in &segment.digit_output {
                if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                    count += 1;
                }
            }
        }
        count
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        input.iter()
            .map(|segment| segment.solve() )
            .sum()
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(26, run_part_one("./inputs/day_8_test_input", Box::new(Day8 {}))?);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(61229, run_part_two("./inputs/day_8_test_input", Box::new(Day8 {}))?);

        Ok(())
    }
}