use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::none_of,
    PResult, Parser as _,
};

use crate::days::Day;

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
    fn elevation(&self, map: &[Vec<u8>]) -> u8 {
        map[self.y as usize][self.x as usize]
    }

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
    map: Vec<Vec<u8>>,
    trail_heads: Vec<Point>,
}

fn parse_row(input: &mut &str) -> PResult<Vec<u8>> {
    let chars: Vec<_> = repeat(1.., none_of('\n')).parse_next(input)?;
    Ok(chars
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect())
}

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

impl Day for Day10 {
    type Input = Puzzle;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let elevations: Vec<_> = separated(1.., parse_row, line_ending).parse_next(input)?;
        let mut trail_heads = Vec::new();
        for (y, row) in elevations.iter().enumerate() {
            for (x, elevation) in row.iter().enumerate() {
                if *elevation == 0 {
                    trail_heads.push(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    });
                }
            }
        }
        Ok(Puzzle {
            map: elevations,
            trail_heads,
        })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .trail_heads
            .iter()
            .map(|p| bfs_reach(p, &input.map).len())
            .sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
