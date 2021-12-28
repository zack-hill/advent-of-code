use itertools::Itertools;

use crate::solver::AoCSolver;
use std::{collections::VecDeque, fs::File, io::BufRead, io::BufReader};

struct LineScore {
    corrupt_char_score: u64,
    incomplete_char_score: u64,
}

pub struct Solver {
    lines: Vec<String>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            lines: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        self.lines
            .iter()
            .map(|l| score_line(l).corrupt_char_score)
            .sum::<u64>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let sorted_scores: Vec<u64> = self
            .lines
            .iter()
            .map(|l| score_line(l).incomplete_char_score)
            .filter(|&s| s != 0)
            .sorted()
            .collect();
        let middle_score = sorted_scores[(sorted_scores.len() - 1) / 2];
        return middle_score.to_string();
    }
}

fn score_line(line: &str) -> LineScore {
    let mut corrupt_char_score: u64 = 0;
    let mut incomplete_char_score: u64 = 0;
    let mut groups = VecDeque::new();
    let mut is_valid = true;

    for char in line.chars() {
        if is_open_char(char) {
            groups.push_back(char);
        } else {
            let current_group = groups.pop_back();
            match current_group {
                Some(current_group) => {
                    let expected = get_matching_char(current_group);
                    if char != expected {
                        // println!("Expected {} but found {}", expected, char);
                        corrupt_char_score += score_corrupt_char(char);
                        is_valid = false;
                        break;
                    }
                }
                None => {
                    println!("Found {}, but no group was open", char);
                }
            }
        }
    }
    if is_valid {
        // println!("Scoring incomplete line");
        incomplete_char_score = groups.iter().rev().fold(0, |acc, &c| {
            // let matching_char = get_matching_char(c);
            // let score = score_incomplete_char(matching_char);
            // println!("{} * 5 + {} ({})", acc, matching_char, score);
            acc * 5 + score_incomplete_char(get_matching_char(c))
        });
        // println!("Incomplete line {:?} scored {}", groups, incomplete_char_score);
    }
    LineScore {
        corrupt_char_score,
        incomplete_char_score,
    }
}

fn is_open_char(char: char) -> bool {
    match char {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn get_matching_char(char: char) -> char {
    match char {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        c => panic!("Unexpected char '{}'", c),
    }
}

fn score_corrupt_char(char: char) -> u64 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("Unexpected char '{}'", c),
    }
}

fn score_incomplete_char(char: char) -> u64 {
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        c => panic!("Unexpected char '{}'", c),
    }
}

pub fn parse_input() -> Vec<String> {
    let file = File::open("src/2021/day_10.txt").unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.unwrap()).collect();
}
