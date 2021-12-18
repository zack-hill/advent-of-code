use itertools::GroupBy;
use itertools::Itertools;
use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_part_1(input: &Vec<Vec<char>>) -> u32 {
    let length = input[0].len();
    let gamma_rate = (0..length)
        .map(|i| {
            input
                .iter()
                .map(|x| x[i])
                .sorted()
                .group_by(|&x| x)
                .into_iter()
                .map(|(key, group)| (key, group.count()))
                .max_by_key(|&(_, count)| count)
                .unwrap()
                .0
        })
        .collect::<String>();
    println!("{:?}", gamma_rate);
    let gamma_rate = u16::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = !gamma_rate;
    // TODO: This is awful and should be rewritten.
    let epsilon_rate = epsilon_rate
        .to_be_bytes()
        .iter()
        .map(|b| format!("{:b}", b))
        .join("")
        .chars()
        .skip(4)
        .take(length)
        .collect::<String>();
    println!("{:?}", epsilon_rate);
    let epsilon_rate = u16::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("{:?}", gamma_rate);
    println!("{:?}", epsilon_rate);
    return gamma_rate as u32 * epsilon_rate as u32;
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> usize {
    return 0;
}

pub fn parse_input() -> Vec<Vec<char>> {
    let file = File::open("src/year_2021/day_03.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
}
