extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    let (rules, messages) = parse_file();

    let mut expanded_rules = HashMap::<u8, String>::new();
    expand_rule(0, 0, &rules, &mut expanded_rules);

    let rule = Regex::new(&format!("^{}$", &expanded_rules[&0])).unwrap();
    return messages.iter().filter(|m| rule.is_match(m)).count();
}
pub fn solve_puzzle_2() -> usize {
    let (mut rules, messages) = parse_file();

    // Replace rules, introducing recursive rules
    rules.insert(8, "42 | 42 8".to_owned());
    rules.insert(11, "42 31 | 42 11 31".to_owned());

    let mut expanded_rules = HashMap::<u8, String>::new();

    expand_rule(0, 0, &rules, &mut expanded_rules);

    let rule = Regex::new(&format!("^{}$", &expanded_rules[&0])).unwrap();
    return messages.iter().filter(|m| rule.is_match(m)).count();
}

fn parse_file() -> (HashMap<u8, String>, Vec<String>) {
    let file = File::open("src/day_19.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules = HashMap::<u8, String>::new();
    let mut messages = Vec::<String>::new();

    let mut is_rule_line = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            is_rule_line = false;
            continue;
        }
        if is_rule_line {
            let index = line.find(":").unwrap();
            let num: u8 = line[..index].parse().unwrap();
            rules.insert(num, line[index + 1..].trim().to_owned());
        } else {
            messages.push(line.to_owned());
        }
    }

    return (rules, messages);
}

fn expand_rule(
    num: u8,
    recursion_depth: u32,
    rules: &HashMap<u8, String>,
    expanded_rules: &mut HashMap<u8, String>,
) {
    if expanded_rules.contains_key(&num) {
        return;
    }
    let rule = &rules[&num];
    if rule.starts_with('"') {
        let rule = rule.trim_matches('"').to_owned();
        expanded_rules.insert(num, rule);
    } else {
        let is_option = rule.contains('|');
        let mut expanded_rule = String::new();
        if is_option {
            expanded_rule.push('(');
        }
        for component in rule.split(' ') {
            if component == "|" {
                expanded_rule.push('|');
            } else {
                let child_rule_num: u8 = component.parse().unwrap();
                // Track recursive depth
                let depth = if child_rule_num == num {
                    recursion_depth + 1
                } else {
                    0
                };
                // Cut off recursive rules at 10 levels deep
                if depth < 10 {
                    expand_rule(child_rule_num, depth, rules, expanded_rules);
                    expanded_rule.push_str(&expanded_rules[&child_rule_num]);
                }
            }
        }
        if is_option {
            expanded_rule.push(')');
        }
        expanded_rules.insert(num, expanded_rule);
    }
}
