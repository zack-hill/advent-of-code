use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_part_1(input: &Vec<Vec<bool>>) -> usize {
    return solve(3, 1, input);
}

pub fn solve_part_2(input: &Vec<Vec<bool>>) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    return slopes.iter().map(|x| solve(x.0, x.1, input)).product();
}

fn solve(slope_x: usize, slope_y: usize, map_data: &Vec<Vec<bool>>) -> usize {
    let mut x = slope_x;
    let mut y = slope_y;
    let mut tree_count = 0;

    while y < map_data.len() {
        let row = &map_data[y];

        if row[x] {
            tree_count += 1;
        }

        // Use modulus to wrap x, simulating the map repeating infinitely horizontally
        x = (x + slope_x) % row.len();
        y += slope_y;
    }
    return tree_count;
}

pub fn parse_input() -> Vec<Vec<bool>> {
    let file = File::open("src/day_03.txt").unwrap();
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
