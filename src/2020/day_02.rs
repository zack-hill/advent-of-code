extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::solver::AoCSolver;

pub struct Solver {
    password_db_entries: Vec<(PasswordPolicy, Password)>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            password_db_entries: parse_input(),
        }
    }
}

pub struct PasswordPolicy {
    n1: usize,
    n2: usize,
    character: char,
}

type Password = String;

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        self.password_db_entries
            .iter()
            .filter(|(password_policy, password)| {
                validate_password_using_count(password_policy, password)
            })
            .count()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.password_db_entries
            .iter()
            .filter(|(policy, password)| validate_password_using_position(policy, password))
            .count()
            .to_string()
    }
}

fn validate_password_using_count(policy: &PasswordPolicy, password: &str) -> bool {
    let count = password.chars().filter(|&x| x == policy.character).count();
    return count >= policy.n1 && count <= policy.n2;
}

fn validate_password_using_position(policy: &PasswordPolicy, password: &str) -> bool {
    let characters: Vec<char> = password.chars().collect();
    // Use XOR to check if EXACTLY ONE of the positions contains the desired letter
    return (characters[policy.n1 - 1] == policy.character)
        ^ (characters[policy.n2 - 1] == policy.character);
}

pub fn parse_input() -> Vec<(PasswordPolicy, Password)> {
    let file = File::open("src/2020/day_02.txt").unwrap();
    let reader = BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)-(\d+) (\S): (\S+)").unwrap();

    let data = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let captures = line_regex.captures(&line).unwrap();

            let n1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let n2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let character = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
            let password = captures.get(4).unwrap().as_str().to_string();
            let password_policy = PasswordPolicy { n1, n2, character };
            return (password_policy, password);
        })
        .collect();
    return data;
}
