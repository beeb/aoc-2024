use enumflags2::{bitflags, BitFlags};
use rayon::iter::{ParallelBridge, ParallelIterator as _};
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: usize = 130;

pub type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;
pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;

pub struct Day06;

/// The possible directions for the guard
#[bitflags]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Retrieve the direction if the guard turns right
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

/// A guard, with its coordinates and direction
#[derive(Debug, Default, Clone)]
pub struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

/// The state object, with the grid, guard and list of visited tiles
#[derive(Debug, Default, Clone)]
pub struct State {
    /// A list of all obstacle coordinates
    obstacles: HashSet<(usize, usize)>,
    /// A list of visited locations and in which directions the guard was pointing as they were visited
    visited: HashMap<(usize, usize), BitFlags<Direction>>,
    /// The initial position of the guard
    init_pos: (usize, usize),
    /// The guard's current position and direction
    guard: Guard,
}

impl State {
    /// If the next tile when moving in `dir` is in bounds, retrieve its coordinates, else `None`
    fn point_in_dir(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::Right => {
                if x == GRID_SIZE - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::Down => {
                if y == GRID_SIZE - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
    }

    /// Register that some coordinate (`x`, `y`) was visited by the guard while pointing in direction `dir`
    ///
    /// Returns `true` if the tile was not already visited while pointing in this direction, `false` if already visited
    fn register_visited(&mut self, x: usize, y: usize, dir: Direction) -> bool {
        if let Some(dirs) = self.visited.get_mut(&(x, y)) {
            if !dirs.contains(dir) {
                *dirs |= dir;
                true
            } else {
                false
            }
        } else {
            self.visited.insert((x, y), dir.into());
            true
        }
    }

    /// Advance the guard position by one move (turn or step)
    ///
    /// If the function returns `None`, then the guard entered a loop. If the return value is `Some(true)`, then the
    /// guard advanced by 1 step or turned. If the return value is `Some(false)`, then the guard exited the area.
    fn advance(&mut self) -> Option<bool> {
        if let Some((new_x, new_y)) = self.point_in_dir(self.guard.x, self.guard.y, self.guard.dir)
        {
            if self.obstacles.contains(&(new_x, new_y)) {
                self.guard.dir = self.guard.dir.turn_right();
                if !self.register_visited(self.guard.x, self.guard.y, self.guard.dir) {
                    return None; // loop
                };
            } else {
                self.guard.x = new_x;
                self.guard.y = new_y;
                if !self.register_visited(new_x, new_y, self.guard.dir) {
                    return None; // loop
                }
            }
            return Some(true); // advanced
        }
        Some(false) // out of bounds
    }

    /// Checks whether the guard would enter a loop if an obstacle is added at position `extra_obstacle`
    fn loops_with_obstacle(&self, extra_obstacle: (usize, usize)) -> bool {
        let mut state = self.clone();
        state.obstacles.insert(extra_obstacle);
        loop {
            match state.advance() {
                Some(true) => {}
                Some(false) => return false,
                None => return true,
            }
        }
    }
}

/// Parse a row of the grid, returning the x coordinate and symbol of each non-empty tile
fn parse_line(input: &mut &str) -> PResult<Vec<(usize, char)>> {
    let cells: Vec<_> = repeat(1.., one_of(('.', '#', '^', '>', 'v', '<'))).parse_next(input)?;
    Ok(cells
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .collect())
}

impl Day for Day06 {
    type Input = State;

    /// Parse the puzzle input into a [`State`]
    ///
    /// Parsing took 110.18us
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let mut puzzle = State::default();
        let lines: Vec<_> = separated(1.., parse_line, line_ending).parse_next(input)?;
        for (y, line) in lines.into_iter().enumerate() {
            for (x, cell) in line {
                match cell {
                    '#' => {
                        puzzle.obstacles.insert((x, y));
                    }
                    '^' => {
                        puzzle.guard = Guard {
                            x,
                            y,
                            dir: Direction::Up,
                        };
                    }
                    '>' => {
                        puzzle.guard = Guard {
                            x,
                            y,
                            dir: Direction::Right,
                        };
                    }
                    'v' => {
                        puzzle.guard = Guard {
                            x,
                            y,
                            dir: Direction::Down,
                        };
                    }
                    '<' => {
                        puzzle.guard = Guard {
                            x,
                            y,
                            dir: Direction::Left,
                        };
                    }
                    _ => unreachable!(),
                }
            }
        }
        puzzle.init_pos.0 = puzzle.guard.x;
        puzzle.init_pos.1 = puzzle.guard.y;
        puzzle.register_visited(puzzle.guard.x, puzzle.guard.y, puzzle.guard.dir);
        Ok(puzzle)
    }

    type Output1 = usize;

    /// Part 1 took 255.44us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut state = input.clone();
        // advance the guard until it exits the area
        while let Some(true) = state.advance() {}
        // return how many tiles were visited
        state.visited.len()
    }

    type Output2 = usize;

    /// Part 2 took 29.03ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut state = input.clone();
        // advance the guard until it exits the area to update the list of visited tiles
        while let Some(true) = state.advance() {}
        // for each visited tile, try to replace it with an obstacle and see if the guard enters a loop in that case
        // note that no obstacle can be placed at the guard's starting location
        state
            .visited
            .keys()
            .par_bridge()
            .filter(|&pos| pos != &state.init_pos && input.loops_with_obstacle(*pos))
            .count()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part2() {
        let parsed = Day06::parser(&mut INPUT).unwrap();
        assert_eq!(Day06::part_2(&parsed), 6);
    }
}
