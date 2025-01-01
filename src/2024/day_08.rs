use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::solver::AoCSolver;
use crate::util::point_2d::Point2D;

#[derive(Clone, Copy, PartialEq)]
enum AntinodeGenerationMode {
    Part1,
    Part2,
}

#[derive(Clone)]
struct Antenna {
    frequency: char,
    position: Point2D,
}

pub struct Solver {
    antennas: Vec<Antenna>,
    x_max: usize,
    y_max: usize,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2024/day_08.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
        let mut antennas = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character == '.' || character == '#' {
                    continue;
                }
                let antenna = Antenna {
                    frequency: character,
                    position: Point2D {
                        x: x as isize,
                        y: y as isize,
                    },
                };
                antennas.push(antenna);
            }
        }
        Solver {
            antennas,
            x_max: lines[0].len() - 1,
            y_max: lines.len() - 1,
        }
    }

    fn count_antinodes(&self, antinode_generation_mode: AntinodeGenerationMode) -> usize {
        let count = &self
            .antennas
            .iter()
            .sorted_by_key(|a| a.frequency)
            .group_by(|a| a.frequency)
            .into_iter()
            .flat_map(|(_, antennas)| {
                antennas
                    .into_iter()
                    .combinations(2)
                    .flat_map(|antenna_pair| {
                        self.get_in_bounds_antinodes(
                            antenna_pair[0].position,
                            antenna_pair[1].position,
                            antinode_generation_mode,
                        )
                    })
            })
            .unique()
            .count();
        return *count;
    }

    fn get_in_bounds_antinodes(
        &self,
        p1: Point2D,
        p2: Point2D,
        antinode_generation_mode: AntinodeGenerationMode,
    ) -> Vec<Point2D> {
        let offset = p2 - p1;
        let mut antinodes = self.get_positions_in_dir(p1, offset, antinode_generation_mode);
        antinodes.append(&mut self.get_positions_in_dir(p2, -offset, antinode_generation_mode));
        return antinodes;
    }

    fn get_positions_in_dir(
        &self,
        pos: Point2D,
        offset: Point2D,
        antinode_generation_mode: AntinodeGenerationMode,
    ) -> Vec<Point2D> {
        let mut positions = Vec::new();

        if antinode_generation_mode == AntinodeGenerationMode::Part2 {
            positions.push(pos);
        }

        let mut pos = pos;
        loop {
            pos = pos - offset;
            if self.is_pos_in_bounds(pos) {
                positions.push(pos);
            } else {
                break;
            }
            if antinode_generation_mode == AntinodeGenerationMode::Part1 {
                break;
            }
        }
        return positions;
    }

    fn is_pos_in_bounds(&self, pos: Point2D) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x <= self.x_max as isize && pos.y <= self.y_max as isize
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        self.count_antinodes(AntinodeGenerationMode::Part1)
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.count_antinodes(AntinodeGenerationMode::Part2)
            .to_string()
    }
}
