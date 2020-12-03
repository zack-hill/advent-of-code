use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> u32 {
    return solve(validate_password_using_count);
}

pub fn solve_puzzle_2() -> u32 {
    return solve(validate_password_using_position);
}

fn solve(validate_password: fn(usize, usize, char, &str) -> bool) -> u32 {
    let file = File::open("src/day_02.txt").unwrap();
    let reader = BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)-(\d+) (\S): (\S+)").unwrap();

    let mut valid_count = 0;

    for line in reader.lines() {
        let line: &str = &line.unwrap();

        let captures = line_regex.captures(line).unwrap();

        let n1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let n2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.get(4).unwrap().as_str();

        if validate_password(n1, n2, letter, password) {
            valid_count += 1;
        }
    }
    return valid_count;
}

fn validate_password_using_count(n1: usize, n2: usize, letter: char, password: &str) -> bool {
    let count = password.chars().filter(|&x| x == letter).count();
    return count >= n1 && count <= n2;
}

fn validate_password_using_position(n1: usize, n2: usize, letter: char, password: &str) -> bool {
    let characters: Vec<char> = password.chars().collect();
    // Use XOR to check if EXACTLY ONE of the positions contains the desired letter
    return (characters[n1 - 1] == letter) ^ (characters[n2 - 1] == letter);
}
