use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_part_1(instructions: &Vec<Instruction>) -> i64 {
    let mut pos = 0;
    let mut depth = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Up(amount) => depth -= amount,
            Instruction::Down(amount) => depth += amount,
            Instruction::Forward(amount) => pos += amount,
        };
    }
    return pos * depth;
}

pub fn solve_part_2(instructions: &Vec<Instruction>) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Up(amount) => aim -= amount,
            Instruction::Down(amount) => aim += amount,
            Instruction::Forward(amount) => {
                pos += amount;
                depth += amount * aim
            }
        }
    }
    return pos * depth;
}

pub enum Instruction {
    Up(i64),
    Down(i64),
    Forward(i64),
}

pub fn parse_input() -> Vec<Instruction> {
    let file = File::open("src/year_2021/day_02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut instructions = Vec::<Instruction>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let segements = line.split(" ").collect::<Vec<&str>>();
        let (instruction, amount) = (segements[0], segements[1]);
        let mut amount = amount.parse::<i64>().unwrap();

        let instruction = match instruction {
            "up" => Instruction::Up(amount),
            "down" => Instruction::Down(amount),
            "forward" => Instruction::Forward(amount),
            _ => panic!("Unexpected instruction")
        };
        instructions.push(instruction);
    }
    return instructions;
}
