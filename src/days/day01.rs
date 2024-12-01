use itertools::Itertools;
use winnow::{
    ascii::{digit1, newline, space1},
    combinator::{separated, separated_pair},
    PResult, Parser,
};

use crate::days::Day;

pub struct Day01;

/// The two columns of the input file parsed as integers
pub struct Numbers {
    a: Vec<i32>,
    b: Vec<i32>,
}

fn parse_line(input: &mut &str) -> PResult<(i32, i32)> {
    separated_pair(digit1.parse_to(), space1, digit1.parse_to()).parse_next(input)
}

impl Day for Day01 {
    type Input = Numbers;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let lines: Vec<_> = separated(0.., parse_line, newline).parse_next(input)?;
        let (a, b) = lines.into_iter().unzip();
        Ok(Numbers { a, b })
    }

    type Output1 = usize;

    /// Part 1 took 29.48us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .a
            .iter()
            .sorted_unstable()
            .zip(input.b.iter().sorted_unstable())
            .map(|(a, b)| (a - b).unsigned_abs() as usize)
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 46.69us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let counts = input.b.iter().counts();
        input
            .a
            .iter()
            .map(|a| *a as usize * counts.get(a).copied().unwrap_or(0))
            .sum()
    }
}
