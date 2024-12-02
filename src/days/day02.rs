use itertools::Itertools;
use winnow::{
    ascii::{digit1, line_ending},
    combinator::separated,
    PResult, Parser as _,
};

use crate::days::Day;

pub struct Day02;

pub struct Report(Vec<i16>);
pub struct Diffs(Vec<i16>);

impl Report {
    fn diffs(&self) -> Diffs {
        Diffs(self.0.iter().tuple_windows().map(|(a, b)| b - a).collect())
    }
    fn all_diffs(&self) -> Vec<Diffs> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut values = self.0.clone();
                values.remove(i);
                Report(values).diffs()
            })
            .collect()
    }
}

impl Diffs {
    fn is_increasing(&self) -> bool {
        self.0.iter().all(|v| (1..=3).contains(v))
    }
    fn is_decreasing(&self) -> bool {
        self.0.iter().all(|v| (-3..=-1).contains(v))
    }
}

fn parse_line(input: &mut &str) -> PResult<Report> {
    let values: Vec<_> = separated(1.., digit1.parse_to::<i16>(), ' ').parse_next(input)?;
    Ok(Report(values))
}

impl Day for Day02 {
    type Input = Vec<Report>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_line, line_ending).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|report| {
                let diffs = report.diffs();
                diffs.is_increasing() || diffs.is_decreasing()
            })
            .count()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|report| {
                report
                    .all_diffs()
                    .iter()
                    .any(|diffs| diffs.is_increasing() || diffs.is_decreasing())
            })
            .count()
    }
}
