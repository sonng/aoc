use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;
use aoc_utils::{Puzzle, run_all};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_12.in", Box::new(Day12 { }))
}

struct Day12;

type Graph = HashMap<Rc<Node>, HashSet<Rc<Node>>>;
type Output = i32;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Node {
    Start, End, Big(String), Small(String)
}

impl Node {
    fn from(s: &str) -> Self {
        return if s.eq("start") {
            Node::Start
        } else if s.eq("end") {
            Node::End
        } else if s.to_string().chars().all(|c| c.is_lowercase()) {
            Node::Small(s.to_string())
        } else if s.to_string().chars().all(|c| c.is_uppercase()) {
            Node::Big(s.to_string())
        } else {
            panic!("invalid input");
        }
    }
}

impl Day12 {

    fn traverse(&self, nodes: &Graph, limit: usize) -> Vec<Vec<Rc<Node>>> {
        let mut paths = vec![];

        let mut stack = vec![];
        stack.push((vec![Rc::new(Node::Start)], false));

        while let Some((path, hit_limit)) = stack.pop() {
            let node = &path[path.len() - 1];
            let child_nodes = &nodes[node];
            for child in child_nodes {
                match child.as_ref() {
                    Node::End => {
                        let mut end_path = path.clone();
                        end_path.push(child.clone());
                        paths.push(end_path);
                    },
                    Node::Start => continue,
                    Node::Big(val) => {
                        let mut branch = path.clone();
                        branch.push(child.clone());
                        stack.push((branch, hit_limit));
                    },
                    Node::Small(val) => {
                        if hit_limit && path.contains(&child) { continue; }
                        let would_hit_limit = if hit_limit { hit_limit } else {
                            path.iter()
                            .filter(|n| n.eq(&child))
                            .count() + 1 >= limit
                        };

                        let mut branch = path.clone();
                        branch.push(child.clone());
                        stack.push((branch, would_hit_limit ));
                    }
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

            connections.entry(Rc::new(Node::from(split[0])))
                .or_insert(HashSet::new())
                .insert(Rc::new(Node::from(split[1])));

            connections.entry(Rc::new(Node::from(split[1])))
                .or_insert(HashSet::new())
                .insert(Rc::new(Node::from(split[0])));
        }

        Ok(connections)
    }

    fn calculate_part_1(&self, input: &Graph) -> Output {
        self.traverse(input, 1).len() as i32
    }

    fn calculate_part_2(&self, input: &Graph) -> Output {
        self.traverse(input, 2).len() as i32
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
    fn test_part_two_example_1() -> Result<(), Box<dyn Error>> {
        assert_eq!(36, run_part_two("./inputs/day_12_test.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_two_example_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(103, run_part_two("./inputs/day_12_test_1.in", Box::new(Day12 { }))?);
        Ok(())
    }

    #[test]
    fn test_part_two_example_3() -> Result<(), Box<dyn Error>> {
        assert_eq!(3509, run_part_two("./inputs/day_12_test_2.in", Box::new(Day12 { }))?);
        Ok(())
    }

}