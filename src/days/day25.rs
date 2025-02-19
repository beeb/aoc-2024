use itertools::Itertools;
use winnow::{
    combinator::{alt, preceded, repeat, separated},
    token::{one_of, take},
    PResult, Parser as _,
};

use crate::days::Day;

pub struct Day25;

/// Enum to differentiate between the parsed values for keys vs locks
#[derive(Debug, Clone)]
pub enum Pins {
    Lock(Vec<u8>),
    Key(Vec<u8>),
}

/// Parse a lock input
fn parse_lock(input: &mut &str) -> PResult<Pins> {
    let mut out = vec![0; 5];
    for i in 0..5 {
        let prec = if i == 0 { "#####\n" } else { "\n" };
        let height: Vec<_> = preceded(prec, repeat(5, one_of(('.', '#')))).parse_next(input)?;
        height.into_iter().enumerate().for_each(|(i, p)| {
            out[i] += (p == '#') as u8;
        });
    }
    take(6usize).parse_next(input)?; // consume the last row
    Ok(Pins::Lock(out))
}

/// Parse a key input
fn parse_key(input: &mut &str) -> PResult<Pins> {
    let mut out = vec![0; 5];
    for i in 0..5 {
        let prec = if i == 0 { ".....\n" } else { "\n" };
        let height: Vec<_> = preceded(prec, repeat(5, one_of(('.', '#')))).parse_next(input)?;
        height.into_iter().enumerate().for_each(|(i, p)| {
            out[i] += (p == '#') as u8;
        });
    }
    take(6usize).parse_next(input)?; // consume the last row
    Ok(Pins::Key(out))
}

#[derive(Debug, Clone, Default)]
pub struct Puzzle {
    locks: Vec<Vec<u8>>,
    keys: Vec<Vec<u8>>,
}

/// Check whether a lock and key overlap.
///
/// They overlap if for any pin, the sum of both heights exceeds 5.
fn overlaps(lock: &[u8], key: &[u8]) -> bool {
    lock.iter().zip(key.iter()).any(|(l, k)| l + k > 5)
}

impl Day for Day25 {
    type Input = Puzzle;

    /// Parse keys and locks into the puzzle input struct
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let items: Vec<_> =
            separated(1.., alt((parse_lock, parse_key)), "\n\n").parse_next(input)?;
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        items.into_iter().for_each(|p| match p {
            Pins::Lock(vec) => locks.push(vec),
            Pins::Key(vec) => keys.push(vec),
        });
        Ok(Puzzle { locks, keys })
    }

    type Output1 = usize;

    /// Part 1 took 212.3us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .locks
            .iter()
            .cartesian_product(input.keys.iter())
            .filter(|(lock, key)| !overlaps(lock, key))
            .count()
    }

    type Output2 = usize;

    /// No part 2!
    fn part_2(_input: &Self::Input) -> Self::Output2 {
        0
    }
}
