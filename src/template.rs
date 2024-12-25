use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::solver::AoCSolver;

pub struct Solver {
    lines: Vec<String>,
}

impl Solver {
    pub fn create() -> Self {
        let lines = parse_input();
        Solver { lines }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        return "".to_string();
    }

    fn solve_part_2(&self) -> String {
        return "".to_string();
    }
}

fn parse_input() -> Vec<String> {
    let file = File::open("src/2020/day_01.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect();
    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
