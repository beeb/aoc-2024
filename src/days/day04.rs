use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: isize = 140;
const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [[isize; 2]; 8] = [
    [0, -1],  // up
    [1, -1],  // top right
    [1, 0],   // right
    [1, 1],   // bottom right
    [0, 1],   // down
    [-1, 1],  // bottom left
    [-1, 0],  // left
    [-1, -1], // top left
];

pub struct Day04;

#[derive(Debug)]
pub struct Row {
    cells: Vec<char>,
}

#[derive(Debug)]
pub struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    /// Retrieve the letter at coordinate (x, y) or `None` if OOB.
    fn get_letter(&self, x: isize, y: isize) -> Option<&char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.rows
            .get(y as usize)
            .and_then(|row| row.cells.get(x as usize))
    }

    /// Check if coordinate (x, y) contains letter. `None` if OOB.
    fn is_letter(&self, x: isize, y: isize, letter: &char) -> Option<bool> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get_letter(x, y).map(|cell| cell == letter)
    }

    /// Search for the word starting at coordinate (col, row) in a given direction.
    fn search_dir(&self, col: isize, row: isize, dir: &[isize; 2]) -> bool {
        for (i, c) in WORD.iter().enumerate().skip(1) {
            let x = col + dir[0] * i as isize;
            let y = row + dir[1] * i as isize;
            if let Some(true) = self.is_letter(x, y, c) {
                continue;
            }
            return false;
        }
        true
    }

    /// Search for the X-MAS cross at a coordinate (x, y).
    fn search_cross(&self, x: isize, y: isize) -> bool {
        match (self.get_letter(x - 1, y - 1), self.get_letter(x + 1, y + 1)) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => {}
            _ => {
                return false;
            }
        }
        match (self.get_letter(x + 1, y - 1), self.get_letter(x - 1, y + 1)) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => {}
            _ => {
                return false;
            }
        }
        true
    }
}

/// Parse a line of the input file into a [`Row`].
fn parse_row(input: &mut &str) -> PResult<Row> {
    let letters = repeat(1.., one_of('A'..='z')).parse_next(input)?;
    Ok(Row { cells: letters })
}

impl Day for Day04 {
    type Input = Grid;

    /// Parser took 119.7us
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let rows = separated(1.., parse_row, line_ending).parse_next(input)?;
        Ok(Grid { rows })
    }

    type Output1 = usize;

    /// Part 1 took 322.3us
    fn part_1(grid: &Self::Input) -> Self::Output1 {
        let mut count = 0;
        // find a grid cell which contains the first letter of the word, and then search in all directions
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some(true) = grid.is_letter(x, y, &WORD[0]) {
                    count += DIRS.iter().filter(|dir| grid.search_dir(x, y, dir)).count();
                }
            }
        }
        count
    }

    type Output2 = usize;

    /// Part 2 took 115.8us
    fn part_2(grid: &Self::Input) -> Self::Output2 {
        let mut count = 0;
        // find a grid cell which contains the 'A' cross center, then search neighboring cells for the right letters
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some(true) = grid.is_letter(x, y, &'A') {
                    if grid.search_cross(x, y) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
