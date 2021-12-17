use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> i64 {
    let (departure_time, bus_lines) = parse_file();
    for time in departure_time.. {
        for (_, bus_line) in bus_lines.iter() {
            if time % bus_line == 0 {
                return (time - departure_time) * bus_line;
            }
        }
    }
    return 0;
}

pub fn solve_puzzle_2() -> i64 {
    let (_, bus_lines) = parse_file();

    // Use Chinese Remainder Theorem
    let prod: i64 = bus_lines.iter().map(|(_, b)| b).product();
    let result = bus_lines
        .iter()
        .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
        .sum::<i64>()
        .rem_euclid(prod);

    return result;
}

fn inv_mod(x: i64, p: i64) -> i64 {
    // p must be prime
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

fn parse_file() -> (i64, Vec<(i64, i64)>) {
    let file = File::open("src/day_13.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut departure_time = String::new();
    reader.read_line(&mut departure_time).unwrap();
    let departure_time: i64 = departure_time.trim_end().parse().unwrap();

    let mut bus_lines = String::new();
    reader.read_line(&mut bus_lines).unwrap();
    let bus_lines: Vec<(i64, i64)> = bus_lines
        .trim_end()
        .split(",")
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(i, x)| (i as i64, x.parse().unwrap()))
        .collect();

    return (departure_time, bus_lines);
}
