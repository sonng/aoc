use std::collections::HashMap;
use std::error::Error;
use aoc_utils::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_file("./rust-2021/inputs/day_6_input")?;
    let parsed_input = parse(contents)?;

    println!("{:?}", calculate_part_one(&parsed_input));
    println!("{:?}", calculate_part_two(&parsed_input));

    Ok(())
}

type ParsedInput = Vec<Fish>;
type Solution = u64;

#[derive(Clone, Eq, Hash)]
struct Fish {
    birth: u64,
    timer: u64
}

impl PartialEq for Fish {
    fn eq(&self, other: &Fish) -> bool {
        self.birth == other.birth && self.timer == other.timer
    }
}

impl Fish {
    fn cycle(&self, max_days: u64, memo: &mut HashMap<Fish, u64>) -> u64 {
        if memo.contains_key(self) {
            return memo[&self];
        }

        let mut timer = self.timer;
        let mut results = 1;
        for i in self.birth..max_days {
            if timer == 0 {
                let new_fish = Fish { birth: i+1, timer: 8 };
                results += new_fish.cycle(max_days, memo);

                timer = 6;
            } else {
                timer -= 1;
            }
        }

        memo.insert(self.clone(), results);
        results
    }
}

fn parse(contents: Vec<String>) -> Result<ParsedInput, Box<dyn Error>> {
    Ok(contents[0].split(',')
        .map(|l| l.parse::<u64>().unwrap())
        .map(|timer| Fish { birth: 0, timer })
        .collect::<Vec<Fish>>())
}

fn calculate_part_one(input: &ParsedInput) -> Solution {
    let mut memo = HashMap::new();
    let mut count = 0;
    for fish in input {
        count += fish.cycle(80, &mut memo);
    }

    count
}

fn calculate_part_two(input: &ParsedInput) -> Solution {
    let mut memo = HashMap::new();
    let mut count = 0;
    for fish in input {
        count += fish.cycle(256, &mut memo);
    }

    count
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calculate_part_1() -> Result<(), Box<dyn Error>> {
        let contents = read_file("./inputs/day_6_test_input")?;
        let input = parse(contents)?;

        assert_eq!(5934, calculate_part_one(&input));

        Ok(())
    }

    #[test]
    fn test_calculate_part_2() -> Result<(), Box<dyn Error>> {
        let contents = read_file("./inputs/day_6_test_input")?;
        let input = parse(contents)?;

        assert_eq!(26984457539, calculate_part_two(&input));

        Ok(())
    }
}