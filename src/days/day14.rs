use itertools::Itertools as _;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::separated,
    seq,
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_WIDTH: isize = if cfg!(test) { 11 } else { 101 };
const GRID_HALF_WIDTH: isize = GRID_WIDTH / 2;
const GRID_HEIGHT: isize = if cfg!(test) { 7 } else { 103 };
const GRID_HALF_HEIGHT: isize = GRID_HEIGHT / 2;

pub struct Day14;

/// A quadrant, or no quadrant (in the middle of the grid)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Quadrant {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// An offset into the grid, either a position or a velocity vector
#[derive(Debug, Clone)]
pub struct Offset {
    x: isize,
    y: isize,
}

impl Offset {
    /// Knowing a position on the grid, return which quadrant it belongs to
    ///
    /// Positions that lie on the center lines are in [`Quandrant::None`].
    fn quadrant(&self) -> Quadrant {
        match (self.x, self.y) {
            (GRID_HALF_WIDTH, _) | (_, GRID_HALF_HEIGHT) => Quadrant::None,
            (..=GRID_HALF_WIDTH, ..=GRID_HALF_HEIGHT) => Quadrant::TopLeft,
            (_, ..=GRID_HALF_HEIGHT) => Quadrant::TopRight,
            (..=GRID_HALF_WIDTH, _) => Quadrant::BottomLeft,
            _ => Quadrant::BottomRight,
        }
    }
}

/// A robot with its initial position and velocity
#[derive(Debug, Clone)]
pub struct Robot {
    start: Offset,
    vel: Offset,
}

impl Robot {
    /// Calculate the position of a robot after `time` seconds elapsed
    fn pos_after(&self, time: isize) -> Offset {
        let x = self.start.x + time * self.vel.x;
        let y = self.start.y + time * self.vel.y;
        Offset {
            x: x.rem_euclid(GRID_WIDTH),
            y: y.rem_euclid(GRID_HEIGHT),
        }
    }
}

/// Parse an offset value (either position or velocity)
fn parse_offset(input: &mut &str) -> PResult<Offset> {
    seq!(Offset {
        _: one_of(('v', 'p')),
        _: '=',
        x: dec_int,
        _: ',',
        y: dec_int,
    })
    .parse_next(input)
}

/// Parse a robot entry
fn parse_robot(input: &mut &str) -> PResult<Robot> {
    seq!(Robot{
        start: parse_offset,
        _: ' ',
        vel: parse_offset,
    })
    .parse_next(input)
}

/// Print the position of the robots on the grid at a given time
#[allow(unused)]
fn print_robots_at_time(robots: &[Robot], time: isize) {
    let mut grid = vec![vec!['.'; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    for robot in robots {
        let pos = robot.pos_after(time);
        grid[pos.y as usize][pos.x as usize] = '#';
    }
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            print!("{}", grid[y as usize][x as usize]);
        }
        println!();
    }
}

/// Find the variance of the x and y coordinates of the robots at a given time.
///
/// We suppose that a shape must be comprised of a bunch of robots in close proximity, which would give a low variance.
///
/// In reality I did this by printing a lot of grids where a bunch of robots has the same X or Y coordinate
/// and looking at the output visually.
fn robots_location_variance(robots: &[Robot], time: isize) -> (usize, usize) {
    let positions = robots.iter().map(|r| r.pos_after(time)).collect_vec();
    let (mean_x, mean_y) = positions
        .iter()
        .fold((0, 0), |acc, pos| (acc.0 + pos.x, acc.1 + pos.y));
    let (sum_diff_x, sum_diff_y) = positions.iter().fold((0, 0), |acc, pos| {
        (
            acc.0 + (pos.x - mean_x).pow(2),
            acc.1 + (pos.y - mean_y).pow(2),
        )
    });
    (
        sum_diff_x as usize / robots.len(),
        sum_diff_y as usize / robots.len(),
    )
}

impl Day for Day14 {
    type Input = Vec<Robot>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_robot, line_ending).parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 12.03us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|r| r.pos_after(100))
            .counts_by(|pos| pos.quadrant())
            .into_iter()
            .filter_map(|(q, c)| match q {
                Quadrant::None => None,
                _ => Some(c),
            })
            .product()
    }

    type Output2 = usize;

    /// Part 2 took 14.3ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // find a time where the variance of x multiplied by the variance of y is minimal
        let (time, _) = (0..10000)
            .map(|time| (time, robots_location_variance(input, time)))
            .min_by_key(|(_, (x, y))| *x * *y)
            .unwrap();
        // print_robots_at_time(input, time);
        time as usize
    }
}
