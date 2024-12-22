use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::separated,
    PResult, Parser as _,
};

use crate::days::Day;

pub type HashMap<K, T> = std::collections::HashMap<K, T, ahash::RandomState>;

pub struct Day22;

/// The mix function
fn mix(value: usize, secret: usize) -> usize {
    value ^ secret
}

/// The prune function
fn prune(value: usize) -> usize {
    value % 16777216 // 24 bits
}

/// Calculate the next secret number knowing the previous one
fn next_number(mut prev: usize) -> usize {
    prev = prune(mix(prev * 64, prev));
    prev = prune(mix(prev / 32, prev));
    prev = prune(mix(prev * 2048, prev));
    prev
}

/// Helper to calculate the n-th secret number
fn nth_number(prev: usize, n: usize) -> usize {
    (0..n).fold(prev, |acc, _| next_number(acc))
}

/// Collect the banana price for any sequence of 4 price differences in the 2000 secret numbers generated from the seed
fn sequences(seed: usize) -> HashMap<(isize, isize, isize, isize), usize> {
    let mut out = HashMap::default();
    let mut diffs = Vec::new();
    let mut prev_price = seed % 10;
    let mut secret = seed;
    for _ in 0..2000 {
        secret = next_number(secret);
        let price = secret % 10;
        diffs.push(price as isize - prev_price as isize);
        prev_price = price;
        if let Some(seq) = diffs.iter().rev().take(4).copied().collect_tuple() {
            out.entry(seq).or_insert(price); // only the first price is used
        }
    }
    out
}

impl Day for Day22 {
    type Input = Vec<usize>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., dec_uint::<_, usize, _>, line_ending).parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 5.2ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|n| nth_number(*n, 2000)).sum()
    }

    type Output2 = usize;

    /// Part 2 took 102.05ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // collect the total number of bananas one would get depending on the given diff sequence
        let mut bananas = HashMap::<(isize, isize, isize, isize), usize>::default();
        for n in input {
            // for each of the sequences of a given buyer, accumulate the number of bananas into the hashmap
            for (seq, price) in sequences(*n) {
                bananas
                    .entry(seq)
                    .and_modify(|p| *p += price)
                    .or_insert(price);
            }
        }
        // the maximum of the values in the hashmap is our answer
        *bananas.values().max().unwrap()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "1
10
100
2024";

    const INPUT2: &str = "1
2
3
2024";

    #[test]
    fn test_part1() {
        let parsed = Day22::parser(&mut INPUT).unwrap();
        assert_eq!(Day22::part_1(&parsed), 37327623);
    }

    #[test]
    fn test_part2() {
        let parsed = Day22::parser(&mut INPUT2).unwrap();
        assert_eq!(Day22::part_2(&parsed), 23);
    }
}
