use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::{self},
};

use crate::solver::AoCSolver;

type Position = (usize, usize);

pub struct Solver {
    codes: Vec<CodeInput>,
}

#[derive(Debug, Clone)]
enum NumericKeyPadOption {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
}

impl NumericKeyPadOption {
    fn from_char(char: char) -> Self {
        match char {
            '0' => NumericKeyPadOption::Num0,
            '1' => NumericKeyPadOption::Num1,
            '2' => NumericKeyPadOption::Num2,
            '3' => NumericKeyPadOption::Num3,
            '4' => NumericKeyPadOption::Num4,
            '5' => NumericKeyPadOption::Num5,
            '6' => NumericKeyPadOption::Num6,
            '7' => NumericKeyPadOption::Num7,
            '8' => NumericKeyPadOption::Num8,
            '9' => NumericKeyPadOption::Num9,
            'A' => NumericKeyPadOption::A,
            _ => panic!("Invalid Input!"),
        }
    }

    pub fn press(position: Position) -> Option<NumericKeyPadOption> {
        match position {
            (1, 0) => Some(NumericKeyPadOption::Num0),
            (2, 0) => Some(NumericKeyPadOption::A),
            (0, 1) => Some(NumericKeyPadOption::Num1),
            (1, 1) => Some(NumericKeyPadOption::Num2),
            (2, 1) => Some(NumericKeyPadOption::Num3),
            (0, 2) => Some(NumericKeyPadOption::Num4),
            (1, 2) => Some(NumericKeyPadOption::Num5),
            (2, 2) => Some(NumericKeyPadOption::Num6),
            (0, 3) => Some(NumericKeyPadOption::Num7),
            (1, 3) => Some(NumericKeyPadOption::Num8),
            (2, 3) => Some(NumericKeyPadOption::Num9),
            _ => None,
        }
    }

    pub fn get_position(&self) -> Position {
        match self {
            NumericKeyPadOption::Num0 => (1, 0),
            NumericKeyPadOption::A => (2, 0),
            NumericKeyPadOption::Num1 => (0, 1),
            NumericKeyPadOption::Num2 => (1, 1),
            NumericKeyPadOption::Num3 => (2, 1),
            NumericKeyPadOption::Num4 => (0, 2),
            NumericKeyPadOption::Num5 => (1, 2),
            NumericKeyPadOption::Num6 => (2, 2),
            NumericKeyPadOption::Num7 => (0, 3),
            NumericKeyPadOption::Num8 => (1, 3),
            NumericKeyPadOption::Num9 => (2, 3),
        }
    }
}

#[derive(Debug, Clone)]
enum DirectionalKeyPadOption {
    Left,
    Down,
    Up,
    Right,
    A,
}

impl DirectionalKeyPadOption {
    fn to_char(&self) -> char {
        match self {
            DirectionalKeyPadOption::Left => '<',
            DirectionalKeyPadOption::Down => 'v',
            DirectionalKeyPadOption::Up => '^',
            DirectionalKeyPadOption::Right => '>',
            DirectionalKeyPadOption::A => 'A',
        }
    }

    fn from_char(char: char) -> Self {
        match char {
            '<' => DirectionalKeyPadOption::Left,
            'v' => DirectionalKeyPadOption::Down,
            '^' => DirectionalKeyPadOption::Up,
            '>' => DirectionalKeyPadOption::Right,
            'A' => DirectionalKeyPadOption::A,
            _ => panic!("Invalid Input!"),
        }
    }

    fn press(position: Position) -> Option<DirectionalKeyPadOption> {
        match position {
            (0, 0) => Some(DirectionalKeyPadOption::Left),
            (1, 0) => Some(DirectionalKeyPadOption::Down),
            (2, 0) => Some(DirectionalKeyPadOption::Right),
            (1, 1) => Some(DirectionalKeyPadOption::Up),
            (2, 1) => Some(DirectionalKeyPadOption::A),
            _ => None,
        }
    }

    fn get_position(&self) -> Position {
        match self {
            DirectionalKeyPadOption::Left => (0, 0),
            DirectionalKeyPadOption::Down => (1, 0),
            DirectionalKeyPadOption::Right => (2, 0),
            DirectionalKeyPadOption::Up => (1, 1),
            DirectionalKeyPadOption::A => (2, 1),
        }
    }
}

