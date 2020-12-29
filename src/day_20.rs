use std::{collections::HashMap, fs::File};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

pub fn solve_puzzle_1() -> u64 {
    let tiles = parse_file();
    let tiles_grid = match_tiles(tiles);

    let first_row = tiles_grid.first().unwrap();
    let last_row = tiles_grid.last().unwrap();

    // Return the product of the four corners.
    return first_row.first().unwrap().number
        * first_row.last().unwrap().number
        * last_row.first().unwrap().number
        * last_row.last().unwrap().number;
}

pub fn solve_puzzle_2() -> u64 {
    let tiles = parse_file();
    return 0;
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    fn opposite(&self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Clone)]
struct Tile {
    number: u64,
    data: Vec<Vec<bool>>,
}

impl Tile {
    fn get_side(&self, side: &Side) -> Vec<bool> {
        match side {
            Side::Top => self.data[0].clone(),
            Side::Bottom => self.data[self.data.len() - 1].clone(),
            Side::Left => self.data.iter().map(|row| row[0]).collect(),
            Side::Right => self
                .data
                .iter()
                .map(|row| row[self.data.len() - 1])
                .collect(),
        }
    }

    fn flip(&mut self, flip_x: bool, flip_y: bool) {
        let dim = self.data.len();
        let mut flipped = self.data.clone();
        for y in 0..dim {
            for x in 0..dim {
                let source_x = if flip_x { dim - 1 - x } else { x };
                let source_y = if flip_y { dim - 1 - y } else { y };
                flipped[y][x] = self.data[source_y][source_x];
            }
        }
        self.data = flipped;
    }

    fn rotate_cw(&mut self) {
        self.rotate(-90.0);
    }

    fn rotate_ccw(&mut self) {
        self.rotate(90.0);
    }

    fn rotate(&mut self, theta: f32) {
        let dim = self.data.len();
        let center = (dim - 1) as f32 / 2.0;
        let theta = theta.to_radians();
        let sin = theta.sin();
        let cos = theta.cos();

        let mut rotated = self.data.clone();
        for y in 0..dim {
            for x in 0..dim {
                let source_x = cos * (x as f32 - center) - sin * (y as f32 - center) + center;
                let source_y = sin * (x as f32 - center) + cos * (y as f32 - center) + center;
                // Round the source coordinates to the nearest integer
                let source_x = (source_x + 0.5).floor() as usize;
                let source_y = (source_y + 0.5).floor() as usize;
                // println!("({}, {}) -> ({}, {})", source_x, source_y, x, y);
                rotated[y][x] = self.data[source_y][source_x];
            }
        }
        self.data = rotated;
    }
}

fn match_tiles(tiles: Vec<Tile>) -> Vec<Vec<Tile>> {
    let mut tiles: HashMap<_, _> = tiles.into_iter().map(|tile| (tile.number, tile)).collect();
    // A queue is used here because a given tile might not have any matching
    // sides at a given step, but by cycling through each unsolved tile we can
    // try all possibilities.
    let mut unsolved_tiles = VecDeque::<u64>::new();
    let mut grid = HashMap::<(i32, i32), u64>::new();

    let directions: HashMap<Side, (i32, i32)> = vec![
        (Side::Right, (1, 0)),
        (Side::Left, (-1, 0)),
        (Side::Top, (0, 1)),
        (Side::Bottom, (0, -1)),
    ]
    .into_iter()
    .collect();

    // Initialize all tiles as unsolved
    for tile in tiles.values() {
        unsolved_tiles.push_back(tile.number);
    }

    // Iterate through the unsolved tiles and try to find a spot in the grid
    // where the sides match up. If a spot cannot be found, the tile is added
    // to the back of the unsolved tiles queue.
    while let Some(tile) = unsolved_tiles.pop_front() {
        // Put the first tile in the center of the grid to start.
        if grid.is_empty() {
            grid.insert((0, 0), tile);
            continue;
        }

        let mut found_match = false;

        // Loop over the solved tiles to see if the current tile fits on
        // any adjacent side.
        let solved_tiles: Vec<(i32, i32)> = grid.keys().cloned().collect();
        for (tile_x, tile_y) in solved_tiles {
            // Check each adjacent tile to the solved tile for an empty space
            for (dir_x, dir_y) in directions.values() {
                let potential_tile = (tile_x + dir_x, tile_y + dir_y);
                // Skip tiles that are already filled
                if grid.contains_key(&potential_tile) {
                    continue;
                }

                // Update tile coordinates to our current potential tile location
                let (tile_x, tile_y) = potential_tile;

                // Get a set of sides that this tile needs to match up with to
                // be valid for this spot
                let sides: Vec<(&Side, Vec<bool>)> = directions
                    .iter()
                    .map(|(side, (dir_x, dir_y))| {
                        (side, grid.get(&(tile_x + dir_x, tile_y + dir_y)))
                    })
                    .filter(|(_, tile_index)| tile_index.is_some())
                    .map(|(side, tile_index)| {
                        (side, tiles[tile_index.unwrap()].get_side(&side.opposite()))
                    })
                    .collect();

                // Flip and rotate the tile checking the given sides to test for a valid fit
                if fit_tile(tiles.get_mut(&tile).unwrap(), &sides) {
                    grid.insert(potential_tile, tile);
                    found_match = true;
                    break;
                }
            }
            if found_match {
                break;
            }
        }
        if !found_match {
            unsolved_tiles.push_back(tile);
        }
    }

    // Get the four corners of the grid
    let min_x = *grid.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *grid.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    // Convert from a HashMap of tiles to a 2D vector
    let mut tile_grid = Vec::<Vec<Tile>>::new();
    for y in min_y..=max_y {
        let mut row = Vec::<Tile>::new();
        for x in min_x..=max_x {
            row.push(tiles[&grid[&(x, y)]].clone())
        }
        tile_grid.push(row);
    }

    return tile_grid;
}

