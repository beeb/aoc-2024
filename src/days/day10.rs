use pathfinding::grid::Grid;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::none_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: usize = 48;

/// Top - Right - Bottom - Left
const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct Day10;

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub struct Point {
    x: i8,
    y: i8,
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

fn dfs_reach(start: &Point, grid: &Grid) {}

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
        println!("{input:?}");
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
