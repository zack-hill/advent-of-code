use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_part_1(input: &Vec<u32>) -> u32 {
    return solve(input, 2, 2020);
}

pub fn solve_part_2(input: &Vec<u32>) -> u32 {
    return solve(input, 3, 2020);
}

fn solve(numbers: &Vec<u32>, n: usize, target: u32) -> u32 {
    // Initialize a vector of length n with zeros for the values
    let mut indices = vec![0; n];

    // Loop over all combinations of indices
    while indices[0] < numbers.len() {
        // Get the set of numbers for this combination
        let nums: Vec<u32> = indices.iter().map(|i| numbers[*i]).collect();

        // Return the product of the numbers if the sum of the numbers matches the target
        let sum: u32 = nums.iter().sum();
        if sum == target {
            return nums.iter().product();
        }

        // Increment the indices to produce the next unique set
        for i in (0..n).rev() {
            if indices[i] + 1 == numbers.len() {
                if i == 0 {
                    // There are no more indices to increment
                    break;
                }
                // The index has reached its max value, wrap it back to zero
                indices[i] = 0;
            } else {
                // We have found an index to increment
                indices[i] += 1;
                break;
            }
        }
    }
    unreachable!()
}

pub fn parse_input() -> Vec<u32> {
    let file = File::open("src/day_01.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
}
