use std::{fs::File, io::BufRead, io::BufReader};
use itertools::Itertools;

pub fn solve_part_1(input: &Vec<u32>) -> usize {
    let count = input
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    return count;
}

pub fn solve_part_2(input: &Vec<u32>) -> usize {
    let count = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    return count;
}

pub fn parse_input() -> Vec<u32> {
    let file = File::open("src/year_2021/day_01.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
}
