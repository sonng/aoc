use std::collections::HashMap;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_14.in", Box::new(Day14 { }))
}

struct Day14;

#[derive(Debug, Clone)]
struct PolymerizationEquipment {
    template: Vec<u8>,
    rules: HashMap<(u8, u8), u8>
}

impl PolymerizationEquipment {
    fn grow(&mut self) {
        let mut new_template = vec![];
        for i in 0..self.template.len() - 1 {
            new_template.push(self.template[i]);
            new_template.push(self.rules[&(self.template[i], self.template[i+1])]);
        }

        new_template.push(self.template[self.template.len() - 1]);

        self.template = new_template;
    }
}

type Input = PolymerizationEquipment;
type Output = i32;

impl Puzzle<Input, Output> for Day14 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let template = contents[0].bytes().collect::<Vec<_>>();
        let mut rules = HashMap::new();

        for i in 2..contents.len() {
            let rule_split = contents[i].split(' ')
                .map(|s| s.bytes().collect::<Vec<_>>())
                .collect::<Vec<Vec<u8>>>();
            let rule = (rule_split[0][0], rule_split[0][1]);
            let action = rule_split[2][0];
            rules.insert(rule, action);
        }

        Ok(PolymerizationEquipment { template, rules })
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut input = input.clone();
        for _ in 0..10 {
            input.grow();
        }

        let mut occurances = HashMap::new();
        for u in input.template {
            *occurances.entry(u).or_insert(0) += 1;
        }

        let mut nums = vec![];
        for (_, value) in occurances {
            nums.push(value);
        }
        nums.sort();

        nums[nums.len() - 1] - nums[0]
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
    fn test_calculate_part_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(1588, run_part_one("./inputs/day_14_test.in", Box::new(Day14 { }))?);
        Ok(())
    }

    #[test]
    fn test_calculate_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_14_test.in", Box::new(Day14 { }))?);
        Ok(())
    }
}