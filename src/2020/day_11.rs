use crate::solver::AoCSolver;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let mut grid = self.grid.clone();
        simulate_until_stable(&mut grid, 4, 1);
        return count_occupied_seats(&grid).to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut grid = self.grid.clone();
        simulate_until_stable(&mut grid, 5, usize::MAX);
        return count_occupied_seats(&grid).to_string();
    }
}

// fn print(grid: &Grid) {
//     for row in grid {
//         let s: String = row.into_iter().clone().collect();
//         println!("{}", s);
//     }
//     println!("");
// }

fn simulate_until_stable(grid: &mut Grid, min_seats_to_vacate: usize, sight_distance: usize) {
    while simulate(grid, min_seats_to_vacate, sight_distance) {}
}

fn simulate(grid: &mut Grid, min_seats_to_vacate: usize, sight_distance: usize) -> bool {
    let starting_grid = grid.clone();

    let mut changed = false;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match starting_grid[y][x] {
                'L' => {
                    if !occupied_seats_in_sight_at_least(&starting_grid, x, y, 1, sight_distance) {
                        changed = true;
                        grid[y][x] = '#'
                    }
                }
                '#' => {
                    if occupied_seats_in_sight_at_least(
                        &starting_grid,
                        x,
                        y,
                        min_seats_to_vacate,
                        sight_distance,
                    ) {
                        changed = true;
                        grid[y][x] = 'L'
                    }
                }
                _ => {}
            };
        }
    }

    return changed;
}

fn occupied_seats_in_sight_at_least(
    grid: &Grid,
    x: usize,
    y: usize,
    min_occupied: usize,
    max_distance: usize,
) -> bool {
    let mut occupied_count: usize = 0;
    let directions: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for (x_dir, y_dir) in directions.iter() {
        if check_in_direction(grid, x, y, *x_dir, *y_dir, max_distance) {
            occupied_count += 1;
            if occupied_count == min_occupied {
                return true;
            }
        }
    }

    return false;
}

fn check_in_direction(
    grid: &Grid,
    x_start: usize,
    y_start: usize,
    x_step: isize,
    y_step: isize,
    max_distance: usize,
) -> bool {
    let mut x = x_start as isize;
    let mut y = y_start as isize;
    let mut distance = 0;
    loop {
        x += x_step;
        y += y_step;
        distance += 1;

        // Check if the coordinates are out of bounds.
        if x < 0 || x >= grid[0].len() as isize || y < 0 || y >= grid.len() as isize {
            return false;
        }

        // Check to see if we found a seat.
        match grid[y as usize][x as usize] {
            '#' => return true,
            'L' => return false,
            _ => {}
        };

        if distance == max_distance {
            return false;
        }
    }
}

fn count_occupied_seats(grid: &Grid) -> usize {
    return grid.iter().flatten().filter(|&x| x == &'#').count();
}

pub fn parse_input() -> Grid {
    let file = File::open("src/2020/day_11.txt").unwrap();
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    return grid;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupied_seats_none_found() {
        let grid = vec![
            vec!['.', '#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '#', '.', '#', '.', '#'],
            vec!['#', '#', '.', '.', '.', '#', '#'],
            vec!['.', '.', '.', 'L', '.', '.', '.'],
            vec!['#', '#', '.', '.', '.', '#', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#'],
            vec!['.', '#', '#', '.', '#', '#', '.'],
        ];
        assert_eq!(
            occupied_seats_in_sight_at_least(&grid, 3, 3, 1, usize::MAX),
            false
        );
    }

    #[test]
    fn occupied_seats_all_found() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', 'L', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(
            occupied_seats_in_sight_at_least(&grid, 3, 4, 8, usize::MAX),
            true
        );
    }

    #[test]
    fn occupied_seats_dist_capped() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', 'L', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(occupied_seats_in_sight_at_least(&grid, 3, 4, 3, 2), true);
    }
}
