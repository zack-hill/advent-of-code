use crate::solver::AoCSolver;
use itertools::Itertools;
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

pub struct Solver {
    entries: Vec<Entry>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            entries: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        self.entries
            .iter()
            .map(|e| {
                e.output_values
                    .iter()
                    .filter(|ov| ov.len() == 2 || ov.len() == 3 || ov.len() == 4 || ov.len() == 7)
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.entries
            .iter()
            .map(|entry| {
                let key = find_key(entry).unwrap();
                let output_value = calculate_output_value(entry, &key);
                // println!("{:?}: {}", entry.output_values, output_value);
                return output_value;
            })
            .sum::<u32>()
            .to_string()
    }
}

pub struct Entry {
    signal_patterns: Vec<String>,
    output_values: Vec<String>,
}

type Key = HashMap<char, char>;

fn find_key(entry: &Entry) -> Result<Key, &str> {
    for permutation in ('a'..='g').permutations(7) {
        let mut potential_replacement_map = HashMap::new();
        for (i, character) in ('a'..='g').enumerate() {
            potential_replacement_map.insert(character, permutation[i]);
        }

        let map_is_valid = entry
            .signal_patterns
            .iter()
            .all(|sp| segments_to_digit(&substitute_chars(sp, &potential_replacement_map)).is_ok());

        if map_is_valid {
            return Ok(potential_replacement_map);
        }
    }
    return Err("Unable to find valid key");
}

fn calculate_output_value(entry: &Entry, key: &Key) -> u32 {
    let digits: Vec<u8> = entry
        .output_values
        .iter()
        .map(|ov| segments_to_digit(&substitute_chars(ov, &key)).unwrap())
        .collect();
    return digits_to_number(&digits);
}

fn substitute_chars(string: &str, key: &Key) -> String {
    string.chars().map(|c| key[&c]).collect()
}

fn segments_to_digit(segments: &str) -> Result<u8, &str> {
    let sorted_chars: String = segments.chars().sorted().collect();
    return match sorted_chars.as_str() {
        "abcefg" => Ok(0),
        "cf" => Ok(1),
        "acdeg" => Ok(2),
        "acdfg" => Ok(3),
        "bcdf" => Ok(4),
        "abdfg" => Ok(5),
        "abdefg" => Ok(6),
        "acf" => Ok(7),
        "abcdefg" => Ok(8),
        "abcdfg" => Ok(9),
        _ => Err("No digit matches the given segments"),
    };
}

fn digits_to_number(digits: &Vec<u8>) -> u32 {
    digits
        .iter()
        .enumerate()
        .map(|(i, d)| *d as u32 * 10u32.pow((digits.len() - 1) as u32 - i as u32))
        .sum()
}

pub fn parse_input() -> Vec<Entry> {
    let file = File::open("src/2021/day_08.txt").unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let (signal_patterns, output_values) = line.split('|').collect_tuple().unwrap();
        let signal_patterns = signal_patterns
            .trim()
            .split(' ')
            .map(|s| s.to_owned())
            .collect();
        let output_values = output_values
            .trim()
            .split(' ')
            .map(|s| s.to_owned())
            .collect();

        let entry = Entry {
            signal_patterns,
            output_values,
        };
        entries.push(entry);
    }

    return entries;
}
