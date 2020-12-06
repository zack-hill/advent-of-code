extern crate regex;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    return read_groups_from_file()
        .iter()
        .map(|g| {
            g.iter().fold(HashSet::<String>::new(), |acc, x| {
                acc.union(&x).cloned().collect()
            })
        }) // fold each group into a single set that contains the union of all sets in the group
        .map(|g| g.len())
        .sum();
}

pub fn solve_puzzle_2() -> usize {
    let all_letters: HashSet<String> = ('a'..='z').map(|c| String::from(c)).collect();
    return read_groups_from_file()
        .iter()
        .map(|g| {
            g.iter().fold(all_letters.clone(), |acc, x| {
                acc.intersection(&x).cloned().collect()
            })
        }) // fold each group into a single set that contains the intersection of all sets in the group
        .map(|g| g.len())
        .sum();
}

fn read_groups_from_file() -> Vec<Vec<HashSet<String>>> {
    let file = File::open("src/day_06.txt").unwrap();
    let reader = BufReader::new(file);

    let mut groups = Vec::<Vec<HashSet<String>>>::new();

    let mut current_group = Vec::<HashSet<String>>::new();

    for line in reader.lines() {
        let line: &str = &line.unwrap();

        if line.len() == 0 {
            groups.push(current_group);
            current_group = Vec::new();
            continue;
        }

        current_group.push(
            line.chars()
                .map(|c| String::from(c))
                .collect::<HashSet<_>>(),
        )
    }

    groups.push(current_group);

    return groups;
}
