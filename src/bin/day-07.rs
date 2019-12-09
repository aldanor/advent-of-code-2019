use std::iter;

use itertools::Itertools;

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
    Input(i64),
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

    pub fn apply(&self, args: &[i64], input: Option<i64>) -> Outcome {
        match self {
            Op::Add => Outcome::Write(args[0] + args[1]),
            Op::Multiply => Outcome::Write(args[0] * args[1]),
            Op::Input => Outcome::Input(input.unwrap()),
            Op::Output => Outcome::Output(args[0]),
            Op::JumpIfTrue => Outcome::jump_if(args[0] != 0, args[1] as _),
            Op::JumpIfFalse => Outcome::jump_if(args[0] == 0, args[1] as _),
            Op::LessThan => Outcome::Write((args[0] < args[1]) as _),
            Op::Equals => Outcome::Write((args[0] == args[1]) as _),
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
    pub params: Vec<Param>,
}

impl Command {
    pub fn parse_args(&self, data: &[i64], pos: usize) -> Vec<i64> {
        self.params
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
        let params = (0..op.n_in())
            .scan(value / 10, |rem, _| {
                *rem = *rem / 10;
                Some(*rem % 10)
            })
            .map(Param::from)
            .collect();
        Self { op, params }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Machine {
    data: Vec<i64>,
    pos: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl Machine {
    pub fn new(data: &[i64], inputs: &[i64]) -> Self {
        Self {
            data: data.into(),
            pos: 0,
            inputs: inputs.into(),
            outputs: Vec::new(),
        }
    }

    fn apply_outcome(&mut self, op: Op, outcome: Outcome) {
        match outcome {
            Outcome::Write(value) | Outcome::Input(value) => {
                let address = self.data[self.pos + op.n_in() + 1] as usize;
                self.data[address] = value;
            }
            Outcome::Output(value) => {
                self.outputs.push(value);
            }
            _ => (),
        }
        if let Outcome::Input(_) = outcome {
            self.inputs.remove(0);
        }
        self.pos = match outcome {
            Outcome::Jump(pos) => pos,
            _ => self.pos + 1 + op.n_params(),
        }
    }

    pub fn step(&mut self) -> bool {
        let command = Command::from(self.data[self.pos]);
        let op = command.op;
        let inputs = command.parse_args(&self.data, self.pos);
        let next_input = self.inputs.first().cloned();
        let outcome = op.apply(&inputs, next_input);
        self.apply_outcome(op, outcome);
        op != Op::Exit
    }

    pub fn run(&mut self) -> i64 {
        iter::repeat(()).skip_while(|_| self.step()).next();
        self.outputs.last().cloned().unwrap()
    }
}

struct Amplifiers {
    data: Vec<i64>,
}

impl Amplifiers {
    pub fn new(data: &[i64]) -> Self {
        Self { data: data.into() }
    }

    pub fn run(&self, phases: &[i64]) -> i64 {
        phases.iter().cloned().fold(0, |state, phase| {
            Machine::new(&self.data, &vec![phase, state]).run()
        })
    }

    pub fn find_best(&self, n: usize) -> (i64, Vec<i64>) {
        (0..n as i64)
            .into_iter()
            .permutations(n)
            .map(|p: Vec<_>| (self.run(&p), p))
            .max()
            .unwrap()
    }
}

fn main() {
    let input = include_str!("inputs/day-07.txt");
    let data: Vec<i64> = parse_ints(input, ',').collect();

    let amp = Amplifiers::new(&data);
    let (answer1, _) = amp.find_best(5);
    println!("{}", answer1);
}

#[test]
fn test_part1() {
    let amp = Amplifiers::new(&[
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ]);
    let (phases, best) = (vec![4, 3, 2, 1, 0], 43210);
    assert_eq!(amp.run(&phases), best);
    assert_eq!(amp.find_best(5), (best, phases));

    let amp = Amplifiers::new(&[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ]);
    let (phases, best) = (vec![0, 1, 2, 3, 4], 54321);
    assert_eq!(amp.run(&phases), best);
    assert_eq!(amp.find_best(5), (best, phases));

    let amp = Amplifiers::new(&[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ]);
    let (phases, best) = (vec![1, 0, 4, 3, 2], 65210);
    assert_eq!(amp.run(&phases), best);
    assert_eq!(amp.find_best(5), (best, phases));
}
