use crate::solver::AoCSolver;
use std::{fs::File, io::BufRead, io::BufReader};

pub struct Solver {
    timers: [u64; 9],
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            timers: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        simulate(&mut self.timers.clone(), 80).to_string()
    }

    fn solve_part_2(&self) -> String {
        simulate(&mut self.timers.clone(), 256).to_string()
    }
}

fn simulate(timers: &mut [u64; 9], days: u32) -> u64 {
    for _ in 0..days {
        let reproducing_count = timers[0];
        for timer_day in 1..=8 {
            timers[timer_day - 1] = timers[timer_day];
        }
        timers[6] += reproducing_count;
        timers[8] = reproducing_count;
    }

    return timers.iter().sum();
}

pub fn parse_input() -> [u64; 9] {
    let file = File::open("src/2021/day_06.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut timer_groups = [0u64; 9];
    let timers: Vec<usize> = line
        .split(',')
        .map(|timer| timer.parse().unwrap())
        .collect();

    for timer in timers {
        timer_groups[timer] += 1;
    }

    return timer_groups;
}
