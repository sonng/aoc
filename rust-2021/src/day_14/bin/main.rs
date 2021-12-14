use std::collections::HashMap;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_14.in", Box::new(Day14(10, 40)))
}

struct Day14(i32, i32);

#[derive(Debug, Clone)]
struct PolymerizationEquipment {
    template: HashMap<(u8, u8), i64>,
    template_raw: Vec<u8>,
    rules: HashMap<(u8, u8), u8>
}

impl PolymerizationEquipment {
    fn grow(&mut self) {
        let mut new_template = HashMap::new();

        for ((left, right), value) in &self.template {
            let spawn = &self.rules[&(*left, *right)];
            *new_template.entry((*left, *spawn)).or_insert(0) += value;
            *new_template.entry((*spawn, *right)).or_insert(0) += value;
        }

        self.template = new_template;
    }

    fn calculate(&self) -> i64 {
        let mut occurances = HashMap::new();
        for ((left, _), amount) in &self.template {
            *occurances.entry(left).or_insert(0) += amount;
        }

        *occurances.entry(&self.template_raw[self.template_raw.len() - 1]).or_insert(0) += 1;

        let mut nums = vec![];
        for (_, value) in occurances {
            nums.push(value);
        }
        nums.sort();

        nums[nums.len() - 1] - nums[0]
    }
}

type Input = PolymerizationEquipment;
type Output = i64;

impl Puzzle<Input, Output> for Day14 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        let template_raw = contents[0].bytes().collect::<Vec<_>>();
        let mut template = HashMap::new();
        for i in 0..template_raw.len() - 1 {
            *template.entry((template_raw[i], template_raw[i + 1])).or_insert(0) += 1;
        }

        let mut rules = HashMap::new();

        for i in 2..contents.len() {
            let rule_split = contents[i].split(' ')
                .map(|s| s.bytes().collect::<Vec<_>>())
                .collect::<Vec<Vec<u8>>>();
            let rule = (rule_split[0][0], rule_split[0][1]);
            let action = rule_split[2][0];
            rules.insert(rule, action);
        }

        Ok(PolymerizationEquipment { template, template_raw, rules })
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut input = input.clone();
        for _ in 0..self.0 {
            input.grow();
        }

        input.calculate()
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let mut input = input.clone();
        for _ in 0..self.1 {
            input.grow();
        }

        input.calculate()
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_calculate_part_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(1588, run_part_one("./inputs/day_14_test.in", Box::new(Day14(10, 40)))?);
        Ok(())
    }

    #[test]
    fn test_calculate_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(2188189693529, run_part_two("./inputs/day_14_test.in", Box::new(Day14(10, 40)))?);
        Ok(())
    }
}