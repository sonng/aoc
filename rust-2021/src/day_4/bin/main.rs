use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./rust-2021/inputs/day_4_input")?;
    let buf_reader = BufReader::new(file);

    let mut contents = vec![];
    for line in buf_reader.lines() {
        contents.push(line?);
    }

    let parsed_input = parse(contents)?;

    calculate_part_1(parsed_input.clone());
    calculate_part_2(parsed_input.clone());

    Ok(())
}

type Board = Vec<Vec<String>>;

#[derive(Debug, Clone)]
struct Puzzle {
    numbers: Vec<String>,
    boards: Vec<Board>
}

fn parse(input: Vec<String>) -> Result<Puzzle, Box<dyn Error>> {
    let mut i_iter = input.into_iter();
    let numbers = i_iter.next().map(|nums| {
        nums.split(',')
            .map(|e| String::from(e))
            .collect::<Vec<String>>()
    }).unwrap();

    let mut boards = vec![];
    let mut board = vec![];
    for line in i_iter {
        if line.len() == 0 {
            if board.len() > 0 {
                boards.push(board);
            }
            board = vec![];
        } else {
            let nums = line.split_whitespace()
                .map(|e| String::from(e))
                .collect::<Vec<String>>();
            board.push(nums);
        }
    }

    if board.len() > 0 {
        boards.push(board);
    }

    Ok(Puzzle { numbers, boards })
}

fn check_board(board: &Board) -> bool {
    for row in board {
        if row.iter()
            .map(|col| col.starts_with("-"))
            .fold(true, |acc, item| acc && item) {
            return true;
        }
    }

    let row_size = board.len();
    let col_size = board[0].len();

    for col in 0..col_size {
        let mut marked = true;
        for row in 0..row_size {
            marked = marked && board[row][col].starts_with("-")
        }

        if marked {
            return true;
        }
    }

    false
}

fn mark_board(board: &mut Board, number: &String) {
    let row_size = board.len();
    let col_size = board[0].len();

    for col in 0..col_size {
        for row in 0..row_size {
            if board[row][col].eq(number) {
                board[row][col].insert(0, '-');
            }
        }
    }
}

fn calculate_part_1(mut puzzle: Puzzle) {
    'outer: for num in puzzle.numbers {
        for mut board in &mut puzzle.boards {
            mark_board(&mut board, &num);

            if check_board(board) {
                let sum: i32 = board.iter()
                    .flat_map(|i| i)
                    .filter(|s| !s.starts_with('-'))
                    .map(|s| s.parse::<i32>().unwrap())
                    .sum();

                println!("{:?}", sum * num.parse::<i32>().unwrap());
                break 'outer;
            }
        }
    }
}

fn calculate_part_2(mut puzzle: Puzzle) {
    let mut boards = puzzle.boards;
    'outer: for num in drawn_numbers {
        let mut to_be_removed = vec![];
        let num_of_boards = boards.len();

        for i in 0..boards.len() {
            let mut board = &mut boards[i];
            mark_board(&mut board, num);

            if check_board(board) {
                if num_of_boards > 1 {
                    to_be_removed.push(i);
                } else {
                    let sum = board.iter()
                        .flat_map(|i| i)
                        .filter(|s| !s.starts_with('-'))
                        .map(|s| s.parse::<i32>().unwrap())
                        .sum::<i32>();

                    println!("{:?}", sum * num.parse::<i32>().unwrap());
                    break 'outer;
                }
            }
        }

        for idx in to_be_removed.into_iter().rev() {
            boards.remove(idx);
        }
    }
}