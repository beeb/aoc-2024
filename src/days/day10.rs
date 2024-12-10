use std::collections::VecDeque;

use itertools::Itertools as _;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: usize = 48;

/// Top - Right - Bottom - Left
const DIRS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct Day10;

pub type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    /// Retrieve the elevation at the coordinate of the point
    fn elevation(&self, map: &[Vec<u8>]) -> u8 {
        map[self.y as usize][self.x as usize]
    }

    /// Get all neighbors of the point which have an elevation one higher than itself
    fn neighbors(&self, map: &[Vec<u8>]) -> Vec<Point> {
        let elev = self.elevation(map);
        DIRS.iter()
            .map(|(dx, dy)| Point {
                x: self.x + dx,
                y: self.y + dy,
            })
            .filter(|p| {
                map.get(p.y as usize).and_then(|row| row.get(p.x as usize)) == Some(&(elev + 1))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Puzzle {
    /// Elevation map
    map: Vec<Vec<u8>>,
    /// Trail heads with an elevation of 0
    trail_heads: Vec<Point>,
}

/// Parse a row of the input map
fn parse_row(input: &mut &str) -> PResult<Vec<u8>> {
    repeat(
        1..,
        one_of('0'..='9').map(|c: char| c.to_digit(10).unwrap() as u8),
    )
    .parse_next(input)
}

/// Search for all reachable points with an elevation of 9, starting from `start`
fn bfs_reach(start: &Point, map: &[Vec<u8>]) -> HashSet<Point> {
    let mut goals = HashSet::<Point>::default();
    let mut to_visit: Vec<Point> = start.neighbors(map);
    while let Some(candidate) = to_visit.pop() {
        if candidate.elevation(map) == 9 {
            goals.insert(candidate);
        } else {
            to_visit.extend(candidate.neighbors(map).into_iter());
        }
    }
    goals
}

/// Search for all trails which end at an elevation of 9, starting from `start`
fn dfs_reach(start: &Point, map: &[Vec<u8>]) -> Vec<Point> {
    let mut trails = Vec::<Point>::default();
    let mut to_visit: VecDeque<Point> = start.neighbors(map).into();
    while let Some(candidate) = to_visit.pop_front() {
        if candidate.elevation(map) == 9 {
            trails.push(candidate);
        } else {
            to_visit.extend(candidate.neighbors(map).into_iter());
        }
    }
    trails
}

impl Day for Day10 {
    type Input = Puzzle;

    /// Parse the input elevation map and identify trail heads
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let elevations: Vec<_> = separated(1.., parse_row, line_ending).parse_next(input)?;
        let trail_heads: Vec<_> = elevations
            .iter()
            .flatten()
            .positions(|e| *e == 0)
            .map(|idx| Point {
                x: (idx % GRID_SIZE) as i8,
                y: (idx / GRID_SIZE) as i8,
            })
            .collect();
        Ok(Puzzle {
            map: elevations,
            trail_heads,
        })
    }

    type Output1 = usize;

    /// Part 1 took 335us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .trail_heads
            .iter()
            .map(|p| bfs_reach(p, &input.map).len())
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 325.5us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .trail_heads
            .iter()
            .map(|p| dfs_reach(p, &input.map).len())
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part2() {
        let parsed = Day10::parser(&mut INPUT).unwrap();
        assert_eq!(Day10::part_2(&parsed), 81);
    }
}
