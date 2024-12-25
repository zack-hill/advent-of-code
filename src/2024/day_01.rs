use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;

use crate::solver::AoCSolver;

struct InputData {
    left_numbers: Vec<usize>,
    right_numbers: Vec<usize>,
}

pub struct Solver {
    input_data: InputData,
}

impl Solver {
    pub fn create() -> Self {
        let input_data = parse_input();
        Solver { input_data }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let left_sorted = self.input_data.left_numbers.iter().sorted();
        let right_sorted = self.input_data.right_numbers.iter().sorted();

        let sum: usize = iter::zip(left_sorted, right_sorted)
            .map(|(l, r)| l.abs_diff(*r))
            .sum();

        return sum.to_string();
    }

    fn solve_part_2(&self) -> String {
        let frequencies =
            self.input_data
                .right_numbers
                .iter()
                .copied()
                .fold(HashMap::new(), |mut map, val| {
                    map.entry(val).and_modify(|frq| *frq += 1).or_insert(1usize);
                    map
                });

        let sum: usize = self
            .input_data
            .left_numbers
            .iter()
            .map(|x| x * frequencies.get(x).unwrap_or(&0))
            .sum();

        return sum.to_string();
    }
}

fn parse_input() -> InputData {
    let file = File::open("src/2024/day_01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in reader.lines() {
        let (left, right) = parse_line(line.unwrap().as_str());
        left_numbers.push(left);
        right_numbers.push(right);
    }

    return InputData {
        left_numbers,
        right_numbers,
    };
}

fn parse_line(line: &str) -> (usize, usize) {
    let spl: Vec<&str> = line.split_whitespace().collect();
    return (spl[0].parse().unwrap(), spl[1].parse().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let line = "123  456";
        let actual = parse_line(&line);
        let expected = (123, 456);
        assert_eq!(actual, expected);
    }
}
