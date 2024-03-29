use crate::solver::AoCSolver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

pub struct Solver {
    fields: Vec<Field>,
    our_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Solver {
    pub fn create() -> Self {
        let file_data = parse_file();
        return Solver {
            fields: file_data.fields,
            our_ticket: file_data.our_ticket,
            nearby_tickets: file_data.nearby_tickets,
        };
    }

    fn validate_ticket(&self, ticket: &Vec<u32>) -> bool {
        for &value in ticket {
            if !self.validate_value(value) {
                return false;
            }
        }
        return true;
    }
    
    fn validate_value(&self, value: u32) -> bool {
        return self.fields.iter().any(|f| f.validate_value(value));
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let ticket_scanning_error_rate = self.
            nearby_tickets
            .iter()
            .flatten()
            .filter(|&v| !self.validate_value(*v))
            .sum::<u32>();
        return ticket_scanning_error_rate.to_string();
    }

    fn solve_part_2(&self) -> String {
        // Filter out invalid nearby tickets
        let valid_tickets: Vec<&Vec<u32>> = self.
            nearby_tickets
            .iter()
            .filter(|t| self.validate_ticket(t))
            .collect();

        // Find a list of potential indices for each field
        let mut valid_field_map = HashMap::<&str, Vec<usize>>::new();
        for field in self.fields.iter() {
            let valid_positions = valid_field_map
                .entry(&field.name)
                .or_insert(Vec::<usize>::new());
            for i in 0..self.fields.len() {
                let values: Vec<u32> = valid_tickets.iter().map(|t| t[i]).collect();
                if field.validate_values(&values) {
                    valid_positions.push(i)
                }
            }
        }

        // Sort the fields in ascending order based on the number of possible positions for that field
        let mut unsolved_fields: Vec<&str> = self.fields.iter().map(|f| f.name.as_str()).collect();
        unsolved_fields.sort_by(|a, b| valid_field_map[a].len().cmp(&valid_field_map[b].len()));

        // Find position for each field using process of elimination. Each field solved reduces the number
        // of possible positions for another field to one. This cascades until all fields are solved.
        let mut field_position_map = HashMap::<&str, usize>::new();
        for field in unsolved_fields.iter() {
            // Check the possible indices for the field
            for possible_index in valid_field_map[field].iter() {
                // If the current possible index is not already claimed, take it
                if field_position_map.values().all(|v| v != possible_index) {
                    field_position_map.insert(field, *possible_index);
                }
            }
        }

        // Calculate the product of each of the departure fields for our ticket
        let result: u64 = field_position_map
            .keys()
            .filter(|k| k.starts_with("departure"))
            .map(|k| field_position_map[k])
            .map(|i| self.our_ticket[i as usize] as u64)
            .product();

        return result.to_string();
    }
}



struct Field {
    name: String,
    ranges: Vec<Range<u32>>,
}

impl Field {
    fn validate_value(&self, value: u32) -> bool {
        return self.ranges.iter().any(|r| r.contains(&value));
    }
    
    fn validate_values(&self, values: &Vec<u32>) -> bool {
        return values.iter().all(|&v| self.validate_value(v));
    }
}

struct FileData {
    fields: Vec<Field>,
    our_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn parse_file() -> FileData {
    let file = File::open("src/2020/day_16.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fields = Vec::<Field>::new();
    let mut our_ticket = Vec::<u32>::new();
    let mut nearby_tickets = Vec::<Vec<u32>>::new();

    let mut read_fields = true;
    let mut read_our_ticket = false;
    let mut read_nearby_tickets = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if read_fields {
            if line.trim() == "" {
                read_fields = false;
                continue;
            }
            let field = read_field(&line);
            fields.push(field);
        }
        if line == "your ticket:" {
            read_our_ticket = true;
            continue;
        }
        if read_our_ticket {
            if line.trim() == "" {
                read_our_ticket = false;
                continue;
            }
            our_ticket = read_ticket(&line);
        }
        if line == "nearby tickets:" {
            read_nearby_tickets = true;
            continue;
        }
        if read_nearby_tickets {
            nearby_tickets.push(read_ticket(&line))
        }
    }

    let file_data = FileData {
        fields,
        our_ticket,
        nearby_tickets,
    };
    return file_data;
}

fn read_field(line: &str) -> Field {
    let sections: Vec<&str> = line.split(":").collect();
    let name = sections[0].to_owned();
    let line = sections[1];
    let mut ranges = Vec::<Range<u32>>::new();
    for text in line.split("or") {
        let sections: Vec<&str> = text.trim().split("-").collect();
        let min: u32 = sections[0].parse().unwrap();
        let max: u32 = sections[1].parse().unwrap();
        // One is added here because the range specified in the file is inclusive
        // on the upper end while the range data type is not.
        ranges.push(min..max + 1);
    }
    Field { name, ranges }
}

fn read_ticket(line: &str) -> Vec<u32> {
    line.split(",").map(|x| x.parse().unwrap()).collect()
}
