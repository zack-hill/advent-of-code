use std::{
    cmp::min,
    collections::{LinkedList, VecDeque},
};

pub fn solve_puzzle_1() -> String {
    let mut cups = get_cups();
    move_cups(100, &mut cups);
    let one_index = cups.iter().position(|&c| c == 1).unwrap();
    let start = min(one_index + 1, cups.len() - 1);
    let end = one_index;
    cups[start..cups.len()]
        .iter()
        .chain(cups[0..end].iter())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .concat()
}

pub fn solve_puzzle_2() -> u32 {
    let mut cups = get_cups();
    let max = *cups.iter().max().unwrap();
    cups.extend(max..1_000_000);
    move_cups(1000, &mut cups);
    let one_index = cups.iter().position(|&c| c == 1).unwrap();
    return cups[one_index + 1] * cups[one_index + 2];
}

fn move_cups(move_count: u32, cups: &mut VecDeque<u32>) {
    let mut index = 0;
    for turn in 0..move_count {
        // println!("-- Turn {} --", turn);
        // println!("cups: {:?}", cups);
        let value = cups[index];
        // println!("current: {}", value);
        let cups_to_move = get_n_cups(3, index + 1, cups);
        // println!("pick up: {:?}", cups_to_move);
        index = cups.iter().position(|&c| c == value).unwrap();
        let destination = find_insert_index(cups[index], &cups);
        // println!("destination: {}", destination);
        for cup in cups_to_move.into_iter().rev() {
            cups.insert(destination, cup);
        }
        index = cups.iter().position(|&c| c == value).unwrap();
        // index += 1;
        // if index >= cups.len() - 1 {
        //     index = 0;
        // }
        index = (index + 1) % cups.len();
        // println!("");
    }
}

fn get_cups() -> VecDeque<u32> {
    // test
    // vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect()
    // prod
    vec![2, 8, 4, 5, 7, 3, 9, 6, 1].into_iter().collect()
}

fn get_n_cups(n: usize, index: usize, cups: &mut VecDeque<u32>) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    let mut index = index;
    for _ in 0..n {
        if index >= cups.len() {
            index = 0;
        }
        result.push(cups.remove(index));
    }
    return result;
}

fn find_insert_index(num: u32, cups: &VecDeque<u32>) -> usize {
    return cups
        .iter()
        .enumerate()
        .filter(|&(_, cup)| *cup < num)
        .max_by_key(|&(_, cup)| cup)
        .or(cups.iter().enumerate().max_by_key(|&(_, cup)| cup))
        .unwrap()
        .0
        + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn get_n_cups() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect();
        let cups_to_move = super::get_n_cups(3, 7, &mut cups);

        assert_eq!(vec![6, 7, 3], cups_to_move);
        assert_eq!(vec![8, 9, 1, 2, 5, 4], cups.iter().collect::<Vec<u32>>());
    }

    #[test]
    fn solve_puzzle_1() {
        assert_eq!("26354798", super::solve_puzzle_1());
    }

    #[test]
    fn solve_puzzle_2() {
        assert_eq!(30, super::solve_puzzle_2());
    }
}
