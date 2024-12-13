use winnow::{
    ascii::{digit1, line_ending},
    combinator::{alt, separated, terminated},
    seq, PResult, Parser as _,
};

use crate::days::Day;

pub struct Day13;

#[derive(Debug, Clone)]
pub struct Offset {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
pub struct Claw {
    a: Offset,
    b: Offset,
    prize: Offset,
}

impl Claw {
    /// Calculate the minimum amount of tokens necessary to win the prize
    ///
    /// For each claw, we have a system of equations:
    /// Px = a * Ax + b Bx; Py = a * Ay + b * By;
    /// By solving it, we can extract values of `a` (presses of A) and `b` (pressed of B) which reach the prize
    /// location.
    /// If those are integers, then we can reach the prize, otherwise we can't. In practice, we check this by putting
    /// the values back into the two equations and checking the equality.
    fn tokens(&self) -> Option<usize> {
        let b = (self.a.y * self.prize.x - self.a.x * self.prize.y)
            / (self.a.y * self.b.x - self.b.y * self.a.x);
        let a = (self.prize.x - b * self.b.x) / self.a.x;
        if a > 0
            && b > 0
            && a * self.a.x + b * self.b.x == self.prize.x
            && a * self.a.y + b * self.b.y == self.prize.y
        {
            Some(a as usize * 3 + b as usize)
        } else {
            None
        }
    }

    /// For part 2, we need to add a constant to the prize position
    fn part2(&self) -> Self {
        let mut new_claw = self.clone();
        new_claw.prize.x += 10000000000000;
        new_claw.prize.y += 10000000000000;
        new_claw
    }
}

/// Parse a button definition into an [`Offset`]
fn parse_button(input: &mut &str) -> PResult<Offset> {
    terminated(
        seq!(Offset {
            _: alt(("Button A: X+", "Button B: X+")),
            x: digit1.parse_to::<isize>(),
            _: ", Y+",
            y: digit1.parse_to::<isize>()
        }),
        line_ending,
    )
    .parse_next(input)
}

/// Parse a prize location definition into an [`Offset`]
fn parse_prize(input: &mut &str) -> PResult<Offset> {
    seq!(Offset {
        _: "Prize: X=",
        x: digit1.parse_to::<isize>(),
        _: ", Y=",
        y: digit1.parse_to::<isize>()
    })
    .parse_next(input)
}

impl Day for Day13 {
    type Input = Vec<Claw>;

    /// Parse the list of claw machines into a list
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(
            1..,
            seq!(Claw {
                a: parse_button,
                b: parse_button,
                prize: parse_prize,
            }),
            (line_ending, line_ending),
        )
        .parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 4.53us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|c| c.tokens().unwrap_or_default()).sum()
    }

    type Output2 = usize;

    /// Part 2 took 4.37us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|c| c.part2().tokens().unwrap_or_default())
            .sum()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        let parsed = Day13::parser(&mut INPUT).unwrap();
        assert_eq!(Day13::part_1(&parsed), 480);
    }
}
