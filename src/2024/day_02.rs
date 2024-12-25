use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::solver::AoCSolver;

pub struct Solver {
    reports: Vec<Vec<usize>>,
}

impl Solver {
    pub fn create() -> Self {
        let reports = parse_input();
        Solver { reports }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        return self
            .reports
            .iter()
            .filter(|r| is_report_safe(r))
            .count()
            .to_string();
    }

    fn solve_part_2(&self) -> String {
        return self
            .reports
            .iter()
            .filter(|r| {
                r.iter()
                    .copied()
                    .combinations(r.len() - 1)
                    .any(|c| is_report_safe(&c))
            })
            .count()
            .to_string();
    }
}

fn is_report_safe(report: &[usize]) -> bool {
    let is_ascending = report.windows(2).all(|w| w[0] < w[1]);
    let is_descending = report.windows(2).all(|w| w[0] > w[1]);
    let are_level_changes_safe = report.windows(2).all(|w| is_level_change_safe(w[0], w[1]));
    return (is_ascending || is_descending) && are_level_changes_safe;
}

fn is_level_change_safe(left: usize, right: usize) -> bool {
    let diff = left.abs_diff(right);
    return diff >= 1 && diff <= 3;
}

fn parse_input() -> Vec<Vec<usize>> {
    let file = File::open("src/2024/day_02.txt").unwrap();
    let reader = BufReader::new(file);
    let reports = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect()
        })
        .collect();
    return reports;
}
