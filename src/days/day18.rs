use pathfinding::{grid::Grid, prelude::astar};
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::separated,
    seq, PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: usize = if cfg!(test) { 7 } else { 71 };
const PART1_LEN: usize = if cfg!(test) { 12 } else { 1024 };

pub struct Day18;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn successors(&self, grid: &Grid) -> Vec<(Pos, usize)> {
        grid.neighbours(self.into())
            .into_iter()
            .map(|p| (p.into(), 1))
            .collect()
    }
}

impl From<&Pos> for (usize, usize) {
    fn from(value: &Pos) -> Self {
        (value.x, value.y)
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn parse_pos(input: &mut &str) -> PResult<Pos> {
    seq!(Pos {
        x: dec_uint,
        _: ',',
        y: dec_uint
    })
    .parse_next(input)
}

fn make_grid(obstacles: &[Pos]) -> Grid {
    let mut grid = Grid::new(GRID_SIZE, GRID_SIZE);
    grid.fill();
    for obs in obstacles {
        grid.remove_vertex(obs.into());
    }
    grid
}

impl Day for Day18 {
    type Input = Vec<Pos>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_pos, line_ending).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let goal: Pos = (GRID_SIZE - 1, GRID_SIZE - 1).into();
        let grid = make_grid(input.get(0..PART1_LEN).unwrap());
        let (_, score) = astar(
            &Pos { x: 0, y: 0 },
            |p| p.successors(&grid),
            |p| p.distance(&goal),
            |p| *p == goal,
        )
        .unwrap();
        score
    }

    type Output2 = String;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let start = (0, 0);
        let goal = (GRID_SIZE - 1, GRID_SIZE - 1);
        let (first, second) = input.split_at(PART1_LEN);
        let grid = make_grid(first);
        // binary search
        // index into the second half of the pieces
        let mut left = 0;
        let mut right = second.len() - 1;
        while left < right {
            let mut grid = grid.clone();
            let m = (left + right) / 2;
            // add obstacles with indices up to and including m
            for obs in second.get(0..=m).unwrap() {
                grid.remove_vertex(obs.into());
            }
            if grid.dfs_reachable(start, |_| true).contains(&goal) {
                // if we can still reach the exit, we increment the left bound
                left = m + 1;
            } else {
                // else we decrement the right bound
                right = m - 1;
            }
        }
        // when left == right, we found the first piece which cuts off the exit
        let obs = second.get(left).unwrap();
        format!("{},{}", obs.x, obs.y)
    }
}
