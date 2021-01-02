#![feature(iterator_fold_self)]
#![feature(linked_list_remove)]

extern crate nom;

mod day_23;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("Puzzle 1: {}", day_23::solve_puzzle_1());
    println!("Puzzle 2: {}", day_23::solve_puzzle_2());

    println!(
        "Time Elapsed: {} ms",
        start.elapsed().as_secs_f64() * 1000f64
    );
}
