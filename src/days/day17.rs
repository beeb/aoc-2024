use itertools::Itertools as _;
use winnow::{
    ascii::{dec_uint, digit1, line_ending},
    combinator::{preceded, separated, separated_pair},
    token::one_of,
    PResult, Parser as _,
};

use crate::days::Day;

pub struct Day17;

/// A combo operator, either a literal value or the value of a register
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ComboOp {
    Lit(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

/// An instruction for the VM
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instruction {
    Adv(ComboOp), // divide A by 2**op -> A
    Bxl(u8),      // XOR B with literal op
    Bst(ComboOp), // op % 8 -> B
    Jnz(u8),      // do nothing if A is 0, otherwise jump to instruction
    Bxc,          // XOR of B and C -> B
    Out(ComboOp), // op % 8 -> output
    Bdv(ComboOp), // divide A by 2**op -> B
    Cdv(ComboOp), // divide A by 2**op -> C
}

/// The state of the VM comprised of 3 registers and an instruction pointer
#[derive(Debug, Clone)]
pub struct State {
    a: usize,
    b: usize,
    c: usize,
    pointer: usize,
    instructions: Vec<Instruction>,
    orig: Vec<u8>,
}

impl State {
    /// Clone the state while changing the initial value of the A register
    fn with_register(&self, a: usize) -> Self {
        let clone = self.clone();
        Self { a, ..clone }
    }

    /// Get the value for a combo operator, either a literal or the current value of a register
    fn get_op_value(&self, op: ComboOp) -> usize {
        match op {
            ComboOp::Lit(x) => x as usize,
            ComboOp::RegisterA => self.a,
            ComboOp::RegisterB => self.b,
            ComboOp::RegisterC => self.c,
        }
    }
}

impl Iterator for State {
    type Item = u8;

    /// Process one or more instructions and advance the state of the VM, until an output is generated
    fn next(&mut self) -> Option<Self::Item> {
        let instr = self.instructions.get(self.pointer)?;
        match instr {
            Instruction::Adv(op) => self.a >>= self.get_op_value(*op),
            Instruction::Bxl(x) => self.b ^= *x as usize,
            Instruction::Bst(op) => self.b = self.get_op_value(*op) % 8,
            Instruction::Jnz(x) => {
                if self.a > 0 {
                    self.pointer = *x as usize;
                    return self.next();
                }
            }
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Out(op) => {
                self.pointer += 1;
                return Some((self.get_op_value(*op) % 8) as u8);
            }
            Instruction::Bdv(op) => self.b = self.a >> self.get_op_value(*op),
            Instruction::Cdv(op) => self.c = self.a >> self.get_op_value(*op),
        }
        self.pointer += 1;
        self.next()
    }
}

/// Recursively find a program input that yields the program itself
///
/// To solve this part, we must first analyze the behavior of the input program and note the following:
/// - the program contains a jump instruction at the end which returns to the first instruction until register A is zero
/// - this means the program is one main loop
/// - there is only 1 instruction which can alter the value of the A register (ADV)
/// - in my case, this instruction divides the value of the A register by 8 (2^3) once per loop iteration
/// - dividing by 8 is equivalent to discarding the 3 lowest bit of the value of the A register (shifting right 3 bits)
/// - by printing the output for initial A register values between 0 and 0b111111 we can see a pattern emerging, whereby
///   the first 3 bits of the input dictate the last output value of the program
/// - likewise, the 3 bits after that dictate the one-before-last output value
///
/// We can thus try all 8 possible values for a sequence of 3 bits appended at the end of the A register value and
/// find which ones (there may be multiple) give us a output matching the end of the original program.
/// By recursively trying to add 3 bits to the A register until we have a perfect match for the full length of the input
/// program, we find the answer.
fn find_input(input: &State, a: usize, i: usize) -> Option<usize> {
    let res = input.with_register(a).collect_vec();
    // if the output matches the program, we found the solution!
    if res == input.orig {
        return Some(a);
    }
    let start = input.orig.len() - i;
    // compare the (partial) output to the end of the original program
    if res == input.orig[start..] || i == 0 {
        // if we have a partial match, we try to append each possible 3-bit number to the input value
        for n in 0..=0b111 {
            if let Some(sol) = find_input(input, (a << 3) + n, i + 1) {
                // if we have a match, it means we found a correct value for those bits
                return Some(sol);
            }
        }
    }
    None
}

/// Parse the initial value for a register
fn parse_register(input: &mut &str) -> PResult<usize> {
    let (_, _, _, reg) = (
        "Register ",
        one_of(('A', 'B', 'C')),
        ": ",
        digit1.parse_to(),
    )
        .parse_next(input)?;
    Ok(reg)
}

/// Parse the 3 registers' initial values
fn parse_registers(input: &mut &str) -> PResult<(usize, usize, usize)> {
    let registers: Vec<_> = separated(3, parse_register, line_ending).parse_next(input)?;
    Ok(registers.into_iter().collect_tuple().unwrap())
}

/// Parse the raw bytecode of the program
fn parse_instructions(input: &mut &str) -> PResult<Vec<u8>> {
    preceded("Program: ", separated(1.., dec_uint::<_, u8, _>, ',')).parse_next(input)
}

impl Day for Day17 {
    type Input = State;

    /// Transform the raw bytecode into a nice typed definition of the program and state
    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let (registers, instructions) =
            separated_pair(parse_registers, "\n\n", parse_instructions).parse_next(input)?;
        let instructions_parsed = instructions
            .chunks_exact(2)
            .map(|instr| {
                let op = match instr[1] {
                    0..=3 => ComboOp::Lit(instr[1]),
                    4 => ComboOp::RegisterA,
                    5 => ComboOp::RegisterB,
                    6 => ComboOp::RegisterC,
                    _ => unimplemented!(),
                };
                match instr[0] {
                    0 => Instruction::Adv(op),
                    1 => Instruction::Bxl(instr[1]),
                    2 => Instruction::Bst(op),
                    3 => Instruction::Jnz(instr[1]),
                    4 => Instruction::Bxc,
                    5 => Instruction::Out(op),
                    6 => Instruction::Bdv(op),
                    7 => Instruction::Cdv(op),
                    _ => unimplemented!(),
                }
            })
            .collect();
        Ok(State {
            a: registers.0,
            b: registers.1,
            c: registers.2,
            pointer: 0,
            instructions: instructions_parsed,
            orig: instructions,
        })
    }

    type Output1 = String;

    /// Part 1 took 2.3us
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.clone().map(|n| n.to_string()).join(",")
    }

    type Output2 = usize;

    /// Part 2 took 104.1us
    fn part_2(input: &Self::Input) -> Self::Output2 {
        find_input(input, 0, 0).unwrap()
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part1() {
        let parsed = Day17::parser(&mut INPUT).unwrap();
        assert_eq!(Day17::part_1(&parsed), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_part2() {
        let parsed = Day17::parser(&mut INPUT2).unwrap();
        assert_eq!(Day17::part_2(&parsed), 117440);
    }
}
