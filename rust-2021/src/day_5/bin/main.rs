use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./rust-2021/inputs/day_5_input")?;
    let buf_reader = BufReader::new(file);

    let mut contents = vec![];
    for line in  buf_reader.lines() {
        contents.push(line?);
    }

    let parsed_input = parse_contents(contents)?;

    calculate_part_1(parsed_input.clone());
    calculate_part_2(parsed_input.clone());

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Debug, Clone)]
struct LineSegment {
    start: Coordinate,
    end: Coordinate
}

impl LineSegment {

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn points(&self) -> HashSet<Coordinate> {
        let (start,  end, axis) = if self.is_horizontal() {
            (min(self.start.y, self.end.y), max(self.start.y, self.end.y), self.start.x)
        } else if self.is_vertical() {
            (min(self.start.x, self.end.x), max(self.start.x, self.end.x), self.start.y)
        } else {
            return self.diag_points();
        };

        let mut results = HashSet::new();
        for i in start..=end {
            results.insert(if self.is_horizontal() {
                Coordinate { x: axis, y: i }
            } else {
                Coordinate { x: i, y: axis }
            });
        }

        results
    }

    fn left_most(&self) -> Coordinate {
        if self.start.x <= self.end.x {
            self.start.clone()
        } else {
            self.end.clone()
        }
    }

    fn right_most(&self) -> Coordinate {
        if self.start.x > self.end.x {
            self.start.clone()
        } else {
            self.end.clone()
        }
    }

    fn gradient(&self) -> i32 {
        (self.end.y - self.start.y) / (self.end.x - self.start.x)
    }

    fn diag_points(&self) -> HashSet<Coordinate> {
        let positive = self.gradient() > 0;

        let mut results = HashSet::new();

        let mut left = self.left_most();
        let mut right = self.right_most();

        while left != right {
            if positive {
                results.insert(left.clone());
                left.x += 1;
                left.y += 1;
            } else {
                results.insert(left.clone());
                left.x += 1;
                left.y -= 1;
            }
        }

        results.insert(right.clone());

        results
    }

    fn intersect(&self, other: &LineSegment) -> HashSet<Coordinate> {
        self.points().intersection(&other.points())
            .map(|i| i.clone())
            .collect()
    }

}

fn parse_coordinate(line: &String) -> Result<Coordinate, Box<dyn Error>> {
    let line_split = line.split(',')
        .map(|e| String::from(e))
        .collect::<Vec<String>>();
    let x = line_split[0].parse::<i32>()?;
    let y = line_split[1].parse::<i32>()?;
    Ok(Coordinate { x, y})
}

fn parse_contents(contents: Vec<String>) -> Result<Vec<LineSegment>, Box<dyn Error>> {
    let mut results = vec![];

    for line in contents {
        let line_split = line.split_whitespace()
            .map(|e| String::from(e))
            .collect::<Vec<String>>();
        let start = parse_coordinate(&line_split[0])?;
        let end = parse_coordinate(&line_split[2])?;

        results.push(LineSegment { start, end })
    }

    Ok(results)
}

fn calculate_part_1(segments: Vec<LineSegment>) {
    let mut set = HashMap::new();

    for segment in segments {
        if !(segment.is_vertical() || segment.is_horizontal()) {
            continue;
        }

        for point in segment.points() {
            let entry = set.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    println!("{:?}", set.iter().filter(|(_coord, count)| count >= &&2).count());
}

fn calculate_part_2(segments: Vec<LineSegment>) {
    let mut set = HashMap::new();

    for segment in segments {
        for point in segment.points() {
            let entry = set.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    println!("{:?}", set.iter().filter(|(_coord, count)| count >= &&2).count());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_positive_diag() {
        let line_1 = LineSegment {
            start: Coordinate { x: 1, y: 1},
            end: Coordinate { x: 3, y: 3 }
        };

        let expected = HashSet::from([
                                         Coordinate::new(1, 1),
                                         Coordinate::new(2, 2),
                                         Coordinate::new(3, 3)
                                     ]);
        assert_eq!(expected, line_1.points());
    }

    #[test]
    fn test_negative_diag() {
        let line_1 = LineSegment {
            start: Coordinate { x: 9, y: 7},
            end: Coordinate { x: 7, y: 9 }
        };

        let expected = HashSet::from([
                                         Coordinate::new(9, 7),
                                         Coordinate::new(8, 8),
                                         Coordinate::new(7, 9)
                                     ]);
        assert_eq!(expected, line_1.points());
    }

    #[test]
    fn test_not_touching() {
        let line_1 = LineSegment {
            start: Coordinate { x: 0, y: 9},
            end: Coordinate { x: 5, y: 9 }
        };

        let line_2 = LineSegment {
            start: Coordinate { x: 8, y: 0},
            end: Coordinate { x: 0, y: 8 }
        };

        let expected = HashSet::new();

        assert_eq!(expected, line_1.intersect(&line_2));
    }

    #[test]
    fn test_horizontal_intersect() {
        let line_1 = LineSegment {
            start: Coordinate { x: 0, y: 9},
            end: Coordinate { x: 5, y: 9 }
        };

        let line_2 = LineSegment {
            start: Coordinate { x: 0, y: 9},
            end: Coordinate { x: 2, y: 9 }
        };

        let expected = HashSet::from([
            Coordinate::new(0, 9),
            Coordinate::new(1, 9),
            Coordinate::new(2, 9)
        ]);

        assert_eq!(expected, line_1.intersect(&line_2));
    }

    #[test]
    fn test_vertical_intersect() {
        let line_1 = LineSegment {
            start: Coordinate { x: 2, y: 1},
            end: Coordinate { x: 2, y: 9 }
        };

        let line_2 = LineSegment {
            start: Coordinate { x: 2, y: 5},
            end: Coordinate { x: 2, y: 7 }
        };

        let expected = HashSet::from([
            Coordinate::new(2, 5),
            Coordinate::new(2, 6),
            Coordinate::new(2, 7)
        ]);

        assert_eq!(expected, line_1.intersect(&line_2));
    }

}