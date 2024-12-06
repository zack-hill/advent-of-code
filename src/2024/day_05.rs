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

fn sort_pages2(pages: &PageCollection, ordering_rules: &Vec<OrderingRule>) -> PageCollection {
    let mut filtered: Vec<&OrderingRule> = ordering_rules
        .iter()
        .filter(|(left, right)| pages.contains(&left) && pages.contains(&right))
        .collect();
    let mut sorted = PageCollection::new();
    let mut to_sort: HashSet<&usize> = pages.iter().collect();

    for page in pages {
        // println!("pages: {:?}", pages);
        // println!("filtered {:?}", filtered);
        let o = **to_sort
            .iter()
            .filter(|p| !filtered.iter().any(|(left, right)| **p == right))
            .exactly_one()
            .unwrap();
        sorted.push(o);
        to_sort.remove(&o);

        let _: Vec<&OrderingRule> = filtered.extract_if(|(left, right)| left == &o).collect();
        continue;

        let a: HashSet<&usize> = filtered
            .iter()
            .filter(|(left, right)| !sorted.contains(left))
            .map(|(left, right)| right)
            .unique()
            .collect();
        let diff = to_sort.difference(&a);
        // println!("a: {:?}", a);
        // println!("to_sort: {:?}", to_sort);
        // println!("diff: {:?}", diff);
        let b = *diff.exactly_one().unwrap();
        // println!("b: {:?}", b);
        // let b = to_sort
        //     .iter()
        //     .filter(|x| a.d(value))
        //     .filter(|x| to_sort.contains(x))
        //     .exactly_one()
        //     .unwrap();
        sorted.push(*b);
        to_sort.remove(b);
        // let _: Vec<&OrderingRule> = filtered.extract_if(|(left, right)| left == b).collect();

        // .sorted()
        // .group_by(|x| x)
        // .into_iter()
        // .filter(|(page, group)| group.count() == 0)
        // .exactly_one()
        // .unwrap()
        // .0;
        // sorted.push(**a);
        // filtered.extract_if(|(left, right)| left == *a);
        // println!("sorted: {:?}", sorted);
        // println!("");
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
