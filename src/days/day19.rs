use std::ops::Deref;

use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated},
    seq,
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

pub struct Day19;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern(Vec<Color>);

impl Deref for Pattern {
    type Target = Vec<Color>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    available: Vec<Pattern>,
    desired: Vec<Pattern>,
}

fn parse_pattern(input: &mut &str) -> PResult<Pattern> {
    Ok(Pattern(
        repeat(
            1..,
            one_of(('w', 'u', 'b', 'r', 'g')).map(|c| match c {
                'w' => Color::White,
                'u' => Color::Blue,
                'b' => Color::Black,
                'r' => Color::Red,
                'g' => Color::Green,
                _ => unimplemented!(),
            }),
        )
        .parse_next(input)?,
    ))
}

fn parse_available(input: &mut &str) -> PResult<Vec<Pattern>> {
    separated(1.., parse_pattern, ", ").parse_next(input)
}

fn parse_desired(input: &mut &str) -> PResult<Vec<Pattern>> {
    separated(1.., parse_pattern, line_ending).parse_next(input)
}

impl Day for Day19 {
    type Input = Puzzle;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        seq!(Puzzle {
            available: parse_available,
            _: "\n\n",
            desired: parse_desired
        })
        .parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        println!("{input:?}");
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
