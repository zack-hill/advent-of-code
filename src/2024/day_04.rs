use crate::solver::AoCSolver;
use std::{fs::File, io::BufRead, io::BufReader};

type Grid = Vec<Vec<char>>;

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
        let mut count = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                count += search_for_xmas_in_direction(&self.grid, x, y, 1, 0);
                count += search_for_xmas_in_direction(&self.grid, x, y, -1, 0);
                count += search_for_xmas_in_direction(&self.grid, x, y, 0, 1);
                count += search_for_xmas_in_direction(&self.grid, x, y, 0, -1);
                count += search_for_xmas_in_direction(&self.grid, x, y, 1, 1);
                count += search_for_xmas_in_direction(&self.grid, x, y, 1, -1);
                count += search_for_xmas_in_direction(&self.grid, x, y, -1, 1);
                count += search_for_xmas_in_direction(&self.grid, x, y, -1, -1);
            }
        }
        return count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut count = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if search_for_mas_x(&self.grid, x as isize, y as isize) {
                    count += 1;
                }
            }
        }
        return count.to_string();
    }
}

fn search_for_xmas_in_direction(
    grid: &Grid,
    x_start: usize,
    y_start: usize,
    x_step: isize,
    y_step: isize,
) -> usize {
    let mut x = x_start as isize;
    let mut y = y_start as isize;
    const EXPECTED: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut distance = 0;
    let max_distance = EXPECTED.len();
    loop {
        let Some(character) = get_character(grid, x, y) else {
            return 0;
        };

        if character != EXPECTED[distance] {
            return 0;
        }

        x += x_step;
        y += y_step;
        distance += 1;

        if distance == max_distance {
            return 1;
        }
    }
}

fn search_for_mas_x(grid: &Grid, x: isize, y: isize) -> bool {
    let Some(center) = get_character(grid, x, y) else {
        return false;
    };
    if center != 'A' {
        return false;
    }

    let Some(top_left) = get_character(grid, x - 1, y - 1) else {
        return false;
    };
    let Some(top_right) = get_character(grid, x + 1, y - 1) else {
        return false;
    };
    let Some(bottom_left) = get_character(grid, x - 1, y + 1) else {
        return false;
    };
    let Some(bottom_right) = get_character(grid, x + 1, y + 1) else {
        return false;
    };

    return (top_left == 'M' && bottom_right == 'S' || (top_left == 'S' && bottom_right == 'M'))
        && ((bottom_left == 'M' && top_right == 'S') || (bottom_left == 'S' && top_right == 'M'));
}

fn get_character(grid: &Grid, x: isize, y: isize) -> Option<char> {
    // Check if the coordinates are out of bounds.
    if x < 0 || x >= grid[0].len() as isize || y < 0 || y >= grid.len() as isize {
        return None;
    }

    return Some(grid[y as usize][x as usize]);
}

pub fn parse_input() -> Grid {
    let file = File::open("src/2024/day_04.txt").unwrap();
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    return grid;
}
