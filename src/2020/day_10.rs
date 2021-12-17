use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_puzzle_1() -> u64 {
    let mut values = parse_file();
    sort_and_cap(&mut values);
    let counts = get_counts(values.windows(2).map(|w| w[1] - w[0]));
    return counts[&1u64] * counts[&3u64];
}

pub fn solve_puzzle_2() -> u64 {
    let mut values = parse_file();
    sort_and_cap(&mut values);
    return count_paths(&values);
}

fn get_counts<K: Ord, I: IntoIterator<Item = K>>(iter: I) -> BTreeMap<K, u64> {
    let mut counter: BTreeMap<K, u64> = BTreeMap::new();
    for key in iter {
        if let Some(value) = counter.get_mut(&key) {
            *value += 1;
        } else {
            counter.insert(key, 1);
        }
    }
    return counter;
}

fn count_paths(values: &Vec<u64>) -> u64 {
    let range = 3;
    let count = values.len();
    // Create a new vector to hold the path counts of each value and
    // initialize each value to zero except for the last one, which
    // is set to one.
    let mut path_counts = Vec::<u64>::new();
    path_counts.resize_with(count, Default::default);
    path_counts[count - 1] = 1;
    // Iterate over the values in reverse order. The number of possible
    // paths to the end for a value is equal to the sum of counts for
    // the values within range.
    for i in (0..count - 1).rev() {
        path_counts[i] = ((i + 1)..=(i + range))
            .filter(|j| *j < count) // Prevent overflow
            .filter(|j| values[*j] - values[i] <= range as u64) // Check if value is in range
            .map(|j| path_counts[j])
            .sum();
    }
    return path_counts[0];
}

fn sort_and_cap(values: &mut Vec<u64>) {
    values.push(0); // Add the zero jolts value
    values.sort();
    let adapter_value = values.last().unwrap() + 3u64;
    values.push(adapter_value); // Add a value representing the adapter jolts
}

fn parse_file() -> Vec<u64> {
    let file = File::open("src/day_10.txt").unwrap();
    let reader = BufReader::new(file);
    let values: Vec<u64> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    return values;
}
