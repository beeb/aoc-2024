use winnow::{
    ascii::{alpha1, line_ending},
    combinator::separated,
    seq, PResult, Parser as _,
};

use crate::days::Day;

pub type HashMap<K, T> = std::collections::HashMap<K, T, ahash::RandomState>;

pub struct Day19;

#[derive(Debug, Clone)]
pub struct Puzzle {
    available: Vec<String>,
    desired: Vec<String>,
}

/// Parse the available towel patterns
fn parse_available(input: &mut &str) -> PResult<Vec<String>> {
    separated(1.., alpha1.map(|s: &str| s.to_string()), ", ").parse_next(input)
}

/// Parse the desired towel arrangements
fn parse_desired(input: &mut &str) -> PResult<Vec<String>> {
    separated(1.., alpha1.map(|s: &str| s.to_string()), line_ending).parse_next(input)
}

/// Check whether an arrangement can be created from the available towels
fn can_create(arr: &str, available: &[String]) -> bool {
    for towel in available {
        if arr == towel {
            return true;
        }
        if let Some(next) = arr.strip_prefix(towel) {
            if can_create(next, available) {
                return true;
            }
        }
    }
    false
}

/// Count how many ways there are to arrange available towels into the desired arrangement
fn count_combinations(
    arr: &str,
    available: &[String],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(res) = cache.get(arr) {
        return *res;
    }
    let mut res = 0;
    for towel in available {
        if arr == towel {
            res += 1;
        } else if let Some(next) = arr.strip_prefix(towel) {
            res += count_combinations(next, available, cache);
        }
    }
    cache.insert(arr.to_string(), res);
    res
}

impl Day for Day19 {
    type Input = Puzzle;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        seq!(Puzzle {
            available: parse_available,
            _: "\n\n",
            desired: parse_desired
        })
        .parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 2.17ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .desired
            .iter()
            .filter(|d| can_create(d, &input.available))
            .count()
    }

    type Output2 = usize;

    /// Part 2 tool 18.3ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut cache = HashMap::default();
        input
            .desired
            .iter()
            .map(|d| count_combinations(d, &input.available, &mut cache))
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        let parsed = Day19::parser(&mut INPUT).unwrap();
        assert_eq!(Day19::part_1(&parsed), 6);
    }

    #[test]
    fn test_part2() {
        let parsed = Day19::parser(&mut INPUT).unwrap();
        assert_eq!(Day19::part_2(&parsed), 16);
    }
}
