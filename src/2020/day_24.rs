use crate::solver::AoCSolver;
use nom::{branch::alt, bytes::complete::tag, combinator::map_res, multi::many1, IResult};
use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader, str::FromStr};
use Direction::*;

pub struct Solver {
    direction_sets: Vec<Vec<Direction>>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            direction_sets: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let tiles = flip_tiles_using_directions(&self.direction_sets);
        return tiles.len().to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut tiles = flip_tiles_using_directions(&self.direction_sets);

        for _ in 0..100 {
            tiles = flip_tiles_using_rules(&tiles);
        }
        return tiles.len().to_string();
    }
}

fn flip_tiles_using_directions(direction_sets: &Vec<Vec<Direction>>) -> HashSet<(i32, i32)> {
    let mut tiles = HashSet::<(i32, i32)>::new();
    for direction_set in direction_sets.iter() {
        let mut pos = (0, 0);
        for direction in direction_set.iter() {
            pos = shift_coord(&pos, direction);
        }

        toggle_tile(pos, &mut tiles);
    }
    return tiles;
}

fn flip_tiles_using_rules(tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_tiles = tiles.clone();
    let min_x = tiles.iter().map(|(x, _)| x).min().unwrap();
    let min_y = tiles.iter().map(|(_, y)| y).min().unwrap();
    let max_x = tiles.iter().map(|(x, _)| x).max().unwrap();
    let max_y = tiles.iter().map(|(_, y)| y).max().unwrap();
    for y in min_y - 1..=max_y + 1 {
        for x in min_x - 1..=max_x + 1 {
            let adjacent_count = count_adjacent_tiles((x, y), tiles);
            let is_black = tiles.contains(&(x, y));
            if (is_black && (adjacent_count == 0 || adjacent_count > 2))
                || (!is_black && adjacent_count == 2)
            {
                toggle_tile((x, y), &mut new_tiles);
            }
        }
    }
    return new_tiles;
}

fn count_adjacent_tiles(pos: (i32, i32), tiles: &HashSet<(i32, i32)>) -> usize {
    let directions = [East, SouthEast, SouthWest, West, NorthWest, NorthEast];
    return directions
        .iter()
        .map(|dir| tiles.contains(&shift_coord(&pos, dir)))
        .filter(|&result| result)
        .count();
}

fn shift_coord(pos: &(i32, i32), direction: &Direction) -> (i32, i32) {
    let (x, y) = pos;
    let is_even = y % 2 == 0;
    let (dx, dy) = match direction {
        East => (1, 0),
        SouthEast => (if is_even { 0 } else { 1 }, -1),
        SouthWest => (if is_even { -1 } else { 0 }, -1),
        West => (-1, 0),
        NorthWest => (if is_even { -1 } else { 0 }, 1),
        NorthEast => (if is_even { 0 } else { 1 }, 1),
    };
    return (x + dx, y + dy);
}

fn toggle_tile(pos: (i32, i32), tiles: &mut HashSet<(i32, i32)>) {
    let added = tiles.insert(pos);
    if !added {
        tiles.remove(&pos);
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::East),
            "se" => Ok(Direction::SouthEast),
            "sw" => Ok(Direction::SouthWest),
            "w" => Ok(Direction::West),
            "nw" => Ok(Direction::NorthWest),
            "ne" => Ok(Direction::NorthEast),
            _ => Err(()),
        }
    }
}

fn parse_directions(i: &str) -> IResult<&str, Vec<Direction>> {
    many1(map_res(
        alt((
            tag("e"),
            tag("se"),
            tag("sw"),
            tag("w"),
            tag("nw"),
            tag("ne"),
        )),
        |s: &str| s.parse(),
    ))(i)
}

fn parse_input() -> Vec<Vec<Direction>> {
    let file = File::open("src/2020/day_24.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| parse_directions(&line.unwrap()).unwrap().1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_directions_works() {
        let (text, directions) = parse_directions("nwwswee").unwrap();

        assert_eq!("", text);
        assert_eq!(vec![NorthWest, West, SouthWest, East, East], directions);
    }
}
