use std::{cmp::Ordering, collections::HashSet as StdHashSet};

use winnow::{
    ascii::{digit1, line_ending},
    combinator::{separated, separated_pair},
    PResult, Parser as _,
};

use crate::days::Day;

pub type HashSet<T> = StdHashSet<T, ahash::RandomState>;

pub struct Day05;

/// Struct to hold a list of pages
#[derive(Debug)]
pub struct Update {
    pages: Vec<u8>,
}

/// The puzzle input, consisting of a list of rules and a list of updates
#[derive(Debug)]
pub struct Puzzle {
    rules: HashSet<(u8, u8)>,
    updates: Vec<Update>,
}

/// Parse a single rule containing two page numbers defining their ordering
fn parse_rule(input: &mut &str) -> PResult<(u8, u8)> {
    separated_pair(digit1.parse_to(), '|', digit1.parse_to()).parse_next(input)
}

/// Parse all the rules into a hashset of (first, second) page tuples
fn parse_rules(input: &mut &str) -> PResult<HashSet<(u8, u8)>> {
    separated(1.., parse_rule, line_ending).parse_next(input)
}

/// Parse an update definition (a list of pages)
fn parse_update(input: &mut &str) -> PResult<Update> {
    let pages = separated(1.., digit1.parse_to::<u8>(), ',').parse_next(input)?;
    Ok(Update { pages })
}

/// Parse all updates
fn parse_updates(input: &mut &str) -> PResult<Vec<Update>> {
    separated(1.., parse_update, line_ending).parse_next(input)
}

/// Compare function for two page numbers
///
/// This function uses the rules hashset to determine if the ordering is ok or not.
fn compare_order(a: &u8, b: &u8, rules: &HashSet<(u8, u8)>) -> Ordering {
    if !rules.contains(&(*a, *b)) {
        Ordering::Greater // a should be after b
    } else {
        Ordering::Less // ordering is ok
    }
}

impl Day for Day05 {
    type Input = Puzzle;

    /// Parsing took 91.3us
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let (rules, updates) =
            separated_pair(parse_rules, (line_ending, line_ending), parse_updates)
                .parse_next(input)?;
        Ok(Puzzle { rules, updates })
    }

    type Output1 = usize;

    /// Part 1 took 6.6us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .updates
            .iter()
            .filter(|u| {
                u.pages.is_sorted_by(|a, b| {
                    matches!(compare_order(a, b, &input.rules), Ordering::Less)
                })
            })
            .map(|u| *(u.pages.get(u.pages.len() / 2).unwrap()) as usize)
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 49.5us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .updates
            .iter()
            .filter_map(|u| {
                let mut pages = u.pages.clone();
                pages.sort_unstable_by(|a, b| compare_order(a, b, &input.rules));
                if pages != u.pages {
                    Some(*(pages.get(pages.len() / 2).unwrap()) as usize)
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

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part2() {
        let parsed = Day05::parser(&mut INPUT).unwrap();
        assert_eq!(Day05::part_2(&parsed), 123);
    }
}
