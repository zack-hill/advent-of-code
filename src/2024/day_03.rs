use crate::solver::AoCSolver;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res},
    multi::{fold_many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

pub struct Solver {
    instructions: Vec<Instruction>,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2024/day_03.txt").unwrap();
        let reader = BufReader::new(file);
        let instructions = reader
            .lines()
            .flat_map(|line| {
                if let Ok((_, x)) = parse_instructions(&line.unwrap()) {
                    x
                } else {
                    Vec::new()
                }
            })
            .collect();
        Solver { instructions }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        return self
            .instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(l, r) => l * r,
                Instruction::Do => 0,
                Instruction::Dont => 0,
            })
            .sum::<usize>()
            .to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut do_mul = true;
        return self
            .instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(l, r) => {
                    if do_mul {
                        l * r
                    } else {
                        0
                    }
                }
                Instruction::Do => {
                    do_mul = true;
                    0
                }
                Instruction::Dont => {
                    do_mul = false;
                    0
                }
            })
            .sum::<usize>()
            .to_string();
    }
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}

fn parse_mul(i: &str) -> IResult<&str, Instruction> {
    map(
        delimited(
            tag("mul("),
            separated_pair(parse_number, tag(","), parse_number),
            tag(")"),
        ),
        |(l, r)| Instruction::Mul(l, r),
    )(i)
}

fn parse_do(i: &str) -> IResult<&str, Instruction> {
    map(tag("do()"), |_: &str| Instruction::Do)(i)
}

fn parse_dont(i: &str) -> IResult<&str, Instruction> {
    map(tag("don't()"), |_: &str| Instruction::Dont)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(i)
}

fn parse_instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    fold_many0(
        many_till(anychar, parse_instruction),
        Vec::new,
        |mut acc: Vec<_>, (_, item)| {
            acc.push(item);
            acc
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_works() {
        let input = "123";
        let expected = 123;

        let (_, actual) = parse_number(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_mul_works() {
        let input = "mul(10,20)";
        let expected = Instruction::Mul(10, 20);

        let (_, actual) = parse_mul(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_do_works() {
        let input = "do()";
        let expected = Instruction::Do;

        let (_, actual) = parse_do(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_dont_works() {
        let input = "don't()";
        let expected = Instruction::Dont;

        let (_, actual) = parse_dont(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_instructions_works() {
        let input = "12dsmul(123,321tmul(10,20)asd23 do()fdsmul(231,1)dsad2don't()e2in";
        let expected = vec![
            Instruction::Mul(10, 20),
            Instruction::Do,
            Instruction::Mul(231, 1),
            Instruction::Dont,
        ];

        let (_, actual) = parse_instructions(input).unwrap();
        assert_eq!(actual, expected);
    }
}
