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
    /// The sequence of differences between an item and it's predecessor
    fn diffs(&self) -> Diffs {
        Diffs(self.0.iter().tuple_windows().map(|(a, b)| b - a).collect())
    }

    /// All the diff sequences with one item removed each time
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
    /// Whether a sequence of differences matches the criterion for an increasing Report
    fn is_increasing(&self) -> bool {
        self.0.iter().all(|v| (1..=3).contains(v))
    }
    /// Whether a sequence of differences matches the criterion for a decreasing Report
    fn is_decreasing(&self) -> bool {
        self.0.iter().all(|v| (-3..=-1).contains(v))
    }
}

fn parse_report(input: &mut &str) -> PResult<Report> {
    let values: Vec<_> = separated(1.., digit1.parse_to::<i16>(), ' ').parse_next(input)?;
    Ok(Report(values))
}

impl Day for Day02 {
    type Input = Vec<Report>;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        separated(1.., parse_report, line_ending).parse_next(input)
    }

    type Output1 = usize;

    /// Part 1 took 28.03us
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

    /// Part 2 took 191.54us
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
