use std::cmp::max;
use std::error::Error;
use aoc_utils::{Puzzle, run_all};
use crate::NodeKind::{Branch, Value};

fn main() -> Result<(), Box<dyn Error>> {
    run_all("./rust-2021/inputs/day_18.in", Box::new(Day18))
}

struct Day18;

type Input = Vec<ChildNode>;
type Output = i64;

#[derive(Debug, PartialEq, Clone)]
enum NodeKind {
    Value(i64),
    Branch
}

type ChildNode = Option<Box<Node>>;

#[derive(Debug, PartialEq, Clone)]
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
        let mut peek = cur;
        for i in 0..s.len() {
            let peek_str = &s[peek..peek+1];
            if peek_str == "[" || peek_str == "," || peek_str == "]" {
                break;
            }
            peek += 1;
        }
        let value = (&s[cur..peek]).parse::<i64>().unwrap();
        (Node::new_child_value(value), peek)
    }
}

fn calc_magnitude(node: ChildNode) -> i64 {
    match node {
        None => 0,
        Some(n) => {
            match n.kind {
                Value(i) => i,
                Branch => {
                    let left = 3 * calc_magnitude(n.left);
                    let right = 2 * calc_magnitude(n.right);

                    left + right
                }
            }
        }
    }
}

fn add(left: ChildNode, right: ChildNode) -> ChildNode {
    Node::new_node(left, right)
}

fn add_and_reduce(left: ChildNode, right: ChildNode) -> ChildNode {
    let mut initial = add(left, right);

    loop {
        let try_process = process(initial);

        initial = try_process.0;

        if try_process.1 == false {
            break;
        }
    }

    initial
}

fn process(node: ChildNode) -> (ChildNode, bool) {
    let after_explode = try_explode(node, 0);

    if let Some(_) = after_explode.1 {
        return (after_explode.0, true);
    } else {
        try_split(after_explode.0)
    }
}

fn try_split(node: ChildNode) -> (ChildNode, bool) {
    let mut splitted = false;
    (node.and_then(|n| {
        match n.kind {
            Branch => {
                let try_left = try_split(n.left);

                if try_left.1 {
                    splitted = true;
                    return Node::new_node(try_left.0, n.right)
                }

                let try_right = try_split(n.right);
                splitted = try_right.1;
                Node::new_node(try_left.0, try_right.0)
            },
            Value(i) if i > 9 => {
                let divide = i as f64 / 2.0;
                let left = divide.floor() as i64;
                let right = divide.ceil() as i64;

                splitted = true;

                Node::new_node(Node::new_child_value(left), Node::new_child_value(right))
            },
            Value(_) => Some(n)
        }
    }), splitted)
}

type Explosion = Option<(Option<i64>, Option<i64>)>;

fn try_explode(node: ChildNode, level: usize) -> (ChildNode, Explosion) {
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
                    let process_left = try_explode(n.left, level + 1);
                    let left = process_left.0;
                    let left_explosion = process_left.1;

                    if let Some((ex_left, ex_right)) = left_explosion {
                        let right = find_and_add_most_left(n.right, ex_right);
                        explosion = Some((ex_left, None));

                        return Node::new_node(left, right);
                    }

                    let process_right = try_explode(n.right, level + 1);
                    let right = process_right.0;
                    let right_explosion = process_right.1;

                    if let Some((ex_left, ex_right)) = right_explosion {
                        let left = find_and_add_most_right(left, ex_left);
                        explosion = Some((None, ex_right));

                        return Node::new_node(left, right);
                    }

                    return Node::new_node(left, right);
                }
            }
        }
    }), explosion)
}

fn find_and_add_most_right(node: ChildNode, explosion: Option<i64>) -> ChildNode {
    match explosion {
        None => node,
        Some(explode_value) => {
            node.and_then(|n| {
                match n.kind {
                    Value(i) => Node::new_child_value(i + explode_value),
                    Branch => {
                        let right = find_and_add_most_right(n.right, Some(explode_value));
                        Node::new_node(n.left, right)
                    }
                }
            })
        }
    }
}

