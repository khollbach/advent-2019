use std::iter;
use Operation::*;
use ParameterMode::*;
use ParameterType::*;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Mul,

    Input,
    Output,

    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,

    AdjustRelBase,

    Halt,
}

impl Operation {
    pub fn new(opcode: i64) -> Self {
        assert!(opcode >= 0);

        match opcode % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => AdjustRelBase,
            99 => Halt,
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    pub fn param_types(self) -> Vec<ParameterType> {
        match self {
            Add => vec![Read, Read, Write],
            Mul => vec![Read, Read, Write],
            Input => vec![Write],
            Output => vec![Read],
            JumpIfTrue => vec![Read, Read],
            JumpIfFalse => vec![Read, Read],
            LessThan => vec![Read, Read, Write],
            Equals => vec![Read, Read, Write],
            AdjustRelBase => vec![Read],
            Halt => vec![],
        }
    }
}

pub enum ParameterType {
    Read,
    Write,
}

pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn new(digit: i64) -> Self {
        match digit {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Invalid parameter mode digit: {}", digit),
        }
    }

    /// Infinite iterator of parameter modes.
    pub fn parse_opcode(opcode: i64) -> impl Iterator<Item = Self> {
        assert!(opcode >= 0);

        let mut digits = opcode / 100;

        iter::repeat_with(move || {
            let d = digits % 10;
            digits /= 10;

            Self::new(d)
        })
    }
}
