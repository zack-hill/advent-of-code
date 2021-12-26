use crate::solver::AoCSolver;
use std::collections::HashMap;

pub struct Solver {
    numbers: Vec<u32>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            numbers: vec![10, 16, 6, 0, 1, 17],
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        return get_number_at_turn(2020, &self.numbers).to_string();
    }

    fn solve_part_2(&self) -> String {
        return get_number_at_turn(30000000, &self.numbers).to_string();
    }
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
