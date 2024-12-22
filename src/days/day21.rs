use std::iter::once;

use itertools::Itertools;
use pathfinding::{grid::Grid, prelude::astar_bag_collect};
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

pub type HashMap<K, T> = std::collections::HashMap<K, T, ahash::RandomState>;
type Pos = (usize, usize);

pub struct Day21;

/// A key of the final numpad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Numpad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

/// All the numpad keys
const NUMPAD: [Numpad; 11] = [
    Numpad::Zero,
    Numpad::One,
    Numpad::Two,
    Numpad::Three,
    Numpad::Four,
    Numpad::Five,
    Numpad::Six,
    Numpad::Seven,
    Numpad::Eight,
    Numpad::Nine,
    Numpad::A,
];

/// Convert from a numpad key to its coordinate on the keypad
impl From<&Numpad> for Pos {
    fn from(value: &Numpad) -> Self {
        match value {
            Numpad::Zero => (1, 3),
            Numpad::One => (0, 2),
            Numpad::Two => (1, 2),
            Numpad::Three => (2, 2),
            Numpad::Four => (0, 1),
            Numpad::Five => (1, 1),
            Numpad::Six => (2, 1),
            Numpad::Seven => (0, 0),
            Numpad::Eight => (1, 0),
            Numpad::Nine => (2, 0),
            Numpad::A => (2, 3),
        }
    }
}

/// Convert from a char to the relevant numpad key
impl From<char> for Numpad {
    fn from(value: char) -> Self {
        match value {
            '0' => Numpad::Zero,
            '1' => Numpad::One,
            '2' => Numpad::Two,
            '3' => Numpad::Three,
            '4' => Numpad::Four,
            '5' => Numpad::Five,
            '6' => Numpad::Six,
            '7' => Numpad::Seven,
            '8' => Numpad::Eight,
            '9' => Numpad::Nine,
            'A' => Numpad::A,
            _ => unreachable!(),
        }
    }
}

/// A directional pad key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dirpad {
    Up,
    Right,
    Down,
    Left,
    Press,
}

/// All the dirpad keys
const DIRPAD: [Dirpad; 5] = [
    Dirpad::Up,
    Dirpad::Right,
    Dirpad::Down,
    Dirpad::Left,
    Dirpad::Press,
];

/// Convert from dirpad key to its coordinate on the keypad
impl From<&Dirpad> for Pos {
    fn from(value: &Dirpad) -> Self {
        match value {
            Dirpad::Up => (1, 0),
            Dirpad::Right => (2, 1),
            Dirpad::Down => (1, 1),
            Dirpad::Left => (0, 1),
            Dirpad::Press => (2, 0),
        }
    }
}

/// Parse a sequence of numpad keys
fn parse_seq(input: &mut &str) -> PResult<Vec<Numpad>> {
    let chars: Vec<_> = repeat(4, one_of('0'..='A')).parse_next(input)?;
    Ok(chars.into_iter().map(Into::into).collect())
}

/// Create the numeric keypad (3 by 4 grid without the lower-left corner)
fn make_numeric_keypad() -> Grid {
    let mut grid = Grid::new(3, 4);
    grid.fill();
    grid.remove_vertex((0, 3));
    grid
}

/// Create the directional keypad (3 by 2 without the top-left corner)
fn make_dir_keypad() -> Grid {
    let mut grid = Grid::new(3, 2);
    grid.fill();
    grid.remove_vertex((0, 0));
    grid
}

/// Find all paths from a coordinate to another on a grid, and convert them to sequences of moves
/// (always ending with a keypress)
fn paths(
    start: Pos,
    end: Pos,
    grid: &Grid,
    cache: &mut HashMap<(Pos, Pos), Vec<Vec<Dirpad>>>,
) -> Vec<Vec<Dirpad>> {
    if let Some(res) = cache.get(&(start, end)) {
        return res.clone();
    }
    let res: Vec<_> = astar_bag_collect(
        &start,
        |p| grid.neighbours(*p).into_iter().map(|n| (n, 1)),
        |p| p.0.abs_diff(end.0) + p.1.abs_diff(end.1),
        |p| *p == end,
    )
    .unwrap()
    .0
    .into_iter()
    .map(|seq| {
        seq.into_iter()
            .tuple_windows()
            .map(
                |(a, b)| match (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize) {
                    (0, -1) => Dirpad::Up,
                    (1, 0) => Dirpad::Right,
                    (0, 1) => Dirpad::Down,
                    (-1, 0) => Dirpad::Left,
                    x => unreachable!("{x:?}"),
                },
            )
            .chain(once(Dirpad::Press))
            .collect_vec()
    })
    .collect();
    cache.insert((start, end), res.clone());
    res
}

