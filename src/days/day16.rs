use pathfinding::{
    directed::astar::{astar, astar_bag},
    grid::Grid,
};
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

const GRID_SIZE: usize = if cfg!(test) { 15 } else { 141 };

use crate::days::Day;

pub type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

pub struct Day16;

/// Cardinal directions
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dir {
    North,
    #[default]
    East,
    South,
    West,
}

impl Dir {
    /// Direction after a right turn
    fn turn_right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    /// Direction after a left turn
    fn turn_left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }
}

/// The position of a reindeer, with its coordinates and the direction it's facing
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Pos {
    /// Manhattan distance, i.e. shortest possible path length to target
    fn distance(&self, grid: &Grid, other: &Pos) -> usize {
        grid.distance((self.x, self.y), (other.x, other.y))
    }

    /// Keep the same coordinates but turn right
    fn turn_right(&self) -> Pos {
        Pos {
            dir: self.dir.turn_right(),
            ..*self
        }
    }

    /// Keep the same coordinates but turn left
    fn turn_left(&self) -> Pos {
        Pos {
            dir: self.dir.turn_left(),
            ..*self
        }
    }

    /// All possible successors to the current position
    ///
    /// In all cases, the reindeer can turn left or right. Otherwise, the reindeer can move in the direction it's
    /// facing if there's a free tile there.
    fn successors(&self, grid: &Grid) -> Vec<(Pos, usize)> {
        // cost of turning is 1000
        let mut neighbours = vec![(self.turn_left(), 1000), (self.turn_right(), 1000)];
        for (x, y) in grid.neighbours((self.x, self.y)) {
            if (self.dir == Dir::West && x < self.x)
                || (self.dir == Dir::East && x > self.x)
                || (self.dir == Dir::North && y < self.y)
                || (self.dir == Dir::South && y > self.y)
            {
                neighbours.push((
                    Pos {
                        x,
                        y,
                        dir: self.dir,
                    },
                    1, // cost of advancing is 1
                ));
            }
        }
        neighbours
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            ..Default::default()
        }
    }
}

/// Puzzle input
#[derive(Debug, Clone)]
pub struct Puzzle {
    grid: Grid,
    start: Pos,
    end: Pos,
}

/// Parse a line of the maze
fn parse_line(input: &mut &str) -> PResult<Vec<char>> {
    repeat(1.., one_of(('#', '.', 'E', 'S'))).parse_next(input)
}

/// Parse the maze into a list of list of characters
fn parse_grid(input: &mut &str) -> PResult<Vec<Vec<char>>> {
    separated(1.., parse_line, line_ending).parse_next(input)
}

impl Day for Day16 {
    type Input = Puzzle;

    /// Parse the input into a grid, collecting the coordinates of the start and end positions
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let mut start = Pos::default();
        let mut end = Pos::default();
        let mut grid: Grid = parse_grid
            .parse_next(input)?
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, c)| {
                let x = i % GRID_SIZE;
                let y = i / GRID_SIZE;
                match c {
                    'S' => {
                        start = (x, y).into();
                        None
                    }
                    'E' => {
                        end = (x, y).into();
                        None
                    }
                    '.' => None,
                    '#' => Some((x, y)),
                    _ => unreachable!(),
                }
            })
            .collect();
        grid.invert(); // we indicated the positions of obstacles, need to invert
        Ok(Puzzle { grid, start, end })
    }

    type Output1 = usize;

    /// Part 1 took 7.48ms
    ///
    /// To see my implementation of A*, check out <https://github.com/beeb/aoc-2022/blob/main/src/days/day12.rs>
    /// Here I used a lib.
    fn part_1(input: &Self::Input) -> Self::Output1 {
        astar(
            &input.start,
            |p| p.successors(&input.grid),
            |p| p.distance(&input.grid, &input.end),
            |p| p.x == input.end.x && p.y == input.end.y,
        )
        .unwrap()
        .1
    }

    type Output2 = usize;

    /// Part 2 took 13.15ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        astar_bag(
            &input.start,
            |p| p.successors(&input.grid),
            |p| p.distance(&input.grid, &input.end),
            |p| p.x == input.end.x && p.y == input.end.y,
        )
        .unwrap()
        .0
        .flat_map(|path| path.into_iter().map(|pos| (pos.x, pos.y)))
        .collect::<HashSet<_>>()
        .len()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part1() {
        let parsed = Day16::parser(&mut INPUT).unwrap();
        assert_eq!(Day16::part_1(&parsed), 7036);
    }
}
