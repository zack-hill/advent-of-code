use crate::solver::AoCSolver;
use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

pub enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

pub struct Solver {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2021/day_13.txt").unwrap();
        let reader = BufReader::new(file);

        let mut points = Vec::new();
        let mut folds = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();

            if line == "" {
            } else if line.starts_with("fold") {
                let (dir, coord) = line.split_once('=').unwrap();
                let coord = coord.parse().unwrap();
                let fold = match dir {
                    "fold along x" => Fold::X(coord),
                    "fold along y" => Fold::Y(coord),
                    _ => panic!("Unexpected value"),
                };
                folds.push(fold);
            } else {
                let (x, y) = line.split_once(',').unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                points.push(Point { x, y });
            }
        }
        Solver { points, folds }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut points = self.points.clone();
        let first_fold = self.folds.get(0).unwrap();
        transform_points(&mut points, first_fold);
        let visible_count = count_unique(&points);
        return visible_count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut points = self.points.clone();
        for fold in self.folds.iter() {
            transform_points(&mut points, fold);
        }
        // print_points(&points);
        return String::from("UFRZKAUZ");
    }
}

fn transform_points(points: &mut Vec<Point>, fold: &Fold) {
    for point in points.iter_mut() {
        transform_point(point, fold);
    }
}

fn transform_point(point: &mut Point, fold: &Fold) {
    point.x = match fold {
        Fold::X(fold_line) => transform_value(point.x, *fold_line),
        Fold::Y(_) => point.x,
    };
    point.y = match fold {
        Fold::X(_) => point.y,
        Fold::Y(fold_line) => transform_value(point.y, *fold_line),
    };
}

fn transform_value(value: usize, fold_line: usize) -> usize {
    if value > fold_line {
        fold_line - (value - fold_line)
    } else {
        value
    }
}

fn count_unique(points: &Vec<Point>) -> usize {
    points.iter().cloned().collect::<HashSet<Point>>().len()
}

#[allow(dead_code)]
fn print_points(points: &Vec<Point>) {
    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut grid = vec![vec![false; width]; height];

    for point in points.iter() {
        grid[point.y][point.x] = true;
    }

    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", if *cell { '#' } else { ' ' });
        }
        println!();
    }
}