fn find_and_add_most_left(node: ChildNode, explosion: Option<i64>) -> ChildNode {
    match explosion {
        Some(explode_value) => {
            node.and_then(|n| {
                match n.kind {
                    Value(i) => Node::new_child_value(i + explode_value),
                    Branch => {
                        let left = find_and_add_most_left(n.left, Some(explode_value));
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
        Ok(contents.iter()
            .map(|s| parse_str_to_tree(&s[..], 0).0)
               .collect())
    }

    fn calculate_part_1(&self, input: &Input) -> Output {
        let mut input = input.clone();
        let mut initial = input[0].take();

        for i in 1..input.len() {
            let right = input[i].take();
            initial = add_and_reduce(initial, right);
        }

        calc_magnitude(initial)
    }

    fn calculate_part_2(&self, input: &Input) -> Output {
        let mut amount = 0;
        for i in 0..input.len() {
            for j in i+1..input.len() {
                let left = input[i].clone();
                let right = input[j].clone();

                let reduced = add_and_reduce(left.clone(), right.clone());
                let left_amount = calc_magnitude(reduced);

                let reduced = add_and_reduce(right.clone(), left.clone());
                let right_amount = calc_magnitude(reduced);
                amount = max(amount, max(left_amount, right_amount));
            }
        }

        amount
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
        let action = process(pre_explosion).0;
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_right_most_explosion() {
        let pre_explosion = parse_str_to_tree("[7,[6,[5,[4,[3,2]]]]]", 0).0;
        let post_explosion = parse_str_to_tree("[7,[6,[5,[7,0]]]]", 0).0;
        let action = process(pre_explosion).0;
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_somewhere_in_the_middle() {
        let pre_explosion = parse_str_to_tree("[[6,[5,[4,[3,2]]]],1]", 0).0;
        let post_explosion = parse_str_to_tree("[[6,[5,[7,0]]],3]", 0).0;
        let action = process(pre_explosion).0;
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_should_only_affect_left_not_right() {
        let pre_explosion = parse_str_to_tree("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", 0).0;
        let post_explosion = parse_str_to_tree("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 0).0;
        let action = process(pre_explosion).0;
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_explosion_random_example() {
        let pre_explosion = parse_str_to_tree("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 0).0;
        let post_explosion = parse_str_to_tree("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", 0).0;
        let action = process(pre_explosion).0;
        assert_eq!(post_explosion, action);
    }

    #[test]
    fn test_star_2() {
        let left = parse_str_to_tree("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]", 0).0;
        let right = parse_str_to_tree("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]", 0).0;
        let sum = add_and_reduce(left, right);
        assert_eq!(3993, calc_magnitude(sum));
    }

    #[test]
    fn test_split_simple() {
        let pre_split = parse_str_to_tree("[10,1]", 0).0;
        let post_split = parse_str_to_tree("[[5,5],1]", 0).0;
        let action = process(pre_split).0;
        assert_eq!(post_split, action);
    }

    #[test]
    fn test_split_one_at_a_time() {
        let pre_split = parse_str_to_tree("[10,10]", 0).0;

        let first_split = parse_str_to_tree("[[5,5],10]", 0).0;
        let action = process(pre_split).0;
        assert_eq!(first_split, action);

        let second_split = parse_str_to_tree("[[5,5],[5,5]]", 0).0;
        let action = process(action).0;
        assert_eq!(second_split, action);
    }

    #[test]
    fn test_sequence() {
        let pre_loop = parse_str_to_tree("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", 0).0;

        let step_1 = parse_str_to_tree("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", 0).0;
        let mut action = process(pre_loop).0;
        assert_eq!(step_1, action);

        let step_2 = parse_str_to_tree("[[[[0,7],4],[15,[0,13]]],[1,1]]", 0).0;
        action = process(action).0;
        assert_eq!(step_2, action);

        let step_3 = parse_str_to_tree("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", 0).0;
        action = process(action).0;
        assert_eq!(step_3, action);

        let step_4 = parse_str_to_tree("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", 0).0;
        action = process(action).0;
        assert_eq!(step_4, action);

        let step_5 = parse_str_to_tree("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 0).0;
        action = process(action).0;
        assert_eq!(step_5, action);
    }

    #[test]
    fn test_magnitude_example_1() {
        let tree = parse_str_to_tree("[[1,2],[[3,4],5]]", 0).0;
        assert_eq!(143, calc_magnitude(tree));
    }

    #[test]
    fn test_magnitude_example_2() {
        let tree = parse_str_to_tree("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 0).0;
        assert_eq!(1384, calc_magnitude(tree));
    }

    #[test]
    fn test_magnitude_example_3() {
        let tree = parse_str_to_tree("[[[[1,1],[2,2]],[3,3]],[4,4]]", 0).0;
        assert_eq!(445, calc_magnitude(tree));
    }

    #[test]
    fn test_magnitude_example_4() {
        let tree = parse_str_to_tree("[[[[3,0],[5,3]],[4,4]],[5,5]]", 0).0;
        assert_eq!(791, calc_magnitude(tree));
    }

    #[test]
    fn test_magnitude_example_5() {
        let tree = parse_str_to_tree("[[[[5,0],[7,4]],[5,5]],[6,6]]", 0).0;
        assert_eq!(1137, calc_magnitude(tree));
    }

    #[test]
    fn test_magnitude_example_6() {
        let tree = parse_str_to_tree("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 0).0;
        assert_eq!(3488, calc_magnitude(tree));
    }

    #[test]
    fn test_final_sum_1() {
        let left = parse_str_to_tree("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", 0).0;
        let right = parse_str_to_tree("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", 0).0;

        let sum = parse_str_to_tree("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_2() {
        let left = parse_str_to_tree("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]", 0).0;
        let right = parse_str_to_tree("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]", 0).0;

        let sum = parse_str_to_tree("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_3() {
        let left = parse_str_to_tree("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]", 0).0;
        let right = parse_str_to_tree("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]", 0).0;

        let sum = parse_str_to_tree("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_4() {
        let left = parse_str_to_tree("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]", 0).0;
        let right = parse_str_to_tree("[7,[5,[[3,8],[1,4]]]]", 0).0;

        let sum = parse_str_to_tree("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_5() {
        let left = parse_str_to_tree("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]", 0).0;
        let right = parse_str_to_tree("[[2,[2,2]],[8,[8,1]]]", 0).0;

        let sum = parse_str_to_tree("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_6() {
        let left = parse_str_to_tree("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]", 0).0;
        let right = parse_str_to_tree("[2,9]", 0).0;

        let sum = parse_str_to_tree("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_7() {
        let left = parse_str_to_tree("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]", 0).0;
        let right = parse_str_to_tree("[1,[[[9,3],9],[[9,0],[0,7]]]]", 0).0;

        let sum = parse_str_to_tree("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_8() {
        let left = parse_str_to_tree("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]", 0).0;
        let right = parse_str_to_tree("[[[5,[7,4]],7],1]", 0).0;

        let sum = parse_str_to_tree("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_final_sum_9() {
        let left = parse_str_to_tree("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]", 0).0;
        let right = parse_str_to_tree("[[[[4,2],2],6],[8,7]]", 0).0;

        let sum = parse_str_to_tree("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 0).0;

        assert_eq!(sum, add_and_reduce(left, right));
    }

    #[test]
    fn test_calculate_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(4140, run_part_one("./inputs/day_18_test.in", Box::new(Day18))?);
        Ok(())
    }

    #[test]
    fn test_calculate_two() -> Result<(), Box<dyn Error>> {
        assert_eq!(3993, run_part_two("./inputs/day_18_test.in", Box::new(Day18))?);
        Ok(())
    }
}