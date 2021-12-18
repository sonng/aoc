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
    fn new_child_value(value: i64) -> ChildNode {
        Some(Node::new_value(value))
    }

    fn new_value(value: i64) -> Box<Node> {
        Box::new(Node { left: None, right: None, kind: Value(value) })
    }

    fn new_node(left: ChildNode, right: ChildNode) -> ChildNode {
        Some(Box::new(Node { left, right, kind: Branch }))
    }
}

impl Node {
    fn is_branch(&self) -> bool {
        if let Value(_) = self.kind { false } else { true }
    }

    fn more_branches(&self) -> bool {
        self.left.as_ref().map_or(false,|n| n.is_branch()) ||
            self.right.as_ref().map_or(false, |n| n.is_branch())
    }

    fn value(&self) -> i64 {
        match self.kind {
            Value(i) => i,
            _ => panic!("can't get a value out of a branch")
        }
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
        (Node::new_child_value(value), cur + 1)
    }
}

fn process(node: ChildNode) -> ChildNode {
    process_inner(node, 0).0
}

type Explosion = Option<(Option<i64>, Option<i64>)>;

fn process_inner(node: ChildNode, level: usize) -> (ChildNode, Explosion) {
    let mut explosion = None;
    (node.and_then(|mut n| {
        match n.kind {
            Value(_) => Some(n),
            Branch => {
                if level == 4 && !n.more_branches() {
                    let left = n.left.take().map(|n| n.value());
                    let right = n.right.take().map(|n| n.value());

                    explosion = Some((left, right));

                    Node::new_child_value(0)
                } else {
                    let process_left = process_inner(n.left, level + 1);
                    let left = process_left.0;
                    let left_explosion = process_left.1;

                    if let Some((ex_left, ex_right)) = left_explosion {
                        let right = process_and_find_most_left(n.right, ex_right);
                        explosion = Some((ex_left, None));

                        return Node::new_node(left, right);
                    }

                    let process_right = process_inner(n.right, level + 1);
                    let right = process_right.0;
                    let right_explosion = process_right.1;

                    if let Some((ex_left, ex_right)) = right_explosion {
                        let left = process_and_find_most_right(left, ex_left);
                        explosion = Some((None, ex_right));

                        return Node::new_node(left, right);
                    }

                    return Node::new_node(left, right);
                }
            }
        }
    }), explosion)
}

fn process_and_find_most_right(node: ChildNode, explosion: Option<i64>) -> ChildNode {
    match explosion {
        None => node,
        Some(explode_value) => {
            node.and_then(|n| {
                match n.kind {
                    Value(i) => Node::new_child_value(i + explode_value),
                    Branch => {
                        let right = process_and_find_most_right(n.right, Some(explode_value));
                        Node::new_node(n.left, right)
                    }
                }
            })
        }
    }
}

