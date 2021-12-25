use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_part_1(ages: &Vec<u32>) -> usize {
    let mut ages = ages.clone();

    let new_fish_age = 8;
    let recovery_time = 6;
    let days = 80;

    for _ in 0..days {
        let mut new_fish = vec![];
        for age in ages.iter_mut() {
            if *age == 0 {
                new_fish.push(new_fish_age);
                *age = recovery_time;
            } else {
                *age -= 1;
            }
        }
        ages.append(&mut new_fish);
    }

    return ages.len();
}

pub fn solve_part_2(ages: &Vec<u32>) -> usize {
    let mut ages = ages.clone();

    let new_fish_age = 8;
    let recovery_time = 6;
    let days = 256;

    for _ in 0..days {
        let mut new_fish = vec![];
        for age in ages.iter_mut() {
            if *age == 0 {
                new_fish.push(new_fish_age);
                *age = recovery_time;
            } else {
                *age -= 1;
            }
        }
        ages.append(&mut new_fish);
    }

    return ages.len();
}

pub fn parse_input() -> Vec<u32> {
    let file = File::open("src/year_2021/day_06.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let ages = line.split(',').map(|age| age.parse().unwrap()).collect();
    return ages;
}
