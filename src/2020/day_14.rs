extern crate regex;

use bit_field::BitField;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> u64 {
    let instructions = parse_file();

    let mut mask = String::new();
    let mut memory = HashMap::<u64, u64>::new();

    for instruction in instructions {
        match instruction {
            Instruction::SetMask(new_mask) => {
                mask = new_mask;
            }
            Instruction::SetValue(address, value) => {
                let mut current_value = *memory.get(&address).unwrap_or(&0u64);
                for (bit, mask_char) in mask.chars().rev().enumerate() {
                    let new_bit_value = match mask_char {
                        '1' => true,
                        '0' => false,
                        'X' => value.get_bit(bit),
                        _ => panic!("Invalid mask char"),
                    };
                    current_value.set_bit(bit, new_bit_value);
                }
                memory.insert(address, current_value);
            }
        };
    }
    return memory.values().sum();
}

pub fn solve_puzzle_2() -> u64 {
    let instructions = parse_file();

    let mut mask = String::new();
    let mut memory = HashMap::<u64, u64>::new();

    for instruction in instructions {
        match instruction {
            Instruction::SetMask(new_mask) => {
                mask = new_mask;
            }
            Instruction::SetValue(address, value) => {
                let mut base_address = address;
                let mut floating_bits = Vec::<usize>::new();

                for (bit, mask_char) in mask.chars().rev().enumerate() {
                    match mask_char {
                        '1' => {
                            base_address.set_bit(bit, true);
                        }
                        'X' => {
                            floating_bits.push(bit);
                        }
                        _ => {}
                    };
                }
                if floating_bits.is_empty() {
                    memory.insert(base_address, value);
                } else {
                    for bits in floating_bits
                        .iter()
                        .map(|&b| vec![(b, false), (b, true)])
                        .flatten()
                        .combinations(floating_bits.len())
                    {
                        let mut address = base_address;
                        for (bit, value) in bits {
                            address.set_bit(bit, value);
                        }
                        memory.insert(address, value);
                    }
                }
            }
        };
    }
    return memory.values().sum();
}

enum Instruction {
    SetMask(String),
    SetValue(u64, u64),
}

fn parse_file() -> Vec<Instruction> {
    let file = File::open("src/day_14.txt").unwrap();
    let reader = BufReader::new(file);
    let mut instructions = Vec::<Instruction>::new();

    let mem_regex = Regex::new(r"\[(\d+)\] = (\d+)").unwrap();
    let mask_regex = Regex::new(r"mask = (.+)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let instruction = if line.starts_with("mem") {
            let caps = mem_regex.captures(&line).unwrap();
            let address = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            Instruction::SetValue(address, value)
        } else {
            let caps = mask_regex.captures(&line).unwrap();
            let mask = caps.get(1).unwrap().as_str().to_owned();
            Instruction::SetMask(mask)
        };
        instructions.push(instruction);
    }
    return instructions;
}
