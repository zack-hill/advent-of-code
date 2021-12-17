#![feature(iterator_fold_self)]

extern crate nom;

mod year_2021;
use year_2021::day_02;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = day_02::parse_input();

    println!("Part 1: {}", day_02::solve_part_1(&input));
    println!("Part 2: {}", day_02::solve_part_2(&input));

    println!(
        "Time Elapsed: {} ms",
        start.elapsed().as_secs_f64() * 1000f64
    );
}
