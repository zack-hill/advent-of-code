use crate::solver::AoCSolver;
use std::{fs::File, io::BufRead, io::BufReader};

pub struct Solver {
    positions: Vec<u32>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            positions: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let max = self.positions.iter().max().unwrap();

        let best_cost = (0..*max)
            .map(|p| calculate_fuel_cost(&self.positions, p))
            .min()
            .unwrap();

        return best_cost.to_string();
    }

    fn solve_part_2(&self) -> String {
        let max = self.positions.iter().max().unwrap();

        let best_cost = (0..*max)
            .map(|p| calculate_fuel_cost_using_triangle_number(&self.positions, p))
            .min()
            .unwrap();

        return best_cost.to_string();
    }
}

fn calculate_fuel_cost(positions: &Vec<u32>, position: u32) -> u32 {
    positions
        .iter()
        .fold(0, |acc, p| acc + abs_diff(position, *p))
}

fn calculate_fuel_cost_using_triangle_number(positions: &Vec<u32>, position: u32) -> u32 {
    positions.iter().fold(0, |acc, p| {
        acc + calculate_triangle_number(abs_diff(position, *p))
    })
}

fn calculate_triangle_number(number: u32) -> u32 {
    (number.pow(2) + number) / 2
}

fn abs_diff(position: u32, destination: u32) -> u32 {
    ((destination as i32) - (position as i32)).abs() as u32
}

pub fn parse_input() -> Vec<u32> {
    let file = File::open("src/2021/day_07.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let positions = line.split(',').map(|pos| pos.parse().unwrap()).collect();

    return positions;
}
