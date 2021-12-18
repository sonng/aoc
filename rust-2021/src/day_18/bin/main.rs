use std::error::Error;
use aoc_utils::{Puzzle, run_all};
use crate::NodeKind::{Branch, Value};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_18.in", Box::new(Day18))
}

struct Day18;

type Input = Node;
type Output = i64;

#[derive(Debug, PartialEq)]
enum NodeKind {
    Value(i64),
    Branch
}

type ChildNode = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
struct Node {
    left: ChildNode,
    right: ChildNode,
    kind: NodeKind
}

impl Node {
    fn new_value(value: i64) -> ChildNode {
        Some(Box::new(Node { left: None, right: None, kind: Value(value)}))
    }

    fn new_node(left: ChildNode, right: ChildNode) -> ChildNode {
        Some(Box::new(Node { left, right, kind: Branch }))
    }
}

fn parse_str_to_tree(s: &str, cur: usize) -> (Option<Box<Node>>, usize) {
    let cur_read_str = &s[cur..cur+1];
    return if cur_read_str == "[" {
        let (left, cur) = parse_str_to_tree(s, cur + 1);
        let (right, cur) = parse_str_to_tree(s, cur);
        (Node::new_node(left, right), cur)
    } else if cur_read_str == "," || cur_read_str == "]" {
        let (node, cur) = parse_str_to_tree(s, cur + 1);
        (node, cur)
    } else {
        let value = cur_read_str.parse::<i64>().unwrap();
        (Node::new_value(value), cur + 1)
    }
}

impl Puzzle<Input, Output> for Day18 {
    fn parse(&self, contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
        todo!()
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        todo!()
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{run_part_one, run_part_two};
    use super::*;

    #[test]
    fn test_parse_simple() {
        let expected = Node::new_node(
            Node::new_value(1),
            Node::new_value(2)
        );
        assert_eq!(expected, parse_str_to_tree("[1,2]", 0).0);
    }

    #[test]
    fn test_parse_left_nested() {
        let expected = Node::new_node(
            Node::new_node(Node::new_value(1), Node::new_value(2)),
            Node::new_value(3)
        );
        assert_eq!(expected, parse_str_to_tree("[[1,2],3]", 0).0);
    }

    #[test]
    fn test_parse_right_nested() {
        let expected = Node::new_node(
            Node::new_value(9),
            Node::new_node(Node::new_value(8), Node::new_value(7))
        );
        assert_eq!(expected, parse_str_to_tree("[9,[8,7]]", 0).0);
    }

    #[test]
    fn test_parse_left_and_right_nested() {
        let expected = Node::new_node(
            Node::new_node(Node::new_value(1), Node::new_value(9)),
            Node::new_node(Node::new_value(8), Node::new_value(5))
        );
        assert_eq!(expected, parse_str_to_tree("[[1,9],[8,5]]", 0).0);
    }

    #[test]
    fn test_parse_complex_nested_1() {
        let expected = Node::new_node(
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_value(1), Node::new_value(2)),
                    Node::new_node(Node::new_value(3), Node::new_value(4))
                ),
                Node::new_node(
                    Node::new_node(Node::new_value(5), Node::new_value(6)),
                    Node::new_node(Node::new_value(7), Node::new_value(8))
                )
            ),
            Node::new_value(9)
        );
        assert_eq!(expected, parse_str_to_tree("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]", 0).0);
    }

    #[test]
    fn test_parse_complex_nested_2() {
        let expected = Node::new_node(
            Node::new_node(
                Node::new_node(
                    Node::new_value(9),
                    Node::new_node(Node::new_value(3), Node::new_value(8))
                ),
                Node::new_node(
                    Node::new_node(Node::new_value(0), Node::new_value(9)),
                    Node::new_value(6)
                )
            ),
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_value(3), Node::new_value(7)),
                    Node::new_node(Node::new_value(4), Node::new_value(9))
                ),
                Node::new_value(3)
            )
        );
        assert_eq!(expected, parse_str_to_tree("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]", 0).0);
    }

    #[test]
    fn test_parse_complex_nested_3() {
        let expected = Node::new_node(
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_value(1), Node::new_value(3)),
                    Node::new_node(Node::new_value(5), Node::new_value(3))
                ),
                Node::new_node(
                    Node::new_node(Node::new_value(1), Node::new_value(3)),
                    Node::new_node(Node::new_value(8), Node::new_value(7))
                )
            ),
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_value(4), Node::new_value(9)),
                    Node::new_node(Node::new_value(6), Node::new_value(9))
                ),
                Node::new_node(
                    Node::new_node(Node::new_value(8), Node::new_value(2)),
                    Node::new_node(Node::new_value(7), Node::new_value(3))
                )
            )
        );
        assert_eq!(expected, parse_str_to_tree("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", 0).0);
    }


    fn test_calculate_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_one("./inputs/day_18_test.in", Box::new(Day18))?);
        Ok(())
    }

    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(0, run_part_two("./inputs/day_18_test.in", Box::new(Day18))?);
        Ok(())
    }
}