use std::collections::{HashMap, HashSet};
use std::error::Error;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_12.in", Box::new(Day12 { }))
}

struct Day12;

type Graph = HashMap<Node, HashSet<Node>>;
type Output = i32;

#[derive(Eq, Hash, Clone, Debug)]
struct Node(String);

impl Node {
    fn is_end(&self) -> bool { self.0.eq("end") }
    fn is_start(&self) -> bool { self.0.eq("start") }
    fn is_repeatable(&self) -> bool { self.0.chars().all(|c| c.is_uppercase() ) }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Day12 {

    fn traverse(&self, nodes: &Graph) -> Vec<Vec<Node>> {
        let mut paths = vec![];

        let mut stack = vec![];
        stack.push(vec![Node("start".to_string())]);

        while let Some(path) = stack.pop() {
            let node = &path[path.len() - 1];
            let child_nodes = &nodes[node];
            for child in child_nodes {
                if child.is_end() {
                    let mut end_path = path.clone();
                    end_path.push(child.clone());
                    paths.push(end_path);
                } else if child.is_repeatable() {
                    let mut branch = path.clone();
                    branch.push(child.clone());
                    stack.push(branch);
                } else if path.contains(&&child) || child.is_start() {
                    continue;
                } else {
                    let mut branch = path.clone();
                    branch.push(child.clone());
                    stack.push(branch);
                }
            }
        }

        paths
    }
}

impl Puzzle<Graph, Output> for Day12 {
    fn parse(&self, contents: Vec<String>) -> Result<Graph, Box<dyn Error>> {
        let mut connections = HashMap::new();
        for line in contents {
            let split = line.split('-').collect::<Vec<&str>>();

            let entry = connections.entry(Node(split[0].to_string())).or_insert(HashSet::new());
            entry.insert(Node(split[1].to_string()));

            let entry = connections.entry(Node(split[1].to_string())).or_insert(HashSet::new());
            entry.insert(Node(split[0].to_string()));
        }

        Ok(connections)
    }

    fn calculate_part_1(&self, input: &Graph) -> Output {
        self.traverse(input).len() as i32
    }

    fn calculate_part_2(&self, input: &Graph) -> Output {
        0
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_part_one_example_1() -> Result<(), Box<dyn Error>> {
        assert_eq!(10, run_part_one("./inputs/day_12_test.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_one_example_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(19, run_part_one("./inputs/day_12_test_1.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_one_example_3() -> Result<(), Box<dyn Error>> {
        assert_eq!(226, run_part_one("./inputs/day_12_test_2.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_12_test.in", Box::new(Day12 { }))?);
        Ok(())
    }

}