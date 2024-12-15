use std::ops::{Deref, DerefMut};

use winnow::{
    ascii::line_ending,
    combinator::{opt, repeat, separated, separated_pair},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

const GRID_SIZE: usize = if cfg!(test) { 10 } else { 50 };

pub type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

pub struct Day15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Floor,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn parse_row(input: &mut &str) -> PResult<Vec<Tile>> {
    repeat(
        1..,
        one_of(('#', '.', 'O', '@')).map(|c: char| match c {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            'O' => Tile::BoxLeft,
            '@' => Tile::Robot,
            _ => unimplemented!(),
        }),
    )
    .parse_next(input)
}

fn parse_grid(input: &mut &str) -> PResult<Grid> {
    let tiles: Vec<Vec<_>> = separated(1.., parse_row, line_ending).parse_next(input)?;
    Ok(Grid(tiles))
}

fn parse_move(input: &mut &str) -> PResult<Move> {
    one_of(('^', '>', 'v', '<'))
        .map(|c: char| match c {
            '^' => Move::Up,
            '>' => Move::Right,
            'v' => Move::Down,
            '<' => Move::Left,
            _ => unimplemented!(),
        })
        .parse_next(input)
}

fn parse_moves(input: &mut &str) -> PResult<Vec<Move>> {
    let res: Vec<_> = repeat(1.., (parse_move, opt(line_ending))).parse_next(input)?;
    let (moves, _): (Vec<_>, Vec<_>) = res.into_iter().unzip();
    Ok(moves)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    /// Apply a move to find the new coordinates
    ///
    /// The current location must be inside the border.
    fn with_move(&self, mov: Move) -> Loc {
        match mov {
            Move::Up => Loc {
                x: self.x,
                y: self.y - 1,
            },
            Move::Right => Loc {
                x: self.x + 1,
                y: self.y,
            },
            Move::Down => Loc {
                x: self.x,
                y: self.y + 1,
            },
            Move::Left => Loc {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid(Vec<Vec<Tile>>);

impl Deref for Grid {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Grid {
    /// Find the start position of the robot
    fn start_pos(&self) -> Loc {
        self.iter()
            .flatten()
            .enumerate()
            .find_map(|(i, tile)| {
                if *tile == Tile::Robot {
                    Some(Loc {
                        x: i % GRID_SIZE,
                        y: i / GRID_SIZE,
                    })
                } else {
                    None
                }
            })
            .unwrap()
    }

    /// Get the tile type at coordinates `x` and `y`
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.0.get(y).and_then(|row| row.get(x))
    }

    /// Get the tile type at location `loc`
    fn get_loc(&self, loc: &Loc) -> Option<&Tile> {
        self.0.get(loc.y).and_then(|row| row.get(loc.x))
    }

    /// Calculate the GPS score
    fn gps_score(&self, part2: bool) -> usize {
        self.iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, tile)| {
                if *tile == Tile::BoxLeft {
                    let x = if part2 {
                        i % (2 * GRID_SIZE)
                    } else {
                        i % GRID_SIZE
                    };
                    let y = if part2 {
                        i / (2 * GRID_SIZE)
                    } else {
                        i / GRID_SIZE
                    };
                    Some(y * 100 + x)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    grid: Grid,
    robot: Loc,
    moves: Vec<Move>,
}

impl Puzzle {
    /// Move the robot (part1) one step in the direction indicated by `mov` if possible
    fn move_robot(&mut self, mov: Move) {
        // check the next tile in the direction of the move
        let neighbor = self.robot.with_move(mov);
        match self.grid.get_loc(&neighbor) {
            Some(&Tile::Wall) | None => {
                // robot can't move
            }
            Some(&Tile::Floor) => {
                // robot can move but no box moves
                self.robot = neighbor;
            }
            Some(&Tile::BoxLeft) => {
                // the neighbor is a box, let's see if there's an empty tile after all the in-line boxes
                let iter: Box<dyn Iterator<Item = usize>> = match mov {
                    Move::Up => Box::new((0..self.robot.y).rev()),
                    Move::Right => Box::new((self.robot.x + 1)..GRID_SIZE),
                    Move::Down => Box::new((self.robot.y + 1)..GRID_SIZE),
                    Move::Left => Box::new((0..self.robot.x).rev()),
                };
                let next = match mov {
                    Move::Up | Move::Down => {
                        let mut iter = iter.skip_while(|y| {
                            self.grid.get(self.robot.x, *y) == Some(&Tile::BoxLeft)
                        });
                        iter.next().and_then(|y| {
                            if self.grid.get(self.robot.x, y) == Some(&Tile::Floor) {
                                Some(Loc { x: self.robot.x, y })
                            } else {
                                None
                            }
                        })
                    }
                    Move::Right | Move::Left => {
                        let mut iter = iter.skip_while(|x| {
                            self.grid.get(*x, self.robot.y) == Some(&Tile::BoxLeft)
                        });
                        iter.next().and_then(|x| {
                            if self.grid.get(x, self.robot.y) == Some(&Tile::Floor) {
                                Some(Loc { x, y: self.robot.y })
                            } else {
                                None
                            }
                        })
                    }
                };
                let Some(next) = next else {
                    // robot can't move
                    return;
                };
                self.grid[neighbor.y][neighbor.x] = Tile::Floor; // will actually be the robot
                self.grid[next.y][next.x] = Tile::BoxLeft;
                self.robot = neighbor;
            }
            Some(&Tile::Robot | &Tile::BoxRight) => unreachable!(),
        }
    }

    /// Check whether the robot can move in the `mov` direction with BFS
    ///
    /// Each box affected by the robot's move must be able to move itself.
    fn can_move(&self, mov: Move) -> bool {
        let mut stack = Vec::new();
        let robot_loc = self.robot.with_move(mov);
        stack.push(robot_loc.clone());
        let mut seen = HashSet::default();
        seen.insert(robot_loc);
        while let Some(loc) = stack.pop() {
            // check whether we're in bounds
            let Some(tile) = self.grid.get_loc(&loc) else {
                continue;
            };
            match tile {
                Tile::Floor => {}
                Tile::Wall => {
                    // if at any point any affected box or the robot is facing a wall, we can't move the robot
                    return false;
                }
                Tile::BoxLeft => {
                    // if we try to move the left part of a box, the right part must be able to move too
                    let next = loc.with_move(mov);
                    let right = loc.with_move(Move::Right);
                    if next != right && seen.insert(next.clone()) {
                        stack.push(next);
                    }
                    if seen.insert(right.clone()) {
                        stack.push(right);
                    }
                }
                Tile::BoxRight => {
                    // if we try to move the right part of a box, the left part must be able to move too
                    let next = loc.with_move(mov);
                    let left = loc.with_move(Move::Left);
                    if next != left && seen.insert(next.clone()) {
                        stack.push(next);
                    }
                    if seen.insert(left.clone()) {
                        stack.push(left);
                    }
                }
                Tile::Robot => unreachable!(),
            }
        }
        true
    }

    /// Move the robot (part2) in the direction of `mov`
    ///
    /// This recursively moves boxes which are neighboring the robot or other boxes affected by the move.
    fn move_robot_part2(&mut self, mov: Move) {
        // check whether we can move in the direction of `mov`
        if !self.can_move(mov) {
            return;
        }
        let neighbor = self.robot.with_move(mov);
        let tile = self.grid.get_loc(&neighbor).unwrap();
        // check the neighbor
        match tile {
            Tile::Floor => {
                // simple move without affecting any box
                self.robot = neighbor;
            }
            Tile::BoxLeft => {
                // if the neighbor is the left part of a box, we first move that box
                self.move_box(&neighbor, mov);
                self.robot = neighbor;
            }
            Tile::BoxRight => {
                // if the neighbor is the right part of a box, we first move that box
                // we always pass the location of the left part of a box
                self.move_box(&neighbor.with_move(Move::Left), mov);
                self.robot = neighbor;
            }
            Tile::Wall | Tile::Robot => unreachable!(),
        }
    }

    /// Move a box in the direction of `mov`
    ///
    /// This recursively moves boxes which are affected by this move.
    fn move_box(&mut self, loc: &Loc, mov: Move) {
        // `loc` is always the left part of the box
        // the neighbors are the two tiles which will hold the box we're currently moving
        let neighbor = loc.with_move(mov); // this could be the right part of the box itself
        let neighbor2 = loc.with_move(Move::Right).with_move(mov); // this could be the left part of the box itself
        let tile = self.grid.get_loc(&neighbor).unwrap();
        let tile2 = self.grid.get_loc(&neighbor2).unwrap();
        match (mov, tile, tile2) {
            (Move::Up | Move::Down, &Tile::Floor, &Tile::Floor)
            | (Move::Left, &Tile::Floor, _)
            | (Move::Right, _, &Tile::Floor) => {
                // can move, no recursion needed
                self.grid[neighbor.y][neighbor.x] = Tile::BoxLeft;
                self.grid[neighbor2.y][neighbor2.x] = Tile::BoxRight;
            }
            (Move::Up | Move::Down, &Tile::BoxLeft, _) => {
                // aligned with another box vertically, we must move that box first
                self.move_box(&neighbor, mov);
                self.grid[neighbor.y][neighbor.x] = Tile::BoxLeft;
                self.grid[neighbor2.y][neighbor2.x] = Tile::BoxRight;
            }
            (Move::Left, &Tile::BoxRight, _)
            | (Move::Up | Move::Down, &Tile::BoxRight, &Tile::Floor) => {
                // aligned with another box horizontally or another box above/below the left side
                // we must move it first
                self.move_box(&neighbor.with_move(Move::Left), mov);
                self.grid[neighbor.y][neighbor.x] = Tile::BoxLeft;
                self.grid[neighbor2.y][neighbor2.x] = Tile::BoxRight;
            }
            (Move::Right, _, &Tile::BoxLeft)
            | (Move::Up | Move::Down, &Tile::Floor, &Tile::BoxLeft) => {
                // aligned with another box horizontally or another box above/below the right side
                // we must move it first
                self.move_box(&neighbor2, mov);
                self.grid[neighbor.y][neighbor.x] = Tile::BoxLeft;
                self.grid[neighbor2.y][neighbor2.x] = Tile::BoxRight;
            }
            (Move::Up | Move::Down, &Tile::BoxRight, &Tile::BoxLeft) => {
                // two boxes above/below, we must move them first
                self.move_box(&neighbor.with_move(Move::Left), mov);
                self.move_box(&neighbor2, mov);
                self.grid[neighbor.y][neighbor.x] = Tile::BoxLeft;
                self.grid[neighbor2.y][neighbor2.x] = Tile::BoxRight;
            }
            _ => {}
        }
        // free the tile(s) which do not have the box we've just moved anymore
        if mov == Move::Up || mov == Move::Down || mov == Move::Right {
            self.grid[loc.y][loc.x] = Tile::Floor;
        }
        if mov == Move::Up || mov == Move::Down || mov == Move::Left {
            self.grid[loc.y][loc.x + 1] = Tile::Floor;
        }
    }

    /// Expand the part1 grid into its wider part2 variant
    fn expand(&self) -> Self {
        let mut out = Vec::new();
        for row in self.grid.iter() {
            let mut new_row = Vec::new();
            for tile in row {
                match tile {
                    Tile::Floor | Tile::Robot => new_row.extend([Tile::Floor, Tile::Floor]),
                    Tile::Wall => new_row.extend([Tile::Wall, Tile::Wall]),
                    Tile::BoxLeft => new_row.extend([Tile::BoxLeft, Tile::BoxRight]),
                    Tile::BoxRight => unreachable!(),
                }
            }
            out.push(new_row);
        }
        Self {
            grid: Grid(out),
            robot: Loc {
                x: self.robot.x * 2,
                y: self.robot.y,
            },
            moves: self.moves.clone(),
        }
    }

    /// Print the grid with the current state
    #[allow(unused)]
    fn print(&self, part2: bool) {
        let width = if part2 { GRID_SIZE * 2 } else { GRID_SIZE };
        for y in 0..GRID_SIZE {
            for x in 0..width {
                if self.robot.x == x && self.robot.y == y {
                    print!("@");
                    continue;
                }
                match self.grid.get(x, y).unwrap() {
                    Tile::Floor => print!("."),
                    Tile::Wall => print!("#"),
                    Tile::BoxLeft => {
                        if part2 {
                            print!("[")
                        } else {
                            print!("#")
                        }
                    }
                    Tile::BoxRight => print!("]"),
                    Tile::Robot => print!("@"),
                }
            }
            println!();
        }
    }
}

impl Day for Day15 {
    type Input = Puzzle;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let (mut grid, moves) =
            separated_pair(parse_grid, "\n\n", parse_moves).parse_next(input)?;
        // extract robot position data
        let start_pos = grid.start_pos();
        grid[start_pos.y][start_pos.x] = Tile::Floor;
        Ok(Puzzle {
            grid,
            robot: start_pos,
            moves,
        })
    }

    type Output1 = usize;

    /// Part 1 took 214us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut data = input.clone();
        for mov in data.moves.clone() {
            data.move_robot(mov);
        }
        // data.print(false);
        data.grid.gps_score(false)
    }

    type Output2 = usize;

    /// Part 2 took 1.40ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut data = input.clone().expand();
        for mov in data.moves.clone() {
            data.move_robot_part2(mov);
        }
        // data.print(true);
        data.grid.gps_score(true)
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        let parsed = Day15::parser(&mut INPUT).unwrap();
        assert_eq!(Day15::part_1(&parsed), 10092);
    }

    #[test]
    fn test_part2() {
        let parsed = Day15::parser(&mut INPUT).unwrap();
        assert_eq!(Day15::part_2(&parsed), 9021);
    }
}