/// For each (start, end) dirpad/numpad key combination, what is the cost of moving to that key
fn move_cost<K: Copy + Eq + std::hash::Hash>(
    grid: &Grid,
    keys: &[K],
    prev: Option<&HashMap<(Dirpad, Dirpad), usize>>,
    paths_cache: &mut HashMap<(Pos, Pos), Vec<Vec<Dirpad>>>,
) -> HashMap<(K, K), usize>
where
    for<'a> &'a K: Into<Pos>,
{
    let mut res = HashMap::default();
    for (a, b) in keys.iter().cartesian_product(keys.iter()) {
        if a == b {
            res.insert((*a, *b), 1);
            continue;
        }
        let cost = paths(a.into(), b.into(), grid, paths_cache)
            .into_iter()
            .map(|path| {
                let Some(prev) = prev else {
                    return path.len();
                };
                let mut cost = 0;
                let mut parent = Dirpad::Press;
                for action in path {
                    // move and press
                    cost += prev.get(&(parent, action)).unwrap();
                    parent = action;
                }
                cost
            })
            .min()
            .unwrap();
        res.insert((*a, *b), cost);
    }
    res
}

/// Extract the numeric part of each code
fn code_to_num(code: &[Numpad]) -> usize {
    let str: String = code
        .iter()
        .map(|n| match n {
            Numpad::Zero => "0",
            Numpad::One => "1",
            Numpad::Two => "2",
            Numpad::Three => "3",
            Numpad::Four => "4",
            Numpad::Five => "5",
            Numpad::Six => "6",
            Numpad::Seven => "7",
            Numpad::Eight => "8",
            Numpad::Nine => "9",
            Numpad::A => "",
        })
        .collect();
    str.parse().unwrap()
}

impl Day for Day21 {
    type Input = Vec<Vec<Numpad>>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_seq, line_ending).parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 199.3us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let dir_keypad = make_dir_keypad();
        let num_keypad = make_numeric_keypad();
        let mut cache = HashMap::default();
        // construct the cost maps of each move at each level
        let cost = move_cost(&dir_keypad, &DIRPAD, None, &mut cache);
        let cost = move_cost(&dir_keypad, &DIRPAD, Some(&cost), &mut cache);
        let cost = move_cost(&num_keypad, &NUMPAD, Some(&cost), &mut cache);
        // for each code sequence, calculate the minimum cost to input it (adding a move from the initial A key)
        input
            .iter()
            .map(|code| {
                let len: usize = once(&Numpad::A)
                    .chain(code.iter())
                    .tuple_windows()
                    .map(|(a, b)| cost.get(&(*a, *b)).unwrap())
                    .sum();
                code_to_num(code) * len
            })
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 209.7us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let dir_keypad = make_dir_keypad();
        let num_keypad = make_numeric_keypad();
        let mut cache = HashMap::default();
        // construct the cost maps of each move at each level
        let mut cost = move_cost(&dir_keypad, &DIRPAD, None, &mut cache);
        for _ in 0..24 {
            cost = move_cost(&dir_keypad, &DIRPAD, Some(&cost), &mut cache);
        }
        let cost = move_cost(&num_keypad, &NUMPAD, Some(&cost), &mut cache);
        // for each code sequence, calculate the minimum cost to input it (adding a move from the initial A key)
        input
            .iter()
            .map(|code| {
                let len: usize = once(&Numpad::A)
                    .chain(code.iter())
                    .tuple_windows()
                    .map(|(a, b)| cost.get(&(*a, *b)).unwrap())
                    .sum();
                code_to_num(code) * len
            })
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        let parsed = Day21::parser(&mut INPUT).unwrap();
        assert_eq!(Day21::part_1(&parsed), 126384);
    }
}
