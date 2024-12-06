use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::solver::AoCSolver;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum MoveResult {
    Success(Position),
    Obstacle,
    OutOfBounds,
}

enum PatrolResult {
    Success(HashSet<Position>),
    LoopDetected,
}

pub struct Solver {
    starting_position: Position,
    obstacles: HashSet<Position>,
    width: usize,
    height: usize,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2024/day_06.txt").unwrap();
        let reader = BufReader::new(file);
        let mut starting_position: Option<Position> = None;
        let mut obstacles = HashSet::new();

        let mut y = 0;
        let mut width = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            width = line.len();

            let mut x = 0;

            for character in line.chars() {
                if character == '#' {
                    obstacles.insert((x, y));
                } else if character == '^' {
                    starting_position = Some((x, y));
                }
                x += 1;
            }
            y += 1;
        }

        let height = y - 1;
        Solver {
            starting_position: starting_position.expect("No starting position found"),
            obstacles,
            width,
            height,
        }
    }

    fn move_in_direction(
        &self,
        (x, y): &Position,
        direction: &Direction,
        new_obstacle: Option<Position>,
    ) -> MoveResult {
        let (x, y) = (*x, *y);
        let new_position = match direction {
            Direction::Up => {
                if y == 0 {
                    return MoveResult::OutOfBounds;
                } else {
                    (x, y - 1)
                }
            }
            Direction::Down => {
                if y == self.height {
                    return MoveResult::OutOfBounds;
                } else {
                    (x, y + 1)
                }
            }
            Direction::Left => {
                if x == 0 {
                    return MoveResult::OutOfBounds;
                } else {
                    (x - 1, y)
                }
            }
            Direction::Right => {
                if x == self.width {
                    return MoveResult::OutOfBounds;
                } else {
                    (x + 1, y)
                }
            }
        };
        if self.obstacles.contains(&new_position)
            || new_obstacle.is_some_and(|obstacle| obstacle == new_position)
        {
            MoveResult::Obstacle
        } else {
            MoveResult::Success(new_position)
        }
    }

    fn rotate(direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn patrol(&self, new_obstacle: Option<Position>) -> PatrolResult {
        let mut visited_cells = HashSet::new();
        let mut position = self.starting_position;
        let mut direction = Direction::Up;
        visited_cells.insert((position, direction));
        loop {
            match self.move_in_direction(&position, &direction, new_obstacle) {
                MoveResult::Success(new_position) => {
                    // println!("Moved to {:?}", new_position);
                    position = new_position;
                    if !visited_cells.insert((position, direction)) {
                        return PatrolResult::LoopDetected;
                    }
                }
                MoveResult::Obstacle => {
                    direction = {
                        // println!(
                        //     "Obstacle found at {:?}. New direction is {:?}",
                        //     new_position, direction
                        // );
                        Solver::rotate(&direction)
                    }
                }
                MoveResult::OutOfBounds => {
                    // println!("Moved out of bounds");
                    return PatrolResult::Success(
                        visited_cells
                            .iter()
                            .map(|(position, _)| *position)
                            .collect(),
                    );
                }
            }
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        match self.patrol(None) {
            PatrolResult::Success(visited_positions) => return visited_positions.len().to_string(),
            PatrolResult::LoopDetected => panic!("Loop detected, that shouldn't happen here!"),
        }
    }

    fn solve_part_2(&self) -> String {
        let default_visited_cells = match self.patrol(None) {
            PatrolResult::Success(visited_positions) => visited_positions,
            PatrolResult::LoopDetected => panic!("Loop detected, that shouldn't happen here!"),
        };

        let mut positions_which_cause_a_loop = 0;
        for position in default_visited_cells {
            match self.patrol(Some(position)) {
                PatrolResult::Success(_) => {}
                PatrolResult::LoopDetected => positions_which_cause_a_loop += 1,
            }
        }
        return positions_which_cause_a_loop.to_string();
    }
}