fn get_directions(
    (x1, y1): Position,
    (x2, y2): Position,
    (gx, gy): Position,
) -> impl Iterator<Item = DirectionalKeyPadOption> {
    let dx = (x2 as i32) - (x1 as i32);
    let dy = (y2 as i32) - (y1 as i32);
    let dx_abs = dx.abs() as usize;
    let dy_abs = dy.abs() as usize;

    let left_count = if dx < 0 { dx_abs } else { 0 };
    let right_count = if dx > 0 { dx_abs } else { 0 };
    let down_count = if dy < 0 { dy_abs } else { 0 };
    let up_count = if dy > 0 { dy_abs } else { 0 };

    let crosses_gap = (left_count >= 1 && gx < x1 && gx >= x2 && gy == y1)
        || (right_count >= 1 && gx > x1 && gx <= x2 && gy == y1)
        || (down_count >= 1 && gy < y1 && gy >= y2 && gx == x1)
        || (up_count >= 1 && gy > y1 && gy <= y2 && gx == x1);

    if crosses_gap {
        return iter::repeat_n(DirectionalKeyPadOption::Right, right_count)
            .chain(iter::repeat_n(DirectionalKeyPadOption::Down, down_count))
            .chain(iter::repeat_n(DirectionalKeyPadOption::Up, up_count))
            .chain(iter::repeat_n(DirectionalKeyPadOption::Left, left_count));
    } else {
        return iter::repeat_n(DirectionalKeyPadOption::Left, left_count)
            .chain(iter::repeat_n(DirectionalKeyPadOption::Up, up_count))
            .chain(iter::repeat_n(DirectionalKeyPadOption::Down, down_count))
            .chain(iter::repeat_n(DirectionalKeyPadOption::Right, right_count));
    }
}

fn expand(options: &Vec<DirectionalKeyPadOption>) -> Vec<DirectionalKeyPadOption> {
    let mut pos = (2, 1);
    let mut expanded_inputs: Vec<DirectionalKeyPadOption> = Vec::new();
    for option in options.iter() {
        let target = option.get_position();
        let inputs = get_directions(pos, target, (0, 1));
        // println!("{:?} -> {:?}", pos, target);
        pos = target;
        expanded_inputs.extend(inputs);
        expanded_inputs.push(DirectionalKeyPadOption::A);
    }
    // println!("expanded: {:?}", expanded_inputs);
    return expanded_inputs;
}

fn offset((x, y): &Position, option: &DirectionalKeyPadOption) -> Position {
    // println!("({}, {}) | {:?}", x, y, option);
    match option {
        DirectionalKeyPadOption::Left => (x - 1, *y),
        DirectionalKeyPadOption::Down => (*x, y - 1),
        DirectionalKeyPadOption::Up => (*x, y + 1),
        DirectionalKeyPadOption::Right => (x + 1, *y),
        _ => panic!("Invalid Input!"),
    }
}

fn compress(
    start: &Position,
    options: &Vec<DirectionalKeyPadOption>,
) -> (Position, Vec<DirectionalKeyPadOption>) {
    let mut pos = *start;
    let mut compressed_inputs: Vec<DirectionalKeyPadOption> = Vec::new();
    for option in options.iter() {
        match option {
            DirectionalKeyPadOption::A => {
                compressed_inputs.push(DirectionalKeyPadOption::press(pos).unwrap())
            }
            direction => pos = offset(&pos, direction),
        }
    }
    // println!("expanded: {:?}", expanded_inputs);
    return (pos, compressed_inputs);
}

fn apply(
    start: &Position,
    options: &Vec<DirectionalKeyPadOption>,
) -> (Position, Vec<NumericKeyPadOption>) {
    let mut pos = *start;
    let mut outputs: Vec<NumericKeyPadOption> = Vec::new();
    for option in options.iter() {
        match option {
            DirectionalKeyPadOption::A => outputs.push(NumericKeyPadOption::press(pos).unwrap()),
            direction => pos = offset(&pos, direction),
        }
    }
    // println!("expanded: {:?}", expanded_inputs);
    return (pos, outputs);
}

