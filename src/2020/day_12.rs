use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> i64 {
    let mut map = HashMap::<i64, (i64, i64)>::new();
    map.insert(0, (1, 0));
    map.insert(90, (0, 1));
    map.insert(180, (-1, 0));
    map.insert(270, (0, -1));

    let instructions = parse_file();
    let mut x = 0;
    let mut y = 0;
    let mut heading = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Move(dx, dy) => {
                x += dx;
                y += dy;
            }
            Instruction::MoveForward(amount) => {
                let (x_dir, y_dir) = map[&heading];
                x += amount * x_dir;
                y += amount * y_dir;
            }
            Instruction::Rotate(amount) => {
                heading = (((heading + amount) % 360) + 360) % 360;
            }
            _ => {}
        };
        // println!("({}, {})", x, y);
    }
    return calc_manhattan_dist(x, y);
}

pub fn solve_puzzle_2() -> i64 {
    let instructions = parse_file();
    let (mut x, mut y) = (0, 0);
    let (mut w_x, mut w_y) = (10, 1);

    for instruction in instructions {
        match instruction {
            Instruction::Move(dx, dy) => {
                w_x += dx;
                w_y += dy;
            }
            Instruction::MoveForward(amount) => {
                x += amount * w_x;
                y += amount * w_y;
            }
            Instruction::Rotate(amount) => {
                let (n_x, n_y) = rotate_point(w_x, w_y, amount as f64, 0, 0);
                w_x = n_x;
                w_y = n_y;
            }
            _ => {}
        };
        // println!("Ship ({}, {}) | Waypoint ({}, {})", x, y, w_x, w_y);
    }
    return calc_manhattan_dist(x, y);
}

fn rotate_point(px: i64, py: i64, theta: f64, cx: i64, cy: i64) -> (i64, i64) {
    let theta = theta.to_radians();
    return (
        theta.cos() as i64 * (px - cx) - theta.sin() as i64 * (py - cy) + cx,
        theta.sin() as i64 * (px - cx) + theta.cos() as i64 * (py - cy) + cy,
    );
}

fn calc_manhattan_dist(x: i64, y: i64) -> i64 {
    x.abs() + y.abs()
}

enum Instruction {
    Move(i64, i64),
    MoveForward(i64),
    Rotate(i64),
}

fn parse_file() -> Vec<Instruction> {
    let file = File::open("src/day_12.txt").unwrap();
    let reader = BufReader::new(file);
    let mut instructions = Vec::<Instruction>::new();
    let mut map = HashMap::<&str, (i64, i64)>::new();
    map.insert("N", (0, 1));
    map.insert("S", (0, -1));
    map.insert("E", (1, 0));
    map.insert("W", (-1, 0));

    for line in reader.lines() {
        let line = line.unwrap();
        let (letter, value) = line.split_at(1);
        let mut value = value.parse::<i64>().unwrap();

        let instruction = match letter {
            "L" => Instruction::Rotate(value),
            "R" => Instruction::Rotate(-value), // Inverting the rotation value for right handed turns encodes the direction into the value
            "F" => Instruction::MoveForward(value),
            _ => Instruction::Move(value * map[letter].0, value * map[letter].1),
        };
        instructions.push(instruction);
    }
    return instructions;
}
