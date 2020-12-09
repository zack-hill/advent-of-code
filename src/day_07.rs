extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    let (parent_map, _) = parse_file();

    let mut upstream_bags = HashSet::<String>::new();
    let mut queue = Vec::<String>::new();
    queue.push("shiny gold".to_string());

    while let Some(bag) = queue.pop() {
        if let Some(parents) = parent_map.get(&bag) {
            for (_, parent) in parents {
                upstream_bags.insert(parent.clone());
                queue.push(parent.clone());
            }
        }
    }

    return upstream_bags.len();
}

pub fn solve_puzzle_2() -> usize {
    let (_, child_map) = parse_file();
    return count_children(&"shiny gold".to_string(), &child_map);
}

fn count_children(bag: &String, child_map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    return match child_map.get(bag) {
        Some(children) => children
            .iter()
            .map(|(qty, child)| {
                let child_count = count_children(&child, child_map);
                // One is added to the child count for the bag itself
                qty * (1 + child_count) //
            })
            .sum(),
        None => 0,
    };
}

fn parse_file() -> (
    HashMap<String, Vec<(usize, String)>>,
    HashMap<String, Vec<(usize, String)>>,
) {
    let file = File::open("src/day_07.txt").unwrap();
    let reader = BufReader::new(file);

    let parent_bag_regex = Regex::new(r"(.+) bags ").unwrap();
    let child_bags_regex = Regex::new(r"(\d+) ([^.,]+) bag").unwrap();

    let mut parent_map = HashMap::<String, Vec<(usize, String)>>::new();
    let mut child_map = HashMap::<String, Vec<(usize, String)>>::new();

    for line in reader.lines() {
        let line: &str = &line.unwrap();

        let parent = parent_bag_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        for cap in child_bags_regex.captures_iter(line) {
            let qty: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let child = cap.get(2).unwrap().as_str();

            insert(child.to_owned(), qty, parent.to_owned(), &mut parent_map);
            insert(parent.to_owned(), qty, child.to_owned(), &mut child_map);
        }
    }
    return (parent_map, child_map);
}

fn insert(
    key: String,
    qty: usize,
    value: String,
    nodes: &mut HashMap<String, Vec<(usize, String)>>,
) {
    match nodes.get_mut(&key) {
        Some(x) => x.push((qty, value)),
        None => {
            let mut collection = Vec::new();
            collection.push((qty, value));
            nodes.insert(key, collection);
        }
    }
}
