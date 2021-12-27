use itertools::Itertools;

use crate::solver::AoCSolver;
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::BufRead,
    io::BufReader,
};

type Grid = Vec<Vec<u8>>;

pub struct Solver {
    height_map: Grid,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            height_map: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let total_risk = get_low_points(&self.height_map)
            .iter()
            .map(|(x, y)| self.height_map[*y][*x] as u32 + 1)
            .sum::<u32>();
        return total_risk.to_string();
    }

    fn solve_part_2(&self) -> String {
        let low_points = get_low_points(&self.height_map);
        let basin_sizes: Vec<usize> = low_points
            .iter()
            .map(|&(x, y)| get_basin_size(&self.height_map, x, y))
            .collect();
        let answer: usize = basin_sizes.iter().sorted().rev().take(3).product();
        return answer.to_string();
    }
}

fn get_low_points(grid: &Grid) -> Vec<(usize, usize)> {
    let width = grid[0].len();
    let height = grid.len();
    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if is_low_point(&grid, x, y) {
                low_points.push((x, y));
            }
        }
    }
    return low_points;
}

fn is_low_point(grid: &Grid, x: usize, y: usize) -> bool {
    let value = grid[y][x];
    let dirs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for dir in dirs.into_iter() {
        if let Some(neighbor) = offset_point((x, y), dir) {
            if let Some(&neighbor_value) = get_grid_value(grid, neighbor) {
                if neighbor_value <= value {
                    return false;
                }
            }
        }
    }

    return true;
}

fn get_basin_size(grid: &Grid, x: usize, y: usize) -> usize {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((x, y));
    let dirs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(point) = to_visit.pop_front() {
        for dir in dirs.iter() {
            if let Some(neighbor) = offset_point(point, *dir) {
                if !visited.contains(&neighbor) {
                    if let Some(&neighbor_value) = get_grid_value(grid, neighbor) {
                        if neighbor_value != 9 {
                            to_visit.push_back(neighbor);
                        }
                    }
                }
            }
        }
        visited.insert(point);
    }
    return visited.len();
}

fn offset_point(point: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    if let (Some(new_x), Some(new_y)) =
        (add_offset(point.0, offset.0), add_offset(point.1, offset.1))
    {
        return Some((new_x, new_y));
    }
    return None;
}

fn add_offset(value: usize, offset: isize) -> Option<usize> {
    if offset < 0 {
        value.checked_sub(offset.wrapping_abs() as usize)
    } else {
        value.checked_add(offset as usize)
    }
}

fn get_grid_value(grid: &Grid, point: (usize, usize)) -> Option<&u8> {
    grid.get(point.1)?.get(point.0)
}

pub fn parse_input() -> Grid {
    let file = File::open("src/2021/day_09.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rows = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let values = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        rows.push(values);
    }

    return rows;
}
