use crate::solver::AoCSolver;
use std::{fs::File, io::BufRead, io::BufReader};

pub struct Solver {
    line_segments: Vec<LineSegment>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            line_segments: parse_input(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn get_points(&self) -> Vec<Point> {
        let x_increment = LineSegment::get_increment(self.start.x, self.end.x);
        let y_increment = LineSegment::get_increment(self.start.y, self.end.y);

        let mut x = self.start.x;
        let mut y = self.start.y;

        let mut points = vec![];

        while x != self.end.x || y != self.end.y {
            points.push(Point { x, y });
            x = LineSegment::add(x, x_increment);
            y = LineSegment::add(y, y_increment);
        }

        points.push(self.end.clone());

        return points;
    }

    fn get_increment(left: usize, right: usize) -> isize {
        if left < right {
            1
        } else if left > right {
            -1
        } else {
            0
        }
    }

    fn add(u: usize, i: isize) -> usize {
        if i.is_negative() {
            u - i.wrapping_abs() as usize
        } else {
            u + i as usize
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let non_diagonal_lines: Vec<&LineSegment> = self
            .line_segments
            .iter()
            .filter(|l| l.is_horizontal() || l.is_vertical())
            .collect();

        let max_x = std::cmp::max(
            non_diagonal_lines.iter().map(|l| l.start.x).max().unwrap(),
            non_diagonal_lines.iter().map(|l| l.end.x).max().unwrap(),
        );
        let max_y = std::cmp::max(
            non_diagonal_lines.iter().map(|l| l.start.y).max().unwrap(),
            non_diagonal_lines.iter().map(|l| l.end.y).max().unwrap(),
        );

        let mut grid = vec![vec![0usize; max_x + 1]; max_y + 1];

        for line in non_diagonal_lines {
            for point in line.get_points() {
                grid[point.y][point.x] += 1;
            }
        }

        return grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&p| p >= 2)
            .count()
            .to_string();
    }

    fn solve_part_2(&self) -> String {
        let max_x = std::cmp::max(
            self.line_segments.iter().map(|l| l.start.x).max().unwrap(),
            self.line_segments.iter().map(|l| l.end.x).max().unwrap(),
        );
        let max_y = std::cmp::max(
            self.line_segments.iter().map(|l| l.start.y).max().unwrap(),
            self.line_segments.iter().map(|l| l.end.y).max().unwrap(),
        );

        let mut grid = vec![vec![0usize; max_x + 1]; max_y + 1];

        for line in self.line_segments.iter() {
            for point in line.get_points() {
                grid[point.y][point.x] += 1;
            }
        }

        return grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&p| p >= 2)
            .count()
            .to_string();
    }
}

pub fn parse_input() -> Vec<LineSegment> {
    let file = File::open("src/2021/day_05.txt").unwrap();
    let reader = BufReader::new(file);

    let mut line_segments = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let values: Vec<usize> = line
            .split("->")
            .flat_map(|x| x.split(","))
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let line_segment = LineSegment {
            start: Point {
                x: values[0],
                y: values[1],
            },
            end: Point {
                x: values[2],
                y: values[3],
            },
        };
        line_segments.push(line_segment);
    }
    return line_segments;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_points_horizontal() {
        let line = LineSegment {
            start: Point { x: 3, y: 4 },
            end: Point { x: 5, y: 4 },
        };
        let expected = vec![
            Point { x: 3, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 4 },
        ];

        let actual = line.get_points();

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_points_horizontal_rev() {
        let line = LineSegment {
            end: Point { x: 5, y: 4 },
            start: Point { x: 3, y: 4 },
        };
        let expected = vec![
            Point { x: 3, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 4 },
        ];

        let actual = line.get_points();

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_points_vertical() {
        let line = LineSegment {
            start: Point { x: 4, y: 3 },
            end: Point { x: 4, y: 5 },
        };
        let expected = vec![
            Point { x: 4, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 4, y: 5 },
        ];

        let actual = line.get_points();

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_points_vertical_rev() {
        let line = LineSegment {
            start: Point { x: 4, y: 5 },
            end: Point { x: 4, y: 3 },
        };
        let expected = vec![
            Point { x: 4, y: 5 },
            Point { x: 4, y: 4 },
            Point { x: 4, y: 3 },
        ];

        let actual = line.get_points();

        assert_eq!(expected, actual);
    }
}
