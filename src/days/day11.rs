use winnow::{ascii::digit1, combinator::separated, PResult, Parser as _};

use crate::days::Day;

pub struct Day11;

pub type HashMap<K, T> = std::collections::HashMap<K, T, ahash::RandomState>;

/// Recursively find out the expanded length of a stone given its number and the number of iterations
fn expanded_length(
    number: u64,
    remaining: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    // end condition, we reached the number of iterations and so we add one stone to the final count
    if remaining == 0 {
        return 1;
    }
    // check cache to see if we've already expanded a similar stone by the required amount of iterations
    if let Some(len) = cache.get(&(number, remaining)) {
        return *len;
    }

    // pre-calculate the number of digits
    let digits = if number > 0 {
        Some(number.ilog10() + 1)
    } else {
        None
    };
    // handle the rules
    let res = match (number, digits) {
        (0, _) => expanded_length(1, remaining - 1, cache),
        (_, Some(d)) if d % 2 == 0 => {
            let power = 10u64.pow(digits.unwrap() / 2);
            let first = number / power;
            let second = number - first * power;
            expanded_length(first, remaining - 1, cache)
                + expanded_length(second, remaining - 1, cache)
        }
        _ => expanded_length(number * 2024, remaining - 1, cache),
    };
    // populate the cache
    cache.insert((number, remaining), res);
    res
}

impl Day for Day11 {
    type Input = Vec<u64>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., digit1.parse_to::<u64>(), ' ').parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut cache = HashMap::default();
        input
            .iter()
            .map(|v| expanded_length(*v, 25, &mut cache))
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut cache = HashMap::default();
        input
            .iter()
            .map(|v| expanded_length(*v, 75, &mut cache))
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        let parsed = Day11::parser(&mut INPUT).unwrap();
        assert_eq!(Day11::part_1(&parsed), 55312);
    }
}
