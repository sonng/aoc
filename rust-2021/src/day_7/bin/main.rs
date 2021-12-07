use std::cmp::min;
use std::error::Error;
use aoc_utils::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_file("./rust-2021/inputs/day_7_input")?;
    let parsed_input = parse(contents)?;

    println!("{:?}", calculate_part_1(&parsed_input));
    println!("{:?}", calculate_part_2(&parsed_input));

    Ok(())
}

type Input = Vec<i64>;
type Output = i64;

fn parse(contents: Vec<String>) -> Result<Input, Box<dyn Error>> {
    Ok(contents[0].split(',')
        .map(|e| e.parse::<i64>().unwrap())
        .collect())
}

fn calculate_part_1(input: &Input) -> Output {
    let mut spots = input.clone();
    spots.sort();

    let median = if spots.len() % 2 == 0 {
        spots[spots.len() / 2]
    } else {
        (spots[spots.len() / 2] + spots[spots.len() / 2 - 1]) / 2
    };

    let mut results = 0;

    for i in spots {
        results += (i - median).abs();
    }

    results
}

fn calc(mean: i64, input: &Input) -> Output {
    let mut results = 0;
    for i in input {
        let distance = (mean - i).abs();
        let fuel = (distance * (distance + 1)) / 2;
        results += fuel;
    }

    results
}

fn calculate_part_2(input: &Input) -> Output {
    let total = input.iter().sum::<i64>();
    let mean = total as f64 / (input.len() as f64);

    let mean_ceil = mean.ceil() as i64;
    let ceil_mean = calc(mean_ceil, &input);

    let mean_floor = mean.floor() as i64;
    let floor_mean = calc(mean_floor, &input);

    min(ceil_mean, floor_mean)
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use aoc_utils::read_file;

    use super::*;

    #[test]
    fn test_calculate_1() -> Result<(), Box<dyn Error>> {
        let contents = read_file("./inputs/day_7_test_input")?;
        let input = parse(contents)?;

        assert_eq!(37, calculate_part_1(&input));
        Ok(())
    }

    #[test]
    fn test_calculate_2() -> Result<(), Box<dyn Error>> {
        let contents = read_file("./inputs/day_7_test_input")?;
        let input = parse(contents)?;

        assert_eq!(168, calculate_part_2(&input));
        Ok(())
    }
}