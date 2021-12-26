use crate::solver::AoCSolver;
use std;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Solver {
    instructions: Vec<(String, isize)>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            instructions: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut visited_lines = HashSet::<usize>::new();
        let mut line = 0;
        let mut acc = 0;
        loop {
            let (operation, argument) = &self.instructions[line];
            perform_operation(operation, *argument, &mut line, &mut acc);
            if visited_lines.contains(&line) {
                // We have hit a loop.
                return acc.to_string();
            }
            visited_lines.insert(line);
        }
    }

    fn solve_part_2(&self) -> String {
        let mut visited_lines = HashSet::<usize>::new();
        let mut line = 0;
        let mut acc = 0;
        let mut swapped_line = 0;
        loop {
            let (operation, argument) = &self.instructions[line];
            let mut operation: &str = operation;

            if line == swapped_line {
                // Swap the operation
                operation = match operation {
                    "nop" => "jmp",
                    "jmp" => "nop",
                    _ => operation,
                };
            }

            perform_operation(operation, *argument, &mut line, &mut acc);

            if visited_lines.contains(&line) {
                // An infinite loop was found, reset and start over with a new swapped line.
                acc = 0;
                line = 0;
                visited_lines.clear();
                // This should ideally increment to the next swappable line to skip
                // unnecessary calculations.
                swapped_line += 1;
                continue;
            } else if line == self.instructions.len() {
                // The last line has executed meaning that the correct line was swapped.
                return acc.to_string();
            }

            visited_lines.insert(line);
        }
    }
}

fn perform_operation(operation: &str, argument: isize, line: &mut usize, acc: &mut isize) {
    match operation {
        "acc" => {
            *acc += argument;
            *line += 1;
        }
        "jmp" => {
            *line = if argument < 0 {
                *line - (-argument) as usize
            } else {
                *line + argument as usize
            }
        }
        "nop" => *line += 1,
        _ => {}
    };
}

pub fn parse_input() -> Vec<(String, isize)> {
    let file = File::open("src/2020/day_08.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let operation = line[0..3].to_owned();
            let argument: isize = line[4..].parse().unwrap();
            (operation, argument)
        })
        .collect();
}
