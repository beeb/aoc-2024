use std::collections::VecDeque;

use pathfinding::grid::Grid;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

const GRID_SIZE: usize = if cfg!(test) { 15 } else { 141 };
const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const SAVINGS_LIMT: usize = if cfg!(test) { 50 } else { 100 };

use crate::days::Day;

pub type IndexSet<K> = indexmap::set::IndexSet<K, ahash::RandomState>;
pub type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;
pub type Pos = (usize, usize);

pub struct Day20;

/// Puzzle input
#[derive(Debug, Clone)]
pub struct Race {
    grid: Grid,
    start: Pos,
    end: Pos,
}

/// Parse a line of the racetrack
fn parse_line(input: &mut &str) -> PResult<Vec<char>> {
    repeat(1.., one_of(('#', '.', 'E', 'S'))).parse_next(input)
}

/// Parse the track into a list of list of characters
fn parse_grid(input: &mut &str) -> PResult<Vec<Vec<char>>> {
    separated(1.., parse_line, line_ending).parse_next(input)
}

/// Get the ordered list of racetrack coordinates
fn get_track(race: &Race) -> IndexSet<Pos> {
    let mut track = IndexSet::default();
    track.insert(race.start);
    let mut current = race.start;
    while current != race.end {
        for n in race.grid.neighbours(current) {
            if track.contains(&n) {
                continue;
            }
            track.insert(n);
            current = n;
            break;
        }
    }
    track
}

/// Count the possible cheats starting at `pos` with maximum `moves` steps
fn count_possible_cheats(pos: Pos, track: &IndexSet<Pos>, moves: usize) -> usize {
    let curr_time = track.get_index_of(&pos).unwrap(); // time at which we reach `pos`
    let mut count = 0;
    let mut seen = HashSet::<Pos>::default(); // visited coordinates
    seen.insert(pos);
    let mut candidates = VecDeque::from([(pos, moves)]); // candidates for DFS
    while let Some((candidate, rem_moves)) = candidates.pop_front() {
        // if the candidate lies on the track, we check if the cheat makes us gain at least 100ps
        // the index into the racetrack list is the time when we visit that location
        if let Some(time) = track.get_index_of(&candidate) {
            let steps = moves - rem_moves; // need to subtract the length of the cheat
            if time.saturating_sub(curr_time).saturating_sub(steps) >= SAVINGS_LIMT {
                count += 1;
            }
        }
        // if we reached the maximum number of cheat steps, we can't go further
        if rem_moves == 0 {
            continue;
        }
        // check which of the four neighbours we can visit (inside the grid)
        let neighbours = DIRS.iter().filter_map(|(dx, dy)| {
            let x = candidate.0 as isize + dx;
            let y = candidate.1 as isize + dy;
            if x < 0 || y < 0 || x as usize > GRID_SIZE - 1 || y as usize > GRID_SIZE - 1 {
                return None;
            }
            Some((x as usize, y as usize))
        });
        for (x, y) in neighbours {
            // for each neighbour we haven't visited yet, we add it to the DFS list
            if !seen.contains(&(x, y)) {
                candidates.push_back(((x, y), rem_moves - 1));
                seen.insert((x, y));
            }
        }
    }
    count
}

impl Day for Day20 {
    type Input = Race;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid: Grid = parse_grid
            .parse_next(input)?
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, c)| {
                let x = i % GRID_SIZE;
                let y = i / GRID_SIZE;
                match c {
                    'S' => {
                        start = (x, y);
                        Some((x, y))
                    }
                    'E' => {
                        end = (x, y);
                        Some((x, y))
                    }
                    '.' => Some((x, y)),
                    '#' => None,
                    _ => unreachable!(),
                }
            })
            .collect();
        Ok(Race { grid, start, end })
    }

    type Output1 = usize;

    /// Part 1 took 4.27ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let track = get_track(input);
        track
            .iter()
            .map(|pos| count_possible_cheats(*pos, &track, 2))
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 227.8ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let track = get_track(input);
        track
            .iter()
            .map(|pos| count_possible_cheats(*pos, &track, 20))
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        let parsed = Day20::parser(&mut INPUT).unwrap();
        assert_eq!(Day20::part_1(&parsed), 0);
    }

    #[test]
    fn test_part2() {
        let parsed = Day20::parser(&mut INPUT).unwrap();
        assert_eq!(Day20::part_2(&parsed), 285);
    }
}
