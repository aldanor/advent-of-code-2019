use std::iter;

use aoc19::parse_ints;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Param {
    Position,
    Immediate,
}

impl From<i64> for Param {
    fn from(value: i64) -> Self {
        match value {
            0 => Param::Position,
            1 => Param::Immediate,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Write(i64),
    Output(i64),
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    Exit,
}

impl Op {
    pub fn n_in(&self) -> usize {
        match self {
            Op::Add | Op::Multiply => 2,
            Op::Output => 1,
            _ => 0,
        }
    }

    pub fn has_out(&self) -> bool {
        match self {
            Op::Add | Op::Multiply | Op::Input => true,
            _ => false,
        }
    }

    pub fn n_params(&self) -> usize {
        self.n_in() + (self.has_out() as usize)
    }

    pub fn apply(&self, inputs: &[i64], input: i64) -> Outcome {
        match self {
            Op::Add => Outcome::Write(inputs[0] + inputs[1]),
            Op::Multiply => Outcome::Write(inputs[0] * inputs[1]),
            Op::Input => Outcome::Write(input),
            Op::Output => Outcome::Output(inputs[0]),
            Op::Exit => Outcome::None,
        }
    }
}

impl From<i64> for Op {
    fn from(opcode: i64) -> Self {
        match opcode {
            1 => Op::Add,
            2 => Op::Multiply,
            3 => Op::Input,
            4 => Op::Output,
            99 => Op::Exit,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Command {
    pub op: Op,
    pub inputs: Vec<Param>,
}

impl Command {
    pub fn parse_inputs(&self, data: &[i64], pos: usize) -> Vec<i64> {
        self.inputs
            .iter()
            .enumerate()
            .map(|(i, param)| {
                let input = data[pos + i + 1];
                match param {
                    Param::Position => data[input as usize],
                    Param::Immediate => input,
                }
            })
            .collect()
    }
}

impl From<i64> for Command {
    fn from(value: i64) -> Self {
        let op = Op::from(value % 100);
        let inputs = (0..op.n_in())
            .scan(value / 10, |rem, _| {
                *rem = *rem / 10;
                Some(*rem % 10)
            })
            .map(Param::from)
            .collect();
        Self { op, inputs }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Machine {
    data: Vec<i64>,
    pos: usize,
    input: i64,
    outputs: Vec<i64>,
}

impl Machine {
    pub fn new(input: i64, data: &[i64]) -> Self {
        Self {
            data: data.into(),
            pos: 0,
            input,
            outputs: Vec::new(),
        }
    }

    fn apply_outcome(&mut self, op: Op, outcome: Outcome) {
        match outcome {
            Outcome::Write(value) => {
                let address = self.data[self.pos + op.n_in() + 1] as usize;
                self.data[address] = value;
            }
            Outcome::Output(value) => {
                self.outputs.push(value);
            }
            _ => (),
        }
        self.pos += 1 + op.n_params();
    }

    pub fn step(&mut self) -> bool {
        let command = Command::from(self.data[self.pos]);
        let op = command.op;
        let inputs = command.parse_inputs(&self.data, self.pos);
        let outcome = op.apply(&inputs, self.input);
        self.apply_outcome(op, outcome);
        outcome != Outcome::None
    }

    pub fn run(&mut self) -> &mut Self {
        iter::repeat(()).skip_while(|_| self.step()).next();
        self
    }

    pub fn output(&self) -> Option<i64> {
        self.outputs.last().cloned()
    }
}

fn main() {
    let input = include_str!("inputs/day-05.txt");
    let data: Vec<i64> = parse_ints(input, ',').collect();

    let answer1 = Machine::new(1, &data).run().output().unwrap();
    println!("{}", answer1);
}
