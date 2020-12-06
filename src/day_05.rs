use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> usize {
    let boarding_passes = parse_file("src/day_05.txt");
    return boarding_passes
        .iter()
        .map(|x| convert_to_seat_position(x))
        .map(|x| calculate_seat_id(x))
        .max()
        .unwrap();
}

pub fn solve_puzzle_2() -> usize {
    let boarding_passes = parse_file("src/day_05.txt");

    let filled_seats: HashSet<_> = boarding_passes
        .iter()
        .map(|x| convert_to_seat_position(x))
        .map(|x| calculate_seat_id(x))
        .collect();

    // Iterate over possible seat id range to find our seat
    for i in 1..1023 {
        let prev = filled_seats.contains(&(i - 1));
        let curr = filled_seats.contains(&i);
        let next = filled_seats.contains(&(i + 1));
        if prev && !curr && next {
            return i;
        }
    }

    return 0;
}

fn convert_to_seat_position(boarding_pass: &str) -> (usize, usize) {
    let (row_text, col_text) = boarding_pass.split_at(7);
    let row = partition(row_text, 'F', 'B');
    let col = partition(col_text, 'L', 'R');
    return (row, col);
}

fn calculate_seat_id(seat_position: (usize, usize)) -> usize {
    return seat_position.0 * 8 + seat_position.1;
}

fn partition(text: &str, left_char: char, right_char: char) -> usize {
    let mut left = 0;
    let mut right = 2usize.pow(text.len() as u32) - 1; // Calculate the size of the space using 2^length of text
    for character in text.chars() {
        let mid: f32 = ((right - left) as f32 / 2.0) + left as f32;
        if character == left_char {
            // We are keeping the left half, move the right side in
            right = mid.floor() as usize;
        } else if character == right_char {
            // We are keeping the right half, move the left side in
            left = mid.ceil() as usize;
        }
    }
    return left;
}

fn parse_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut boarding_passes = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().to_owned();
        boarding_passes.push(line);
    }

    return boarding_passes;
}
