extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    return count_valid_passports(validate_passport_fields);
}

pub fn solve_puzzle_2() -> usize {
    return count_valid_passports(validate_passport_fields_and_values);
}

fn count_valid_passports(validate: fn(&HashMap<String, String>) -> bool) -> usize {
    let passports = parse_file("src/day_04.txt");
    return passports.iter().filter(|p| validate(p)).count();
}

fn validate_passport_fields(passport: &HashMap<String, String>) -> bool {
    return ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|x| passport.contains_key(*x));
}

fn validate_passport_fields_and_values(passport: &HashMap<String, String>) -> bool {
    let validation_rules = [
        ("byr", validate_birth_year as fn(&str) -> bool),
        ("iyr", validate_issue_year),
        ("eyr", validate_expiration_year),
        ("hgt", validate_height),
        ("hcl", validate_hair_color),
        ("ecl", validate_eye_color),
        ("pid", validate_passport_id),
    ];
    for (field, validate) in validation_rules.iter() {
        let is_valid = match passport.get(*field) {
            Some(value) => validate(value),
            None => false,
        };
        if !is_valid {
            return false;
        }
    }
    return true;
}

fn validate_birth_year(value: &str) -> bool {
    return validate_number(value, 1920, 2002);
}

fn validate_issue_year(value: &str) -> bool {
    return validate_number(value, 2010, 2020);
}

fn validate_expiration_year(value: &str) -> bool {
    return validate_number(value, 2020, 2030);
}

fn validate_height(value: &str) -> bool {
    if value.len() <= 2 {
        return false;
    }

    let (height, unit) = value.split_at(value.len() - 2);
    return match unit {
        "cm" => validate_number(height, 150, 193),
        "in" => validate_number(height, 59, 76),
        _ => false
    };
}

fn validate_hair_color(value: &str) -> bool {
    return Regex::new(r"#[a-f0-9]{6}").unwrap().is_match(value);
}

fn validate_eye_color(value: &str) -> bool {
    return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value);
}

fn validate_passport_id(value: &str) -> bool {
    return value.len() == 9 && value.parse::<usize>().is_ok();
}

fn validate_number(value: &str, min: usize, max: usize) -> bool {
    return match value.parse::<usize>() {
        Ok(expiration_year) => (expiration_year >= min && expiration_year <= max),
        Err(_) => false,
    };
}

fn parse_file(path: &str) -> Vec<HashMap<String, String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut passports = Vec::<HashMap<String, String>>::new();

    let mut current_passport = HashMap::new();

    for line in reader.lines() {
        let line: &str = &line.unwrap();

        if line.len() == 0 {
            passports.push(current_passport);
            current_passport = HashMap::new();
            continue;
        }

        for kvp in line.split(" ") {
            let elements: Vec<&str> = kvp.split(":").collect();
            let (key, value) = (elements[0], elements[1]);
            current_passport.insert(key.to_owned(), value.to_owned());
        }
    }

    passports.push(current_passport);

    return passports;
}
