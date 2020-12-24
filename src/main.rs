#[macro_use]
extern crate nom;

mod day_18;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("Puzzle 1: {}", day_18::solve_puzzle_1());
    println!("Puzzle 2: {}", day_18::solve_puzzle_2());

    println!(
        "Time Elapsed: {} ms",
        start.elapsed().as_secs_f64() * 1000f64
    );
}