fn find_required_inputs(code: &CodeInput, robot_count: usize) -> usize {
    println!("Code: {:?}", code);

    let mut numeric_keypad_position: Position = (2, 0);

    // let mut human_inputs: Vec<DirectionalKeyPadOption> = Vec::new();
    let mut input_count = 0;

    for keypad_option in code.keypad_options.iter() {
        let numeric_keypad_target = keypad_option.get_position();
        println!("Steps for {:?}", keypad_option);
        // println!("l0: {:?} -> {:?}", p0, t0);

        let mut first_directional_keypad_inputs: Vec<DirectionalKeyPadOption> =
            get_directions(numeric_keypad_position, numeric_keypad_target, (0, 0)).collect();
        first_directional_keypad_inputs.push(DirectionalKeyPadOption::A);
        // println!("l1_inputs: {:?}", l1_inputs);

        numeric_keypad_position = numeric_keypad_target;

        let mut last_layer_inputs = first_directional_keypad_inputs;
        for i in 0..robot_count - 1 {
            last_layer_inputs = expand(&last_layer_inputs);
            // println!("l{}_inputs: {:?}", i, last_layer_inputs);
            println!(
                "Evaluated Layer {}. Input Length = {}",
                i,
                last_layer_inputs.len()
            );
        }
        input_count += last_layer_inputs.len();
        // human_inputs.extend(last_layer_inputs);
        // println!("l3_inputs: {:?}", l3_inputs);
    }

    // println!(
    //     "{:3}A: {}",
    //     code.code_value,
    //     human_inputs.iter().map(|i| i.to_char()).collect::<String>()
    // );
    // println!("human_inputs: {:?}", human_inputs);
    // println!("human_inputs len: {:?}", human_inputs.len());

    // let (_, a) = compress(&(2, 1), &human_inputs);
    // println!("output l2_inputs: {:?}", a);
    // let (_, b) = compress(&(2, 1), &a);
    // println!("output l1_inputs: {:?}", b);
    // let (_, c) = apply(&(2, 0), &b);
    // println!("output: {:?}", c);

    println!("{} * {}", input_count, code.code_value);
    return input_count * code.code_value;
    // return human_inputs;
}

fn calculate_complexity(code: &CodeInput, inputs: &Vec<DirectionalKeyPadOption>) -> usize {
    inputs.len() * code.code_value
}

impl Solver {
    pub fn create() -> Self {
        let codes = parse_input();
        Solver { codes }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut sum: usize = 0;
        for code in self.codes.iter() {
            let complexity = find_required_inputs(code, 3);
            // let complexity = calculate_complexity(code, &inputs);
            println!("{}: {}", code.code_value, complexity);
            println!("");
            sum += complexity;
        }

        return sum.to_string();
    }

    fn solve_part_2(&self) -> String {
        // return "".to_string();
        let mut sum: usize = 0;
        for code in self.codes.iter() {
            let complexity = find_required_inputs(code, 26);
            // let complexity = calculate_complexity(code, &inputs);
            // println!("complexity: {:?}", complexity);
            // println!("");
            println!("{}: {}", code.code_value, complexity);
            sum += complexity;
        }

        return sum.to_string();
    }
}

#[derive(Debug)]
struct CodeInput {
    keypad_options: Vec<NumericKeyPadOption>,
    code_value: usize,
}

fn parse_input() -> Vec<CodeInput> {
    let file = File::open("src/2024/day_21.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let keypad_options = line
                .chars()
                .map(|c| NumericKeyPadOption::from_char(c))
                .collect();
            let code_value: usize = line
                .chars()
                .take(3)
                .collect::<String>()
                .parse()
                .expect("Invalid Input");
            CodeInput {
                keypad_options,
                code_value,
            }
        })
        .collect();
    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupied_seats_none_found() {
        let options = vec![
            DirectionalKeyPadOption::Left,
            DirectionalKeyPadOption::A,
            DirectionalKeyPadOption::Left,
            DirectionalKeyPadOption::A,
            DirectionalKeyPadOption::Left,
            DirectionalKeyPadOption::Left,
            DirectionalKeyPadOption::A,
        ];
        let expanded = expand(&options);
        println!("{:?}", expanded)
    }
}
