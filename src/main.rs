#![feature(extract_if)]

extern crate nom;

mod solver;

use solver::AoCSolver;
use std::time::Instant;

#[path = "util/mod.rs"]
mod util;
#[path = "2020/mod.rs"]
mod y_2020;
#[path = "2021/mod.rs"]
mod y_2021;
#[path = "2024/mod.rs"]
mod y_2024;

fn main() {
    solve(2024, 8);
    // for year in 2021..=2021 {
    //     for day in 1..=8 {
    //         solve(year, day);
    //     }
    // }
}

fn solve(year: u32, day: u32) {
    let total_start = Instant::now();

    let input_parse_start = Instant::now();
    let solver = get_solver(year, day).unwrap()();
    let input_parse_time = &input_parse_start.elapsed().as_micros();

    let part_1_start = Instant::now();
    let part_1_result = solver.solve_part_1();
    let part_1_time = &part_1_start.elapsed().as_micros();

    let part_2_start = Instant::now();
    let part_2_result = solver.solve_part_2();
    let part_2_time = &part_2_start.elapsed().as_micros();

    let total_time = &total_start.elapsed().as_micros();

    println!("===== {}: Day {} =====", year, day);
    println!("  - Part 1: {}", part_1_result);
    println!("  - Part 2: {}", part_2_result);
    println!("  - Timings");
    println!("    - Parsing: {} μs", input_parse_time);
    println!("    - Part 1: {} μs", part_1_time);
    println!("    - Part 2: {} μs", part_2_time);
    println!("    - Total: {} μs", total_time);

    // println!("| {} | {} μs | {} μs | {} μs |", day, input_parse_time, part_1_time, part_2_time);
}

fn get_solver(year: u32, day: u32) -> Result<fn() -> Box<dyn AoCSolver>, String> {
    match year {
        2020 => match day {
            01 => Ok(|| Box::new(y_2020::day_01::Solver::create())),
            02 => Ok(|| Box::new(y_2020::day_02::Solver::create())),
            03 => Ok(|| Box::new(y_2020::day_03::Solver::create())),
            04 => Ok(|| Box::new(y_2020::day_04::Solver::create())),
            05 => Ok(|| Box::new(y_2020::day_05::Solver::create())),
            06 => Ok(|| Box::new(y_2020::day_06::Solver::create())),
            07 => Ok(|| Box::new(y_2020::day_07::Solver::create())),
            08 => Ok(|| Box::new(y_2020::day_08::Solver::create())),
            09 => Ok(|| Box::new(y_2020::day_09::Solver::create())),
            10 => Ok(|| Box::new(y_2020::day_10::Solver::create())),
            11 => Ok(|| Box::new(y_2020::day_11::Solver::create())),
            12 => Ok(|| Box::new(y_2020::day_12::Solver::create())),
            13 => Ok(|| Box::new(y_2020::day_13::Solver::create())),
            14 => Ok(|| Box::new(y_2020::day_14::Solver::create())),
            15 => Ok(|| Box::new(y_2020::day_15::Solver::create())),
            16 => Ok(|| Box::new(y_2020::day_16::Solver::create())),
            17 => Ok(|| Box::new(y_2020::day_17::Solver::create())),
            18 => Ok(|| Box::new(y_2020::day_18::Solver::create())),
            19 => Ok(|| Box::new(y_2020::day_19::Solver::create())),
            20 => Ok(|| Box::new(y_2020::day_20::Solver::create())),
            21 => Ok(|| Box::new(y_2020::day_21::Solver::create())),
            22 => Ok(|| Box::new(y_2020::day_22::Solver::create())),
            23 => Ok(|| Box::new(y_2020::day_23::Solver::create())),
            24 => Ok(|| Box::new(y_2020::day_24::Solver::create())),
            25 => Ok(|| Box::new(y_2020::day_25::Solver::create())),
            _ => Err(format!(
                "No solvers implemented for year {} day {}",
                year, day
            )),
        },
        2021 => match day {
            01 => Ok(|| Box::new(y_2021::day_01::Solver::create())),
            02 => Ok(|| Box::new(y_2021::day_02::Solver::create())),
            03 => Ok(|| Box::new(y_2021::day_03::Solver::create())),
            04 => Ok(|| Box::new(y_2021::day_04::Solver::create())),
            05 => Ok(|| Box::new(y_2021::day_05::Solver::create())),
            06 => Ok(|| Box::new(y_2021::day_06::Solver::create())),
            07 => Ok(|| Box::new(y_2021::day_07::Solver::create())),
            08 => Ok(|| Box::new(y_2021::day_08::Solver::create())),
            09 => Ok(|| Box::new(y_2021::day_09::Solver::create())),
            10 => Ok(|| Box::new(y_2021::day_10::Solver::create())),
            11 => Ok(|| Box::new(y_2021::day_11::Solver::create())),
            12 => Ok(|| Box::new(y_2021::day_12::Solver::create())),
            13 => Ok(|| Box::new(y_2021::day_13::Solver::create())),
            // 14 => Ok(|| Box::new(y_2021::day_14::Solver::create())),
            // 15 => Ok(|| Box::new(y_2021::day_15::Solver::create())),
            // 16 => Ok(|| Box::new(y_2021::day_16::Solver::create())),
            // 17 => Ok(|| Box::new(y_2021::day_17::Solver::create())),
            // 18 => Ok(|| Box::new(y_2021::day_18::Solver::create())),
            // 19 => Ok(|| Box::new(y_2021::day_19::Solver::create())),
            // 20 => Ok(|| Box::new(y_2021::day_20::Solver::create())),
            // 21 => Ok(|| Box::new(y_2021::day_21::Solver::create())),
            // 22 => Ok(|| Box::new(y_2021::day_22::Solver::create())),
            // 23 => Ok(|| Box::new(y_2021::day_23::Solver::create())),
            // 24 => Ok(|| Box::new(y_2021::day_24::Solver::create())),
            // 25 => Ok(|| Box::new(y_2021::day_25::Solver::create())),
            _ => Err(format!(
                "No solvers implemented for year {} day {}",
                year, day
            )),
        },
        2024 => match day {
            01 => Ok(|| Box::new(y_2024::day_01::Solver::create())),
            02 => Ok(|| Box::new(y_2024::day_02::Solver::create())),
            03 => Ok(|| Box::new(y_2024::day_03::Solver::create())),
            04 => Ok(|| Box::new(y_2024::day_04::Solver::create())),
            05 => Ok(|| Box::new(y_2024::day_05::Solver::create())),
            06 => Ok(|| Box::new(y_2024::day_06::Solver::create())),
            08 => Ok(|| Box::new(y_2024::day_08::Solver::create())),
            _ => Err(format!(
                "No solvers implemented for year {} day {}",
                year, day
            )),
        },
        _ => Err(format!(
            "No solvers implemented for year {} day {}",
            year, day
        )),
    }
}
