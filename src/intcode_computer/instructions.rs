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
    Halt,
}

impl Operation {
    pub fn new(opcode: i32) -> Self {
        assert!(opcode >= 0);

        match opcode % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
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
}

impl ParameterMode {
    fn new(bit: i32) -> Self {
        match bit {
            0 => Position,
            1 => Immediate,
            _ => panic!("Invalid parameter mode bit: {}", bit),
        }
    }

    /// Infinite iterator of parameter modes.
    pub fn parse_opcode(opcode: i32) -> impl Iterator<Item = Self> {
        assert!(opcode >= 0);

        let mut bits = opcode / 100;

        iter::repeat_with(move || {
            let bit = bits % 10;
            bits /= 10;

            Self::new(bit)
        })
    }
}
