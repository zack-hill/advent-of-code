use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::solver::AoCSolver;

type OrderingRule = (usize, usize);
type PageCollection = Vec<usize>;

pub struct Solver {
    ordering_rules: Vec<OrderingRule>,
    updates: Vec<PageCollection>,
}

impl Solver {
    pub fn create() -> Self {
        let (ordering_rules, updates) = parse_input();
        Solver {
            ordering_rules,
            updates,
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let sum: usize = self
            .updates
            .iter()
            .filter(|x| check_is_sorted(x, &self.ordering_rules))
            .map(|x| get_middle_page(x))
            .sum();
        return sum.to_string();
    }

    fn solve_part_2(&self) -> String {
        let sum: usize = self
            .updates
            .iter()
            .filter(|x| !check_is_sorted(x, &self.ordering_rules))
            .map(|x| get_middle_page(&sort_pages(x, &self.ordering_rules)))
            .sum();
        return sum.to_string();
    }
}

fn check_is_sorted(pages: &PageCollection, ordering_rules: &Vec<OrderingRule>) -> bool {
    let filtered = ordering_rules
        .iter()
        .filter(|(left, right)| pages.contains(&left) && pages.contains(&right));
    for (left, right) in filtered {
        let left_index = pages.iter().position(|x| x == left).unwrap();
        let right_index = pages.iter().position(|x| x == right).unwrap();
        if left_index > right_index {
            return false;
        }
    }
    return true;
}

fn get_middle_page(pages: &PageCollection) -> usize {
    pages[pages.len() / 2]
}

fn sort_pages(pages: &PageCollection, ordering_rules: &Vec<OrderingRule>) -> PageCollection {
    let filtered: Vec<&OrderingRule> = ordering_rules
        .iter()
        .filter(|(left, right)| pages.contains(&left) && pages.contains(&right))
        .collect();
    let mut pages = pages.clone();
    // println!("{:?} {:?}", pages, filtered);

    loop {
        let mut sorted = true;
        for (left, right) in filtered.iter() {
            let left_index = pages.iter().position(|x| x == left).unwrap();
            let right_index = pages.iter().position(|x| x == right).unwrap();
            if left_index > right_index {
                // println!(
                //     "{:?} Swapping {:?} and {:?} | {:?}",
                //     pages, pages[left_index], pages[right_index], filtered
                // );
                pages.swap(left_index, right_index);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }

    pages
}

enum ParseMode {
    OrderingRule,
    UpdateSet,
}

fn parse_input() -> (Vec<OrderingRule>, Vec<PageCollection>) {
    let file = File::open("src/2024/day_05.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ordering_rules = Vec::new();
    let mut updates = Vec::new();

    let mut parse_mode = ParseMode::OrderingRule;

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            parse_mode = ParseMode::UpdateSet;
            continue;
        }

        match parse_mode {
            ParseMode::OrderingRule => {
                ordering_rules.push(parse_ordering_rule(&line));
            }
            ParseMode::UpdateSet => {
                updates.push(parse_update(&line));
            }
        };
    }

    (ordering_rules, updates)
}

fn parse_ordering_rule(line: &str) -> OrderingRule {
    let (left, right) = line.split_once('|').unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

fn parse_update(line: &str) -> PageCollection {
    line.split(',').map(|x| x.parse().unwrap()).collect()
}
