use std::collections::VecDeque;

use winnow::{
    ascii::{alphanumeric1, line_ending},
    combinator::{alt, separated, separated_pair},
    seq,
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

pub type HashMap<K, T> = std::collections::HashMap<K, T, ahash::RandomState>;

pub struct Day24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
pub struct Gate {
    input0: String,
    input1: String,
    output: String,
    op: Operator,
}

#[derive(Debug, Clone)]
pub struct Device {
    values: HashMap<String, bool>,
    gates: VecDeque<Gate>,
}

impl Device {
    fn execute(&mut self) -> u64 {
        let mut out = 0u64;
        while let Some(gate) = self.gates.pop_front() {
            let Some(input0) = self.values.get(&gate.input0) else {
                self.gates.push_back(gate);
                continue;
            };
            let Some(input1) = self.values.get(&gate.input1) else {
                self.gates.push_back(gate);
                continue;
            };
            let res = match gate.op {
                Operator::And => input0 & input1,
                Operator::Or => input0 | input1,
                Operator::Xor => input0 ^ input1,
            };
            if let Some(bit) = gate
                .output
                .strip_prefix('z')
                .and_then(|n| n.parse::<usize>().ok())
            {
                out |= (res as u64) << bit;
            } else {
                self.values.insert(gate.output, res);
            }
        }
        out
    }
}

fn parse_value(input: &mut &str) -> PResult<(String, bool)> {
    separated_pair(
        alphanumeric1.map(|n: &str| n.to_string()),
        ": ",
        one_of(('0', '1')).map(|c: char| c != '0'),
    )
    .parse_next(input)
}

fn parse_values(input: &mut &str) -> PResult<HashMap<String, bool>> {
    separated(1.., parse_value, line_ending).parse_next(input)
}

fn parse_gate(input: &mut &str) -> PResult<Gate> {
    seq!(Gate {
        input0: alphanumeric1.map(|n: &str| n.to_string()),
        op: alt((" AND ", " OR ", " XOR ")).map(|op: &str| match op {
            " AND " => Operator::And,
            " OR " => Operator::Or,
            " XOR " => Operator::Xor,
            _ => unimplemented!(),
        }),
        input1: alphanumeric1.map(|n: &str| n.to_string()),
        _: " -> ",
        output: alphanumeric1.map(|n: &str| n.to_string())
    })
    .parse_next(input)
}

fn parse_gates(input: &mut &str) -> PResult<Vec<Gate>> {
    separated(1.., parse_gate, line_ending).parse_next(input)
}

impl Day for Day24 {
    type Input = Device;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let (values, gates) =
            separated_pair(parse_values, "\n\n", parse_gates).parse_next(input)?;
        Ok(Device {
            values,
            gates: VecDeque::from(gates),
        })
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut device = input.clone();
        device.execute()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
