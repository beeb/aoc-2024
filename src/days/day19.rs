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

fn parse_available(input: &mut &str) -> PResult<Vec<String>> {
    separated(1.., alpha1.map(|s: &str| s.to_string()), ", ").parse_next(input)
}

fn parse_desired(input: &mut &str) -> PResult<Vec<String>> {
    separated(1.., alpha1.map(|s: &str| s.to_string()), line_ending).parse_next(input)
}

fn can_create(pattern: &str, available: &[String]) -> bool {
    for towel in available {
        if pattern == towel {
            return true;
        }
        if let Some(next) = pattern.strip_prefix(towel) {
            if can_create(next, available) {
                return true;
            }
        }
    }
    false
}

fn count_combinations(
    pattern: &str,
    available: &[String],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(res) = cache.get(pattern) {
        return *res;
    }
    let mut res = 0;
    for towel in available {
        if pattern == towel {
            res += 1;
        } else if let Some(next) = pattern.strip_prefix(towel) {
            res += count_combinations(next, available, cache);
        }
    }
    cache.insert(pattern.to_string(), res);
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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .desired
            .iter()
            .filter(|d| can_create(d, &input.available))
            .count()
    }

    type Output2 = usize;

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
