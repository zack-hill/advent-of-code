use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> u64 {
    let values = parse_file();
    let preamble = 25;
    return find_invalid_number(&values, preamble);
}

pub fn solve_puzzle_2() -> u64 {
    let values = parse_file();
    let preamble = 25;
    let target_number = find_invalid_number(&values, preamble);
    let mut longest_chain = Vec::<u64>::new();
    for i in 0..values.len() {
        let mut sum = 0;
        for j in i..values.len() {
            sum += values[j];
            if sum > target_number {
                break;
            }
            // Check if we have found a range of numbers that sums to our target
            if sum == target_number {
                let chain_length = j + 1 - i;
                if chain_length > longest_chain.len() {
                    // Store the values that produced the longest chain
                    longest_chain.clear();
                    for k in i..=j {
                        longest_chain.push(values[k]);
                    }
                } else {
                    break;
                }
            }
        }
    }
    return longest_chain.iter().min().unwrap() + longest_chain.iter().max().unwrap();
}

fn find_invalid_number(values: &Vec<u64>, preamble: usize) -> u64 {
    for i in preamble..values.len() {
        if !validate(&values, i, preamble) {
            return values[i];
        }
    }
    return 0;
}

fn validate(values: &Vec<u64>, index: usize, preamble: usize) -> bool {
    let start = index - preamble;
    for i in start..index {
        for j in start..index {
            if i == j {
                continue;
            }
            if values[i] + values[j] == values[index] {
                return true;
            }
        }
    }
    return false;
}

fn parse_file() -> Vec<u64> {
    let file = File::open("src/day_09.txt").unwrap();
    let reader = BufReader::new(file);
    let values: Vec<u64> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    return values;
}
