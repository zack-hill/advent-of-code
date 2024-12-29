use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;

use crate::solver::AoCSolver;

struct Equation {
    total: usize,
    numbers: Vec<usize>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Combine,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
            Operation::Combine => write!(f, "||"),
        }
    }
}

pub struct Solver {
    equations: Vec<Equation>,
}

impl Solver {
    pub fn create() -> Self {
        let equations = parse_input();
        Solver { equations }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        self.equations
            .iter()
            // .filter(|equation| find_valid_operators_2(&equation.numbers, equation.total).is_some())
            .filter(|equation| {
                try_find_valid_solution_using_recursion(
                    equation.numbers[0],
                    &equation.numbers,
                    1,
                    equation.total,
                    &vec![Operation::Add, Operation::Multiply],
                )
            })
            .map(|x| x.total)
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.equations
            .iter()
            .filter(|equation| {
                try_find_valid_solution_using_recursion(
                    equation.numbers[0],
                    &equation.numbers,
                    1,
                    equation.total,
                    &vec![Operation::Add, Operation::Multiply, Operation::Combine],
                )
            })
            .map(|x| x.total)
            .sum::<usize>()
            .to_string()
    }
}

fn try_find_valid_solution_using_recursion(
    left: usize,
    numbers: &Vec<usize>,
    index: usize,
    total: usize,
    operations: &Vec<Operation>,
) -> bool {
    let right = numbers[index];

    for operation in operations.iter() {
        let result = evaluate(left, right, *operation);
        // println!("{} {} {} = {}", left, operation, right, result);

        let is_leaf_node = index == numbers.len() - 1;
        if is_leaf_node {
            if result == total {
                // println!("{} {} {} = {}", left, operation, right, result);
                return true;
            }
        } else {
            if try_find_valid_solution_using_recursion(
                result,
                numbers,
                index + 1,
                total,
                &operations,
            ) {
                return true;
            }
        }
    }
    return false;
}

fn try_find_valid_solution_using_stacks(
    numbers: &Vec<usize>,
    total: usize,
) -> Option<Vec<Operation>> {
    let mut temp = numbers.clone();
    temp.reverse();
    let mut available_numbers = VecDeque::from(temp);
    let mut used_numbers = VecDeque::new();
    let mut result_stack = VecDeque::new();
    let mut operation_stack = VecDeque::from(vec![Operation::Add; 1]);

    result_stack.push_back(available_numbers.pop_back().unwrap());

    let mut backtrack = false;

    loop {
        let left = result_stack.pop_back().unwrap();
        let right = available_numbers.pop_back().unwrap();
        let operation = operation_stack.pop_back().unwrap();

        used_numbers.push_back(right);

        if backtrack {
            if operation == Operation::Multiply {
                available_numbers.push_back(right);
                continue;
            }
        }

        let result = evaluate(left, right, operation);

        println!("{} {} {} = {}", left, operation, right, result);

        if available_numbers.is_empty() {
            if result == total {
                println!("Result == Total");
                // return Some(Vec::new());
            } else {
                println!("Result != Total");
            }

            // If add didn't work, try multiply
            if operation == Operation::Add {
                println!("Operation was add");
                result_stack.push_back(left);
                available_numbers.push_back(right);
                operation_stack.push_back(Operation::Multiply);
            } else {
                println!("Operation was multiply, backtracking");
                result_stack.push_back(left);
                available_numbers.push_back(right);
                backtrack = true;
            }
        } else {
            println!("More available numbers");
            if operation == Operation::Add {
                println!("Operation was add");
                operation_stack.push_back(operation);
                operation_stack.push_back(Operation::Add);
                result_stack.push_back(left);
                result_stack.push_back(result);
            } else {
                println!("Operation was multiply");
                available_numbers.push_back(used_numbers.pop_back().unwrap());
                continue;
            }
        }
    }
    return None;
}

fn try_find_valid_solution_using_brute_force(
    numbers: &Vec<usize>,
    total: usize,
) -> Option<Vec<Operation>> {
    let add_operations = iter::repeat(Operation::Add).take(numbers.len() - 1);
    let multiply_operations = iter::repeat(Operation::Multiply).take(numbers.len() - 1);
    let oera: Vec<Operation> = add_operations.chain(multiply_operations).collect();

    println!("{}: {:?} Searching for valid operators...", total, numbers);

    for perm in oera.iter().permutations(numbers.len() - 1).unique() {
        if validate_operators(&numbers, &perm, total) {
            let operations = perm.into_iter().map(|x| x.to_owned()).collect();
            println!(
                "{}: {:?} is valid using operators {:?}",
                total, numbers, operations
            );
            return Some(operations);
        }
    }
    return None;
}

fn validate_operators(numbers: &Vec<usize>, operators: &Vec<&Operation>, total: usize) -> bool {
    let mut value = numbers[0];

    for i in 1..numbers.len() {
        value = evaluate(value, numbers[i], *operators[i - 1]);
        if value > total {
            return false;
        }
    }

    return value == total;
}

fn evaluate(left: usize, right: usize, operation: Operation) -> usize {
    match operation {
        Operation::Add => left + right,
        Operation::Multiply => left * right,
        Operation::Combine => [left.to_string(), right.to_string()]
            .concat()
            .parse()
            .unwrap(),
    }
}

fn parse_input() -> Vec<Equation> {
    let file = File::open("src/2024/day_07.txt").unwrap();
    let reader = BufReader::new(file);

    let mut equations = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let (total, numbers_section) = line.split_once(':').unwrap();
        let total: usize = total.parse().unwrap();
        let numbers: Vec<usize> = numbers_section
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        equations.push(Equation { total, numbers });
    }

    return equations;
}