fn process_and_find_most_left(node: ChildNode, explosion: Option<i64>) -> ChildNode {
    match explosion {
        Some(explode_value) => {
            node.and_then(|n| {
                match n.kind {
                    Value(i) => Node::new_child_value(i + explode_value),
                    Branch => {
                        let left = process_and_find_most_left(n.left, Some(explode_value));
                        Node::new_node(left, n.right)
                    }
                }
            })
        },
        None => node
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
            Node::new_child_value(1),
            Node::new_child_value(2)
        );
        assert_eq!(expected, parse_str_to_tree("[1,2]", 0).0);
    }

    #[test]
    fn test_parse_left_nested() {
        let expected = Node::new_node(
            Node::new_node(Node::new_child_value(1), Node::new_child_value(2)),
            Node::new_child_value(3)
        );
        assert_eq!(expected, parse_str_to_tree("[[1,2],3]", 0).0);
    }

    #[test]
    fn test_parse_right_nested() {
        let expected = Node::new_node(
            Node::new_child_value(9),
            Node::new_node(Node::new_child_value(8), Node::new_child_value(7))
        );
        assert_eq!(expected, parse_str_to_tree("[9,[8,7]]", 0).0);
    }

    #[test]
    fn test_parse_left_and_right_nested() {
        let expected = Node::new_node(
            Node::new_node(Node::new_child_value(1), Node::new_child_value(9)),
            Node::new_node(Node::new_child_value(8), Node::new_child_value(5))
        );
        assert_eq!(expected, parse_str_to_tree("[[1,9],[8,5]]", 0).0);
    }

    #[test]
    fn test_parse_complex_nested_1() {
        let expected = Node::new_node(
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_child_value(1), Node::new_child_value(2)),
                    Node::new_node(Node::new_child_value(3), Node::new_child_value(4))
                ),
                Node::new_node(
                    Node::new_node(Node::new_child_value(5), Node::new_child_value(6)),
                    Node::new_node(Node::new_child_value(7), Node::new_child_value(8))
                )
            ),
            Node::new_child_value(9)
        );
        assert_eq!(expected, parse_str_to_tree("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]", 0).0);
    }

    #[test]
    fn test_parse_complex_nested_2() {
        let expected = Node::new_node(
            Node::new_node(
                Node::new_node(
                    Node::new_child_value(9),
                    Node::new_node(Node::new_child_value(3), Node::new_child_value(8))
                ),
                Node::new_node(
                    Node::new_node(Node::new_child_value(0), Node::new_child_value(9)),
                    Node::new_child_value(6)
                )
            ),
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_child_value(3), Node::new_child_value(7)),
                    Node::new_node(Node::new_child_value(4), Node::new_child_value(9))
                ),
                Node::new_child_value(3)
            )
        );
        assert_eq!(expected, parse_str_to_tree("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]", 0).0);
    }

    #[test]
    fn test_parse_complex_nested_3() {
        let expected = Node::new_node(
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_child_value(1), Node::new_child_value(3)),
                    Node::new_node(Node::new_child_value(5), Node::new_child_value(3))
                ),
                Node::new_node(
                    Node::new_node(Node::new_child_value(1), Node::new_child_value(3)),
                    Node::new_node(Node::new_child_value(8), Node::new_child_value(7))
                )
            ),
            Node::new_node(
                Node::new_node(
                    Node::new_node(Node::new_child_value(4), Node::new_child_value(9)),
                    Node::new_node(Node::new_child_value(6), Node::new_child_value(9))
                ),
                Node::new_node(
                    Node::new_node(Node::new_child_value(8), Node::new_child_value(2)),
                    Node::new_node(Node::new_child_value(7), Node::new_child_value(3))
                )
            )
        );
        assert_eq!(expected, parse_str_to_tree("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", 0).0);
    }

    #[test]
    fn test_explosion_left_most_explosion() {
        let pre_explosion = parse_str_to_tree("[[[[[9,8],1],2],3],4]", 0).0;
        let post_explosion = parse_str_to_tree("[[[[0,9],2],3],4]", 0).0;
        let action = process(pre_explosion);
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_right_most_explosion() {
        let pre_explosion = parse_str_to_tree("[7,[6,[5,[4,[3,2]]]]]", 0).0;
        let post_explosion = parse_str_to_tree("[7,[6,[5,[7,0]]]]", 0).0;
        let action = process(pre_explosion);
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_somewhere_in_the_middle() {
        let pre_explosion = parse_str_to_tree("[[6,[5,[4,[3,2]]]],1]", 0).0;
        let post_explosion = parse_str_to_tree("[[6,[5,[7,0]]],3]", 0).0;
        let action = process(pre_explosion);
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_should_only_affect_left_not_right() {
        let pre_explosion = parse_str_to_tree("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", 0).0;
        let post_explosion = parse_str_to_tree("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 0).0;
        let action = process(pre_explosion);
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_random_example() {
        let pre_explosion = parse_str_to_tree("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 0).0;
        let post_explosion = parse_str_to_tree("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", 0).0;
        let action = process(pre_explosion);
        assert_eq!(post_explosion, action);
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