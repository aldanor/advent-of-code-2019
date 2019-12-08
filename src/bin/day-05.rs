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
    Jump(usize),
    None,
}

impl Outcome {
    pub fn jump_if(condition: bool, value: usize) -> Self {
        if condition {
            Self::Jump(value)
        } else {
            Self::None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit,
}

impl Op {
    pub fn n_in(&self) -> usize {
        match self {
            Op::Input | Op::Exit => 0,
            Op::Output => 1,
            _ => 2,
        }
    }

    pub fn has_out(&self) -> bool {
        match self {
            Op::Add | Op::Multiply | Op::Input | Op::LessThan | Op::Equals => true,
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
            Op::JumpIfTrue => Outcome::jump_if(inputs[0] != 0, inputs[1] as _),
            Op::JumpIfFalse => Outcome::jump_if(inputs[0] == 0, inputs[1] as _),
            Op::LessThan => Outcome::Write((inputs[0] < inputs[1]) as _),
            Op::Equals => Outcome::Write((inputs[0] == inputs[1]) as _),
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
            5 => Op::JumpIfTrue,
            6 => Op::JumpIfFalse,
            7 => Op::LessThan,
            8 => Op::Equals,
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
        self.pos = match outcome {
            Outcome::Jump(pos) => pos,
            _ => self.pos + 1 + op.n_params(),
        }
    }

    pub fn step(&mut self) -> bool {
        let command = Command::from(self.data[self.pos]);
        let op = command.op;
        let inputs = command.parse_inputs(&self.data, self.pos);
        let outcome = op.apply(&inputs, self.input);
        self.apply_outcome(op, outcome);
        op != Op::Exit
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

    let answer2 = Machine::new(5, &data).run().output().unwrap();
    println!("{}", answer2);
}

#[test]
fn test_machine() {
    let data = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    for input in 0..20 {
        let output = Machine::new(input, &data).run().output().unwrap();
        let expected = if input < 8 {
            999
        } else if input == 8 {
            1000
        } else {
            1001
        };
        assert_eq!(output, expected);
    }
}