/// Flips and rotates a tile, checking each possible orientation to see if
/// the sides of the line match the given set of sides.
/// Returns true if a matching orientation if found.
fn fit_tile(tile: &mut Tile, sides: &Vec<(&Side, Vec<bool>)>) -> bool {
    let mutations: Vec<fn(&mut Tile)> = vec![
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.flip(true, false),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.flip(false, true),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.flip(true, false),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
        |t| t.rotate_cw(),
    ];
    for mutator in mutations {
        mutator(tile);
        if check_tile(tile, sides) {
            return true;
        }
    }

    return false;
}

/// Compares the sides of a tile against the given set of sides.
/// Returns true if the given set of sides matches the sides of the tile.
fn check_tile(tile: &Tile, sides: &Vec<(&Side, Vec<bool>)>) -> bool {
    sides
        .iter()
        .all(|(side, values)| &tile.get_side(side) == values)
}

fn parse_file() -> Vec<Tile> {
    let file = File::open("src/day_20.txt").unwrap();
    let reader = BufReader::new(file);

    let mut tiles = Vec::<Tile>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("Tile") {
            let tile = Tile {
                number: line[5..=8].parse().unwrap(),
                data: Vec::<Vec<bool>>::new(),
            };
            tiles.push(tile);
        } else if line == "" {
            continue;
        } else {
            let tile = tiles.last_mut().unwrap();
            tile.data.push(line.chars().map(|c| c == '#').collect());
        }
    }

    return tiles;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tile() -> Tile {
        let data = vec![
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![true, false, false, false],
            vec![true, true, true, true],
        ];
        Tile { number: 0, data }
    }

    #[test]
    fn get_top_side() {
        let tile = create_test_tile();
        let expected = vec![false, false, true, true];

        let actual = tile.get_side(&Side::Top);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_bottom_side() {
        let tile = create_test_tile();
        let expected = vec![true, true, true, true];

        let actual = tile.get_side(&Side::Bottom);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_left_side() {
        let tile = create_test_tile();
        let expected = vec![false, true, true, true];

        let actual = tile.get_side(&Side::Left);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_right_side() {
        let tile = create_test_tile();
        let expected = vec![true, false, false, true];

        let actual = tile.get_side(&Side::Right);

        assert_eq!(expected, actual);
    }

    #[test]
    fn flip_x() {
        let mut tile = create_test_tile();
        let expected = vec![
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![false, false, false, true],
            vec![true, true, true, true],
        ];

        tile.flip(true, false);

        assert_eq!(expected, tile.data);
    }

    #[test]
    fn flip_y() {
        let mut tile = create_test_tile();
        let expected = vec![
            vec![true, true, true, true],
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, false, true, true],
        ];

        tile.flip(false, true);

        assert_eq!(expected, tile.data);
    }

    #[test]
    fn rotate_cw() {
        let mut tile = create_test_tile();

        let expected = vec![
            vec![true, true, true, false],
            vec![true, false, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
        ];

        tile.rotate_cw();

        assert_eq!(expected, tile.data);
    }

    #[test]
    fn rotate_ccw() {
        let mut tile = create_test_tile();

        let expected = vec![
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![false, true, false, true],
            vec![false, true, true, true],
        ];

        tile.rotate_ccw();

        assert_eq!(expected, tile.data);
    }
}
