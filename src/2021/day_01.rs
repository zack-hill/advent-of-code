use crate::solver::AoCSolver;
use itertools::Itertools;
use std::{fs::File, io::BufRead, io::BufReader};

pub struct Solver {
    input: Vec<u32>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            input: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let count = self
            .input
            .iter()
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count();
        return count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let count = self
            .input
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count();
        return count.to_string();
    }
}

pub fn parse_input() -> Vec<u32> {
    let file = File::open("src/2021/day_01.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
}
