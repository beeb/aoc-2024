use winnow::{
    ascii::digit1,
    combinator::{alt, delimited, repeat, repeat_till, separated_pair, terminated},
    token::any,
    PResult, Parser as _,
};

use crate::days::Day;

pub struct Day03;

/// An instruction, either "mul", "do" or "don't"
#[derive(Debug)]
pub enum Instr {
    Mul { x: u64, y: u64 },
    Do,
    Dont,
}

/// Parser for multiplication instructions
fn parse_mul(input: &mut &str) -> PResult<Instr> {
    let (x, y) = delimited(
        "mul(",
        separated_pair(digit1.parse_to(), ',', digit1.parse_to()),
        ')',
    )
    .parse_next(input)?;
    Ok(Instr::Mul { x, y })
}

/// Parser for "do" instructions
fn parse_do(input: &mut &str) -> PResult<Instr> {
    "do()".map(|_| Instr::Do).parse_next(input)
}

/// Parser for "don't" instructions
fn parse_dont(input: &mut &str) -> PResult<Instr> {
    "don't()".map(|_| Instr::Dont).parse_next(input)
}

/// Parser for a sequence of garbage bytes followed by an instruction
///
/// This parser consumes but ignores the garbage portion.
fn parse_instr(input: &mut &str) -> PResult<Instr> {
    let ((), instr) =
        repeat_till(0.., any, alt((parse_mul, parse_do, parse_dont))).parse_next(input)?;
    Ok(instr)
}

impl Day for Day03 {
    type Input = Vec<Instr>;

    /// Parsing took 119.5us
    ///
    /// Could also be done with a regex: (?:mul\((\d+),(\d+)\)|(do(?:n't)?\(\)).*?)+?
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        terminated(
            repeat(1.., parse_instr),
            repeat::<_, _, (), _, _>(0.., any), // there could be garbage after the last instruction
        )
        .parse_next(input)
    }

    type Output1 = u64;

    /// Part 1 took 1.1us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter_map(|m| match m {
                Instr::Mul { x, y } => Some(x * y),
                _ => None,
            })
            .sum()
    }

    type Output2 = u64;

    /// Part 2 took 2.67us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut accumulate = true;
        let mut sum = 0;
        for instr in input {
            match instr {
                Instr::Mul { x, y } => {
                    if accumulate {
                        sum += x * y
                    }
                }
                Instr::Do => accumulate = true,
                Instr::Dont => accumulate = false,
            }
        }
        sum
    }
}
