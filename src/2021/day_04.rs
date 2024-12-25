use crate::solver::AoCSolver;
use std::{fs::File, io::BufRead, io::BufReader};

type Board = [[u8; 5]; 5];

pub struct Solver {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Solver {
    pub fn create() -> Self {
        let (numbers, boards) = parse_input();
        return Solver { numbers, boards };
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut numbers: Vec<u8> = self.numbers.iter().rev().cloned().collect();
        let mut called_numbers = vec![];

        while let Some(number) = numbers.pop() {
            called_numbers.push(number);
            for board in self.boards.iter() {
                if has_board_won(&board, &called_numbers) {
                    let board_score = score_board(&board, &called_numbers);
                    let final_score = board_score * number as u32;
                    return final_score.to_string();
                }
            }
        }

        panic!("No solution found");
    }

    fn solve_part_2(&self) -> String {
        let mut numbers: Vec<u8> = self.numbers.iter().rev().cloned().collect();
        let mut called_numbers = vec![];
        let mut boards = self.boards.clone();

        while let Some(number) = numbers.pop() {
            called_numbers.push(number);
            if boards.len() == 1 {
                // We found the last board to win, now keep playing until it has won
                let last_board_to_win = boards.get(0).unwrap();
                if has_board_won(last_board_to_win, &called_numbers) {
                    let board_score = score_board(last_board_to_win, &called_numbers);
                    let final_score = board_score * number as u32;
                    // println!("{:?}", number);
                    return final_score.to_string();
                }
            } else {
                // Remove any board that wins
                let _ = boards.extract_if(|board| has_board_won(&board, &called_numbers));
            }
        }

        panic!("No solution found");
    }
}

fn has_board_won(board: &Board, numbers: &Vec<u8>) -> bool {
    // Check rows
    for y in 0..5 {
        if board[y].iter().all(|value| numbers.contains(value)) {
            return true;
        }
    }

    // Check cols
    for x in 0..5 {
        if (0..5)
            .map(|y| &board[y][x])
            .all(|value| numbers.contains(value))
        {
            return true;
        }
    }

    return false;
}

fn score_board(board: &Board, numbers: &Vec<u8>) -> u32 {
    board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|v| !numbers.contains(v))
        .map(|v| *v as u32)
        .sum()
}

fn parse_input() -> (Vec<u8>, Vec<Board>) {
    let file = File::open("src/2021/day_04.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];
    let mut boards = vec![];
    let mut current_board = [[0; 5]; 5];
    let mut current_board_row: usize = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            numbers = line.split(',').map(|s| s.parse::<u8>().unwrap()).collect();
        } else if line != "" {
            let values: Vec<u8> = line
                .split(' ')
                .filter(|x| *x != "")
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            for (j, value) in values.iter().enumerate() {
                current_board[current_board_row][j] = *value;
            }

            current_board_row += 1;
            if current_board_row == 5 {
                // println!("{:?}", current_board);
                boards.push(current_board);
                current_board_row = 0;
            }
        }
    }
    return (numbers, boards);
}
