use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::{cmp::max, cmp::min};
use std::{collections::HashSet, fs::File};

pub fn solve_puzzle_1() -> usize {
    solve("src/day_17.txt", 3)
}

pub fn solve_puzzle_2() -> usize {
    solve("src/day_17.txt", 4)
}

fn solve(file: &str, dimensions: u32) -> usize {
    let mut points = parse_file(file, dimensions);
    for _cycle in 0..6 {
        let mut new_points = HashSet::<Vec<i32>>::new();
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

fn get_bounds(points: &HashSet<Vec<i32>>, dimensions: u32) -> Vec<Range> {
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

fn count_active_neighbors(point: &Vec<i32>, points: &HashSet<Vec<i32>>) -> u32 {
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

fn parse_file(path: &str, dimensions: u32) -> HashSet<Vec<i32>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut data = HashSet::<Vec<i32>>::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for x in line
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(x, _)| x)
        {
            let mut point = vec![x as i32, y as i32];
            for _ in 2..dimensions {
                point.push(0);
            }
            data.insert(point);
        }
    }
    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_dim() {
        assert_eq!(112, solve("src/day_17_example.txt", 3));
    }

    #[test]
    fn puzzle_1() {
        assert_eq!(213, solve("src/day_17.txt", 3));
    }

    #[test]
    fn example_4_dim() {
        assert_eq!(848, solve("src/day_17_example.txt", 4));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(1624, solve("src/day_17.txt", 4));
    }
}
