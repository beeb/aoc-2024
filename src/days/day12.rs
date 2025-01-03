use itertools::Itertools as _;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: usize = if cfg!(test) { 4 } else { 140 };
// up - right - down - left
const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
// top-right - bottom-right - bottom-left - top-left
const DIAGONALS: [(isize, isize); 4] = [(1, -1), (1, 1), (-1, 1), (-1, -1)];

pub type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

pub struct Day12;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Point {
    y: isize,
    x: isize,
}

impl Point {
    /// Retrieve the crop type at this point, or `None` if out of bounds
    fn crop<'a>(&self, map: &'a [Vec<char>]) -> Option<&'a char> {
        map.get(self.y as usize)
            .and_then(|row| row.get(self.x as usize))
    }

    /// Get the neighbors of this point which have the same crop type
    fn neighbors(&self, map: &[Vec<char>]) -> Vec<Point> {
        let crop = self.crop(map);
        DIRS.iter()
            .map(|(dx, dy)| Point {
                x: self.x + dx,
                y: self.y + dy,
            })
            .filter(|p| p.crop(map) == crop)
            .collect()
    }

    /// Count how many convex and concave corners are bordering this plot
    fn count_corners(&self, map: &[Vec<char>]) -> usize {
        let mut corners = 0;
        let crop = self.crop(map);
        // convex corners, retrieve the symbol at each cardinal point
        let neighbors = DIRS
            .iter()
            .map(|(dx, dy)| {
                Point {
                    x: self.x + dx,
                    y: self.y + dy,
                }
                .crop(map)
            })
            .collect_vec();
        // if two consecutive neighbors (turning clockwise in this case) are both different from the current plot,
        // we have a convex corner
        corners += neighbors
            .iter()
            .cycle()
            .tuple_windows()
            .take(4)
            .filter(|(&a, &b)| a != crop && b != crop)
            .count();
        // concave corners, retrieve the diagonals
        let diagonals = DIAGONALS
            .iter()
            .map(|(dx, dy)| {
                Point {
                    x: self.x + dx,
                    y: self.y + dy,
                }
                .crop(map)
            })
            .collect_vec();
        // if two consecutive neighbors are both the same as the current plot, and the corner in-between is different,
        // then we have a concave corner
        corners += neighbors
            .iter()
            .cycle()
            .interleave(diagonals.iter().cycle())
            .tuple_windows()
            .step_by(2)
            .take(4)
            .filter(|(&a, &b, &c)| a == crop && b != crop && c == crop)
            .count();
        corners
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    points: HashSet<Point>,
    perimeter: usize,
    corners: usize,
}

impl Region {
    /// Check whether the region contains a given plot
    fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }
}

/// Use a BFS flooding algorithm to find all the plots belonging to the same region as `start`
///
/// At the same time, calculate the number of corners in the region, as well as the perimeter length.
fn bfs_flood(start: &Point, map: &[Vec<char>], part2: bool) -> Region {
    let mut perimeter = 0;
    let mut corners = 0;
    let mut region = HashSet::default();
    let mut stack = Vec::new();
    stack.push(*start);
    while let Some(plot) = stack.pop() {
        let neighbors = plot.neighbors(map);
        let perimeter_increase = 4 - neighbors.len();
        stack.extend(neighbors.into_iter().filter(|p| !region.contains(p)));
        if region.insert(plot) {
            perimeter += perimeter_increase;
            if part2 {
                corners += plot.count_corners(map);
            }
        }
    }
    Region {
        points: region,
        perimeter,
        corners,
    }
}

/// Create a list of all regions
fn get_regions(map: &[Vec<char>], part2: bool) -> Vec<Region> {
    let mut regions = Vec::<Region>::new();
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let point = Point {
                x: x as isize,
                y: y as isize,
            };
            if regions.iter().any(|r| r.contains(&point)) {
                continue;
            }
            regions.push(bfs_flood(&point, map, part2));
        }
    }
    regions
}

/// Parse a line of the input
fn parse_line(input: &mut &str) -> PResult<Vec<char>> {
    repeat(1.., one_of('A'..='Z')).parse_next(input)
}

impl Day for Day12 {
    type Input = Vec<Vec<char>>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_line, line_ending).parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 15.3ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let regions = get_regions(input, false);
        regions
            .into_iter()
            .map(|r| r.points.len() * r.perimeter)
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 16.7ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let regions = get_regions(input, true);
        regions
            .into_iter()
            .map(|r| r.points.len() * r.corners)
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn test_part1() {
        let parsed = Day12::parser(&mut INPUT).unwrap();
        assert_eq!(Day12::part_1(&parsed), 140);
    }

    #[test]
    fn test_part2() {
        let parsed = Day12::parser(&mut INPUT).unwrap();
        assert_eq!(Day12::part_2(&parsed), 80);
    }
}
