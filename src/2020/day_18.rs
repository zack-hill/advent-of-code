use crate::solver::AoCSolver;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// nom code from https://github.com/Geal/nom/blob/master/tests/arithmetic.rs
// TODO: The two solutions no not share any code. It would be nice if the factor/parens functions could be shared at least.

pub struct Solver {
    lines: Vec<String>,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2020/day_18.txt").unwrap();
        let reader = BufReader::new(file);
        return Solver {
            lines: reader.lines().map(|l| l.unwrap()).collect(),
        };
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        self.lines
            .iter()
            .map(|l| p1_expr(l.as_str()).unwrap().1)
            .sum::<i64>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.lines
            .iter()
            .map(|l| p2_expr(l.as_str()).unwrap().1)
            .sum::<i64>()
            .to_string()
    }
}

fn p1_expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = p1_factor(i)?;

    return fold_many0(
        pair(alt((char('+'), char('*'))), p1_factor),
        || init,
        |acc, (op, val): (char, i64)| match op {
            '+' => acc + val,
            _ => acc * val,
        },
    )(i);
}

fn p1_factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        p1_parens,
    ))(i)
}

fn p1_parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), p1_expr, tag(")")), space)(i)
}

fn p2_expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = p2_term(i)?;

    return fold_many0(
        pair(char('*'), p2_term),
        || init,
        |acc, (_, val): (char, i64)| acc * val,
    )(i);
}

fn p2_term(i: &str) -> IResult<&str, i64> {
    let (i, init) = p2_factor(i)?;

    return fold_many0(
        pair(char('+'), p2_factor),
        || init,
        |acc, (_, val): (char, i64)| acc + val,
    )(i);
}

fn p2_factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        p2_parens,
    ))(i)
}

fn p2_parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), p2_expr, tag(")")), space)(i)
}
