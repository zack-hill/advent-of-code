use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct PasswordPolicy {
    n1: usize,
    n2: usize,
    character: char,
}

pub fn solve_part_1(input: &Vec<(PasswordPolicy, String)>) -> usize {
    let valid_count = input
        .iter()
        .filter(|(password_policy, password)| {
            validate_password_using_count(password_policy, password)
        })
        .count();
    return valid_count;
}

pub fn solve_part_2(input: &Vec<(PasswordPolicy, String)>) -> usize {
    let valid_count = input
        .iter()
        .filter(|(password_policy, password)| {
            validate_password_using_position(password_policy, password)
        })
        .count();
    return valid_count;
}

fn validate_password_using_count(password_policy: &PasswordPolicy, password: &str) -> bool {
    let count = password
        .chars()
        .filter(|&x| x == password_policy.character)
        .count();
    return count >= password_policy.n1 && count <= password_policy.n2;
}

fn validate_password_using_position(password_policy: &PasswordPolicy, password: &str) -> bool {
    let characters: Vec<char> = password.chars().collect();
    // Use XOR to check if EXACTLY ONE of the positions contains the desired letter
    return (characters[password_policy.n1 - 1] == password_policy.character)
        ^ (characters[password_policy.n2 - 1] == password_policy.character);
}

pub fn parse_input() -> Vec<(PasswordPolicy, String)> {
    let file = File::open("src/day_02.txt").unwrap();
    let reader = BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)-(\d+) (\S): (\S+)").unwrap();

    let mut data = Vec::new();

    for line in reader.lines() {
        let line: &str = &line.unwrap();

        let captures = line_regex.captures(line).unwrap();

        let n1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let n2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let character = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.get(4).unwrap().as_str().to_string();
        let password_policy = PasswordPolicy { n1, n2, character };

        data.push((password_policy, password));
    }

    return data;
}
