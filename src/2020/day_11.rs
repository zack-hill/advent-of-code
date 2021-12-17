use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    let mut grid = parse_file();
    loop {
        let new_grid = simulate(&grid, 4, 1);
        if !is_changed(&new_grid, &grid) {
            break;
        }
        grid = new_grid;
    }
    return count_occupied_seats(&grid);
}

pub fn solve_puzzle_2() -> usize {
    let mut grid = parse_file();
    loop {
        let new_grid = simulate(&grid, 5, usize::MAX);
        // print(&grid);
        if !is_changed(&new_grid, &grid) {
            break;
        }
        grid = new_grid;
    }
    return count_occupied_seats(&grid);
}

fn print(grid: &Vec<Vec<char>>) {
    for row in grid {
        let s: String = row.into_iter().clone().collect();
        println!("{}", s);
    }
    println!("");
}

fn simulate(
    starting_grid: &Vec<Vec<char>>,
    min_seats_to_vacate: usize,
    sight_distance: usize,
) -> Vec<Vec<char>> {
    let mut grid = starting_grid.clone();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            grid[y][x] = match starting_grid[y][x] {
                'L' => {
                    if occupied_seats_in_sight_at_least(&starting_grid, x, y, 1, sight_distance) {
                        'L'
                    } else {
                        '#'
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
                        'L'
                    } else {
                        '#'
                    }
                }
                _ => starting_grid[y][x],
            };
        }
    }

    return grid;
}

fn occupied_seats_in_sight_at_least(
    grid: &Vec<Vec<char>>,
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
    grid: &Vec<Vec<char>>,
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

fn is_changed(grid_1: &Vec<Vec<char>>, grid_2: &Vec<Vec<char>>) -> bool {
    for y in 0..grid_1.len() {
        for x in 0..grid_1[0].len() {
            if grid_1[y][x] != grid_2[y][x] {
                return true;
            }
        }
    }
    return false;
}

fn count_occupied_seats(grid: &Vec<Vec<char>>) -> usize {
    return grid.iter().flatten().filter(|x| **x == '#').count();
}

fn parse_file() -> Vec<Vec<char>> {
    let file = File::open("src/day_11.txt").unwrap();
    let reader = BufReader::new(file);
    let values: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    return values;
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
