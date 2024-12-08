use ahash::HashSetExt;
use itertools::Itertools;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::none_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: i8 = if cfg!(test) { 12 } else { 50 };

pub type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;
pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;

pub struct Day08;

/// An antenna's position
#[derive(Debug)]
pub struct Antenna {
    x: i8,
    y: i8,
}

/// Parse a row of the grid, returning a list of x coordinates and antenna frequency
fn parse_row(input: &mut &str) -> PResult<Vec<(usize, char)>> {
    let cells: Vec<_> = repeat(1.., none_of(['\n'])).parse_next(input)?;
    Ok(cells
        .into_iter()
        .enumerate()
        .filter(|(_, s)| *s != '.')
        .collect())
}

/// Get the antinodes, knowing the list of antennae grouped by frequency
fn get_antinodes(antennae: &HashMap<char, Vec<Antenna>>, part1: bool) -> HashSet<(i8, i8)> {
    let mut antinodes = HashSet::new();
    for list in antennae.values() {
        antinodes.extend(
            list.iter()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    let mut res = Vec::new();
                    let dx = a.x - b.x;
                    let dy = a.y - b.y;
                    let (start, limit) = if part1 {
                        (1, 2)
                    } else {
                        (0, (GRID_SIZE / dx.abs()).min(GRID_SIZE / dy.abs()))
                    };
                    for i in start..limit {
                        res.push((a.x + i * dx, a.y + i * dy));
                        res.push((b.x - i * dx, b.y - i * dy));
                    }
                    res
                })
                .filter(|(ax, ay)| (0..GRID_SIZE).contains(ax) && (0..GRID_SIZE).contains(ay)),
        );
    }
    antinodes
}

impl Day for Day08 {
    type Input = HashMap<char, Vec<Antenna>>;

    /// Parsing took 34.2us
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let rows: Vec<_> = separated(1.., parse_row, line_ending).parse_next(input)?;
        let mut out = HashMap::<char, Vec<Antenna>>::default();
        for (y, row) in rows.into_iter().enumerate() {
            for (x, symbol) in row {
                let antenna = Antenna {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                if let Some(antennae) = out.get_mut(&symbol) {
                    antennae.push(antenna);
                } else {
                    out.insert(symbol, vec![antenna]);
                }
            }
        }
        Ok(out)
    }

    type Output1 = usize;

    /// Part 1 took 16.33us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_antinodes(input, true).len()
    }

    type Output2 = usize;

    /// Part 2 took 61.9us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        get_antinodes(input, false).len()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part2() {
        let parsed = Day08::parser(&mut INPUT).unwrap();
        assert_eq!(Day08::part_2(&parsed), 34);
    }
}
