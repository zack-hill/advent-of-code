use std::collections::VecDeque;

pub fn solve_puzzle_1() -> String {
    let cups = get_cups();
    let mut linked_list = convert_to_linked_list(&cups);
    move_cups(100, &mut linked_list);
    return "".to_string();
    // while cups.front().unwrap() != &1 {
    //     cups.rotate_left(1);
    // }
    // return cups
    //     .iter()
    //     .skip(1)
    //     .map(|c| c.to_string())
    //     .collect::<Vec<String>>()
    //     .concat();
}

// pub fn solve_puzzle_2() -> u32 {
//     let mut cups = get_cups();

//     // Add remaining cups up through one million
//     let max = *cups.iter().max().unwrap();
//     cups.extend(max + 1..=1_000_000);

//     move_cups(1000, &mut cups);

//     let one_index = cups.iter().position(|&c| c == 1).unwrap();
//     return cups[one_index + 1] * cups[one_index + 2];
// }

fn move_cups(move_count: u32, cups: &mut Vec<usize>) {
    let mut value = 0;
    for _ in 0..move_count {
        // println!("-- Turn {} --", turn + 1);
        // println!("cups: {:?}", cups);
        // let value = *cups.front().unwrap();
        // println!("current: {}", value);
        // cups.rotate_left(1);
        // let cups_to_move = get_n_cups(3, cups);
        // cups.rotate_right(1);
        // println!("pick up: {:?}", cups_to_move);
        // let destination = find_insert_index(value, &cups);
        // cups.rotate_left(destination);
        // println!("destination: {}", destination);
        // for cup in cups_to_move.into_iter().rev() {
        // cups.push_front(cup);
        // cups.insert(destination, cup);
        // }
        // cups.rotate_right(destination);
        // println!("");
    }
    // println!("final: {:?}", cups);
}

fn get_cups() -> Vec<usize> {
    // test
    // vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect()
    // prod
    vec![2, 8, 4, 5, 7, 3, 9, 6, 1].into_iter().collect()
}

fn convert_to_linked_list(cups: &Vec<usize>) -> Vec<usize> {
    let mut linked_list = Vec::new();
    linked_list.resize(cups.len(), 0);
    for test in cups.windows(2) {
        linked_list[test[0] - 1] = test[1];
    }
    return linked_list;
}

fn convert_from_linked_list(linked_list: &Vec<usize>) -> Vec<usize> {
    let mut vec = Vec::new();
    vec.resize(linked_list.len(), 0);
    let mut num = linked_list[0];
    for i in 0..linked_list.len() {
        let tmp = linked_list[num];
        vec[i] = tmp;
        num = tmp;
    }
    return vec;
}

// fn get_n_cups(n: usize, cups: &mut Vec<u32>) -> Vec<u32> {
//     (0..n).map(|_| cups.pop_front().unwrap()).collect()
// }

fn find_insert_index(num: u32, cups: &Vec<u32>) -> usize {
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
    // fn get_n_cups() {
    //     let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect();
    //     let cups_to_move = super::get_n_cups(3, &mut cups);
    //     assert_eq!(vec![3, 8, 9], cups_to_move);
    //     assert_eq!(vec![1, 2, 5, 4, 6, 7], cups.iter().collect::<Vec<u32>>());
    // }

    #[test]
    fn convert_to_linked_list() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

        let cups_to_move = super::convert_to_linked_list(&cups);

        assert_eq!(vec![2, 5, 8, 6, 4, 7, 0, 9, 1], cups_to_move);
    }

    #[test]
    fn convert_from_linked_list() {
        let expected = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

        let linked_list = super::convert_to_linked_list(&expected);
        let actual = super::convert_from_linked_list(&linked_list);

        assert_eq!(expected, actual);
    }

    #[test]
    fn move_cups_example() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut linked_list = super::convert_to_linked_list(&cups);

        move_cups(10, &mut linked_list);

        // let result = linked_list.iter().map(|x| linked_list[x]);

        assert_eq!(vec![8, 3, 7, 4, 1, 9, 2, 6, 5], linked_list);
    }

    // #[test]
    // fn solve_puzzle_1() {
    //     assert_eq!("26354798", super::solve_puzzle_1());
    // }

    // #[test]
    // fn solve_puzzle_2() {
    //     assert_eq!(33, super::solve_puzzle_2());
    // }
}
