use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    return solve(3, 1);
}

pub fn solve_puzzle_2() -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    return slopes.iter().map(|x| solve(x.0, x.1)).product();
}

fn solve(slope_x: usize, slope_y: usize) -> usize {
    let map = parse_file("src/day_03.txt");
    let mut x = slope_x;
    let mut y = slope_y;
    let mut tree_count = 0;

    while y < map.len() {
        let row = &map[y];

        if row[x] {
            tree_count += 1;
        }
        
        // Use modulus to wrap x, simulating the map repeating infinitely horizontally
        x = (x + slope_x) % row.len();
        y += slope_y;
    }
    return tree_count;
}

fn parse_file(path: &str) -> Vec<Vec<bool>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Store the tree data as a 2d array of bools where true represents a tree
    let mut data = Vec::<Vec<bool>>::new();
    for line in reader.lines() {
        let mut row = Vec::<bool>::new();

        let line: &str = &line.unwrap();
        for character in line.chars() {
            row.push(match character {
                '#' => true,
                '.' => false,
                _ => panic!(),
            });
        }

        data.push(row);
    }

    return data;
}
