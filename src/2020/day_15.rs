use std::collections::HashMap;

pub fn solve_puzzle_1() -> u32 {
    let numbers = get_numbers();
    return get_number_at_turn(2020, &numbers);
}

pub fn solve_puzzle_2() -> u32 {
    let numbers = get_numbers();
    return get_number_at_turn(30000000, &numbers);
}

fn get_number_at_turn(target_turn: u32, numbers: &Vec<u32>) -> u32 {
    let mut history = HashMap::<u32, Vec<u32>>::new();
    let mut turn = 1;
    let mut last_number = 0;
    for &number in numbers {
        history.insert(number, vec![turn]);
        last_number = number;
        turn += 1;
    }
    loop {
        // println!("TURN {}", turn);
        let turn_history = history.entry(last_number).or_insert(Vec::new());
        let number = if turn_history.len() >= 2 {
            let n1 = turn_history[turn_history.len() - 1];
            let n2 = turn_history[turn_history.len() - 2];
            // println!("Number {} last said on turn {} and {}", last_number, n1, n2);
            n1 - n2
        } else {
            // println!(
            //     "Number {} only said {} times",
            //     last_number,
            //     turn_history.len()
            // );
            0
        };

        let turn_history = history.entry(number).or_insert(Vec::new());
        turn_history.push(turn);
        last_number = number;
        // println!("Number: {}", number);
        // println!("");

        if turn == target_turn {
            return number;
        }

        turn += 1;
    }
}

fn get_numbers() -> Vec<u32> {
    return vec![10, 16, 6, 0, 1, 17];
}
