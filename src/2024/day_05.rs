use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

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
            .map(|x| {
                get_middle_page(&sort_pages_using_ordering_rule_elimination(
                    x,
                    &self.ordering_rules,
                ))
            })
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

#[cfg(test)]
fn sort_pages_using_swap(
    pages: &PageCollection,
    ordering_rules: &Vec<OrderingRule>,
) -> PageCollection {
    // Filter down ordering rules to just those which apply to our set of pages
    let relevant_ordering_rules: Vec<&OrderingRule> = ordering_rules
        .iter()
        .filter(|(left, right)| pages.contains(&left) && pages.contains(&right))
        .collect();

    let mut pages = pages.clone();

    // Finds the indexes of the pages of an ordering rule and swaps them if they are out of order
    // If no swaps are made then the collection is in order and is returned
    loop {
        let mut sorted = true;
        for (left, right) in relevant_ordering_rules.iter() {
            let left_index = pages.iter().position(|x| x == left).unwrap();
            let right_index = pages.iter().position(|x| x == right).unwrap();
            if left_index > right_index {
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

fn sort_pages_using_ordering_rule_elimination(
    pages: &PageCollection,
    ordering_rules: &Vec<OrderingRule>,
) -> PageCollection {
    // Filter down ordering rules to just those which apply to our set of pages
    let mut relevant_ordering_rules: Vec<&OrderingRule> = ordering_rules
        .iter()
        .filter(|(left, right)| pages.contains(&left) && pages.contains(&right))
        .collect();

    let mut sorted = PageCollection::new();
    let mut to_sort: HashSet<&usize> = pages.iter().collect();

    for _ in pages {
        // Find the page which doesn't have any rules which put after another page
        let page = **to_sort
            .iter()
            .filter(|p| {
                !relevant_ordering_rules
                    .iter()
                    .any(|(_, right)| **p == right)
            })
            .exactly_one()
            .unwrap();
        sorted.push(page);
        to_sort.remove(&page);

        // Remove ordering rules specific to the page we just sorted
        let _: Vec<&OrderingRule> = relevant_ordering_rules
            .extract_if(|(left, _)| left == &page)
            .collect();
    }

    sorted
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_pages_using_swap_works() {
        let pages = vec![3, 1, 2, 5];
        let ordering_rules = vec![(1, 2), (2, 3), (3, 4), (4, 5), (3, 5)];
        let ordered_pages = sort_pages_using_swap(&pages, &ordering_rules);
        assert_eq!(ordered_pages, vec![1, 2, 3, 5]);
    }

    #[test]
    fn sort_pages_using_ordering_rule_elimination_works() {
        let pages = vec![3, 1, 2, 5];
        let ordering_rules = vec![(1, 2), (2, 3), (3, 4), (4, 5), (3, 5)];
        let ordered_pages = sort_pages_using_ordering_rule_elimination(&pages, &ordering_rules);
        assert_eq!(ordered_pages, vec![1, 2, 3, 5]);
    }
}
