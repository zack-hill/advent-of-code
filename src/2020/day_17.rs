use crate::solver::AoCSolver;
use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::{cmp::max, cmp::min};
use std::{collections::HashSet, fs::File};

type Point = Vec<i32>;

pub struct Solver {
    points: HashSet<Point>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            points: parse_file(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let points = self
            .points
            .iter()
            .map(|point| vec![point[0], point[1], 0])
            .collect();
        solve(&points, 3).to_string()
    }

    fn solve_part_2(&self) -> String {
        let points = self
            .points
            .iter()
            .map(|point| vec![point[0], point[1], 0, 0])
            .collect();
        solve(&points, 4).to_string()
    }
}

fn solve(points: &HashSet<Point>, dimensions: u32) -> usize {
    let mut points = points.clone();
    for _cycle in 0..6 {
        let mut new_points = HashSet::<Point>::new();
        let bounds = get_bounds(&points, dimensions);

        for point in bounds
            .iter()
            .map(|b| (b.min - 1)..=(b.max + 1))
            .multi_cartesian_product()
        {
            let count = count_active_neighbors(&point, &points);
            if count == 3 || (count == 2 && points.contains(&point)) {
                new_points.insert(point);
            }
        }

        points = new_points;
        // println!("Cycle {} has {} active points", _cycle, points.len());
    }

    return points.len();
}

fn get_bounds(points: &HashSet<Point>, dimensions: u32) -> Vec<Range> {
    let mut bounds = vec![Range::empty(); dimensions as usize];
    for point in points {
        for d in 0..dimensions as usize {
            bounds[d] = expand_range(bounds.get(d).unwrap(), point[d]);
        }
    }
    return bounds;
}

fn expand_range(range: &Range, value: i32) -> Range {
    Range {
        min: min(range.min, value),
        max: max(range.max, value),
    }
}

fn count_active_neighbors(point: &Point, points: &HashSet<Point>) -> u32 {
    let mut active_neightbors = 0;
    for neighbor in point
        .iter()
        .map(|d| (d - 1)..=(d + 1))
        .multi_cartesian_product()
    {
        if &neighbor == point {
            // println!("{:?} = {:?}", neighbor, point);
            continue;
        }
        if points.contains(&neighbor) {
            // println!("{:?} contains {:?}", points, neighbor);
            active_neightbors += 1;
        }
    }
    return active_neightbors;
}

#[derive(Clone)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn empty() -> Range {
        Range {
            min: i32::MAX,
            max: i32::MIN,
        }
    }
}

fn parse_file() -> HashSet<Point> {
    let file = File::open("src/2020/day_17.txt").unwrap();
    let reader = BufReader::new(file);

    let mut data = HashSet::<Point>::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for x in line
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(x, _)| x)
        {
            data.insert(vec![x as i32, y as i32]);
        }
    }
    return data;
}
