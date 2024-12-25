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
            let bit = match gate.op {
                Operator::And => input0 & input1,
                Operator::Or => input0 | input1,
                Operator::Xor => input0 ^ input1,
            };
            if let Some(pos) = gate
                .output
                .strip_prefix('z')
                .and_then(|n| n.parse::<usize>().ok())
            {
                out |= (bit as u64) << pos;
            } else {
                self.values.insert(gate.output, bit);
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

    /// Part 1 took 97.8us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut device = input.clone();
        device.execute()
    }

    type Output2 = String;

    /// Part 2 took 69.5us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut to_swap = Vec::new();
        for gate in &input.gates {
            match gate.op {
                Operator::Xor => {
                    // XOR gates which combine X and Y into a intermediary value, have their output be the input to an
                    // AND and a XOR gate (except first one).
                    // Other XOR gates should output a Z (except for z00).
                    if gate.input0.starts_with('x') || gate.input1.starts_with('x') {
                        // these should not output a z
                        let is_first = gate.input0 == "x00" || gate.input1 == "x00";
                        if is_first {
                            if gate.output != "z00" {
                                to_swap.push(gate.output.clone());
                            }
                            continue;
                        } else if gate.output == "z00" {
                            to_swap.push(gate.output.clone());
                            continue;
                        }
                        // the output should not be z
                        if gate.output.starts_with('z') {
                            to_swap.push(gate.output.clone());
                            continue;
                        }
                        // the output should not be the input to an OR gate
                        if input.gates.iter().any(|g| {
                            (g.input0 == gate.output || g.input1 == gate.output)
                                && g.op == Operator::Or
                        }) {
                            to_swap.push(gate.output.clone());
                            continue;
                        }
                    } else {
                        // these should output a z
                        if !gate.output.starts_with('z') {
                            to_swap.push(gate.output.clone());
                            continue;
                        }
                    }
                }
                Operator::And => {
                    // AND gates which combine X and Y into a value should have that value OR'd (except first one)
                    if (gate.input0.starts_with('x') && gate.input1.starts_with('y'))
                        || (gate.input0.starts_with('y') && gate.input1.starts_with('x'))
                    {
                        let is_first = gate.input0 == "x00" || gate.input1 == "x00";
                        if !is_first
                            && !input.gates.iter().any(|g| {
                                (g.input0 == gate.output || g.input1 == gate.output)
                                    && g.op == Operator::Or
                            })
                        {
                            to_swap.push(gate.output.clone());
                            continue;
                        }
                    }
                }
                Operator::Or => {}
            }
            // check gates which output z and make sure they are XOR (except last one)
            if gate.output.starts_with('z') {
                let is_last = gate.output == "z45";
                if is_last {
                    if gate.op != Operator::Or {
                        to_swap.push(gate.output.clone());
                    }
                    continue;
                } else if gate.op != Operator::Xor {
                    to_swap.push(gate.output.clone());
                    continue;
                }
            }
        }
        to_swap.sort_unstable();
        to_swap.join(",")
    }
}
