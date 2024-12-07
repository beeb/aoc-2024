use itertools::Itertools;
use winnow::{
    ascii::{digit1, line_ending},
    combinator::{separated, separated_pair},
    PResult, Parser as _,
};

use crate::days::Day;

pub struct Day07;

#[derive(Debug)]
pub struct Line {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Mul,
    Concat,
}

/// Parse a list of operands separated by spaces
fn parse_operands(input: &mut &str) -> PResult<Vec<u64>> {
    separated(1.., digit1.parse_to::<u64>(), ' ').parse_next(input)
}

/// Parse a line which consists of a result and operands separated by a colon and space
fn parse_line(input: &mut &str) -> PResult<Line> {
    let (result, operands) =
        separated_pair(digit1.parse_to::<u64>(), ": ", parse_operands).parse_next(input)?;
    Ok(Line { result, operands })
}

/// Try to combine `operands` with any combination of `operators` and check it the result matches `result`
fn try_operators(result: u64, operands: &[u64], operators: &[Operator]) -> bool {
    let num_operators = operands.len() - 1;
    let ops_comb = (0..num_operators)
        .map(|_| operators.iter())
        .multi_cartesian_product();
    for ops in ops_comb {
        let res = operands
            .iter()
            .tuple_windows()
            .zip(ops.iter())
            .fold(0, |acc, ((a, b), op)| {
                let first = if acc == 0 { *a } else { acc };
                match op {
                    Operator::Add => first + b,
                    Operator::Mul => first * b,
                    Operator::Concat => first * 10u64.pow(b.ilog10() + 1) + b,
                }
            });
        if res == result {
            return true;
        }
    }
    false
}

impl Day for Day07 {
    type Input = Vec<Line>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_line, line_ending).parse_next(input)
    }

    type Output1 = u64;

    /// Part 1 took 7.96ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter_map(|l| {
                if try_operators(l.result, &l.operands, &[Operator::Add, Operator::Mul]) {
                    Some(l.result)
                } else {
                    None
                }
            })
            .sum()
    }

    type Output2 = u64;

    /// Part 2 took 321.1ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter_map(|l| {
                if try_operators(
                    l.result,
                    &l.operands,
                    &[Operator::Add, Operator::Mul, Operator::Concat],
                ) {
                    Some(l.result)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let parsed = Day07::parser(&mut INPUT).unwrap();
        assert_eq!(Day07::part_1(&parsed), 3749);
    }

    #[test]
    fn test_part2() {
        let parsed = Day07::parser(&mut INPUT).unwrap();
        assert_eq!(Day07::part_2(&parsed), 11387);
    }
}
