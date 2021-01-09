extern crate regex;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Group = Vec<HashSet<String>>;

pub fn solve_part_1(groups: &Vec<Group>) -> usize {
    return groups
        .iter()
        .map(|g| {
            g.iter().fold(HashSet::<String>::new(), |acc, x| {
                acc.union(&x).cloned().collect()
            })
        }) // fold each group into a single set that contains the union of all sets in the group
        .map(|g| g.len())
        .sum();
}

pub fn solve_part_2(groups: &Vec<Group>) -> usize {
    let all_letters: HashSet<String> = ('a'..='z').map(|c| String::from(c)).collect();
    return groups
        .iter()
        .map(|g| {
            g.iter().fold(all_letters.clone(), |acc, x| {
                acc.intersection(&x).cloned().collect()
            })
        }) // fold each group into a single set that contains the intersection of all sets in the group
        .map(|g| g.len())
        .sum();
}

pub fn parse_input() -> Vec<Group> {
    let file = File::open("src/day_06.txt").unwrap();
    let reader = BufReader::new(file);

    let mut groups = Vec::<Group>::new();

    let mut current_group = Group::new();

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
