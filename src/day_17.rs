use std::io::{BufRead, BufReader};
use std::{cmp::max, cmp::min};
use std::{collections::HashSet, fs::File};

pub fn solve_puzzle_1() -> usize {
    let mut points = parse_file();
    for _cycle in 0..6 {
        let mut new_points = HashSet::<Point>::new();
        let (x_range, y_range, z_range) = get_bounds(&points);

        for x in (x_range.min - 1)..=(x_range.max + 1) {
            for y in (y_range.min - 1)..=(y_range.max + 1) {
                for z in (z_range.min - 1)..=(z_range.max + 1) {
                    // println!("({}, {}, {})", x, y, z);
                    let point = Point::new(x, y, z);
                    let count = count_active_neighbors(&point, &points);
                    if count == 3 || (count == 2 && points.contains(&point)) {
                        // println!("{:?},  {:?}", point, count);
                        new_points.insert(point);
                    }
                }
            }
        }

        points = new_points;

        // println!("X Range: {:?}", x_range);
        // println!("Y Range: {:?}", y_range);
        // println!("Z Range: {:?}", z_range);
    }

    return points.len();
}

pub fn solve_puzzle_2() -> i32 {
    return 0;
}

fn get_bounds(points: &HashSet<Point>) -> (Range, Range, Range) {
    let mut x_range = Range::empty();
    let mut y_range = Range::empty();
    let mut z_range = Range::empty();
    for point in points {
        x_range = expand_range(&x_range, point.x);
        y_range = expand_range(&y_range, point.y);
        z_range = expand_range(&z_range, point.z);
    }
    (x_range, y_range, z_range)
}

fn expand_range(range: &Range, value: i32) -> Range {
    Range {
        min: min(range.min, value),
        max: max(range.max, value),
    }
}

fn count_active_neighbors(point: &Point, points: &HashSet<Point>) -> u32 {
    let mut active_neightbors = 0;
    for x in (point.x - 1)..=(point.x + 1) {
        for y in (point.y - 1)..=(point.y + 1) {
            for z in (point.z - 1)..=(point.z + 1) {
                let neighbor = Point::new(x, y, z);
                if &neighbor == point {
                    // println!("{:?} = {:?}", neighbor, point);
                    continue;
                }
                if points.contains(&neighbor) {
                    // println!("{:?} contains {:?}", points, neighbor);
                    active_neightbors += 1;
                }
            }
        }
    }
    return active_neightbors;
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

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
    let file = File::open("src/day_17.txt").unwrap();
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
            data.insert(Point::new(x as i32, y as i32, 0));
        }
    }
    return data;
}
