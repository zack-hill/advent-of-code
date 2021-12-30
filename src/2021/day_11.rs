use crate::solver::AoCSolver;
use colored::*;
use std::{fs::File, io::BufRead, io::BufReader};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const BUFFERED_WIDTH: usize = 12;
const BUFFERED_HEIGHT: usize = 12;

type Grid = [[u8; BUFFERED_WIDTH]; BUFFERED_HEIGHT];

pub struct Solver {
    grid: Grid,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            grid: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut grid_copy = self.grid.clone();
        let mut flash_count = 0;

        // print_grid(&grid_copy);
        // println!("");

        for _ in 0..100 {
            flash_count += run_cycle(&mut grid_copy);
            // println!("After Step {}", step + 1);
            // print_grid(&grid_copy);
            // println!("");
        }

        return flash_count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut grid_copy = self.grid.clone();
        let mut step = 0;
        loop {
            step += 1;
            let flash_count = run_cycle(&mut grid_copy);
            if flash_count == 100 {
                break;
            }
        }
        return step.to_string();
    }
}

fn run_cycle(grid: &mut Grid) -> u32 {
    // 1) Increment all cells by one
    for y in 1..=HEIGHT {
        for x in 1..=WIDTH {
            grid[y][x] += 1;
        }
    }

    // 2) Perform Flashes
    let mut flash_count = 0;
    for y in 1..=HEIGHT {
        for x in 1..=WIDTH {
            flash_count += perform_flash(grid, (x, y));
        }
    }

    return flash_count;
}

fn perform_flash(grid: &mut Grid, position: (usize, usize)) -> u32 {
    let (x, y) = position;
    let power = grid[y][x];
    let mut flash_count = 0;
    if power > 9 {
        grid[y][x] = 0;
        flash_count += 1;
        flash_count += try_flash_neighbor(grid, (x - 1, y - 1));
        flash_count += try_flash_neighbor(grid, (x, y - 1));
        flash_count += try_flash_neighbor(grid, (x + 1, y - 1));
        flash_count += try_flash_neighbor(grid, (x - 1, y));
        flash_count += try_flash_neighbor(grid, (x + 1, y));
        flash_count += try_flash_neighbor(grid, (x - 1, y + 1));
        flash_count += try_flash_neighbor(grid, (x, y + 1));
        flash_count += try_flash_neighbor(grid, (x + 1, y + 1));
    }
    return flash_count;
}

fn try_flash_neighbor(grid: &mut Grid, position: (usize, usize)) -> u32 {
    let (x, y) = position;
    if grid[y][x] != 0 {
        grid[y][x] += 1;
        return perform_flash(grid, position);
    }
    return 0;
}

pub fn print_grid(grid: &Grid) {
    for y in 1..=HEIGHT {
        for x in 1..=WIDTH {
            let value = grid[y][x];
            if value == 0 {
                print!("{}", value.to_string().bold());
            } else {
                print!("{}", value.to_string());
            }
        }
        println!("");
    }
}

pub fn parse_input() -> Grid {
    let file = File::open("src/2021/day_11.txt").unwrap();
    let reader = BufReader::new(file);

    // A buffer is added to the grid to make it so we can skip bounds checks when iterating over the cells.
    let mut grid = [[0u8; BUFFERED_WIDTH]; BUFFERED_HEIGHT];

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for (x, char) in line.chars().enumerate() {
            let value = char.to_digit(10).unwrap();
            grid[y + 1][x + 1] = value as u8;
        }
    }

    return grid;
}
