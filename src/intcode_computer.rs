use std::fmt;
use instructions::{Operation, ParameterMode, ParameterType};
use std::io::BufRead;
use mem::Memory;

mod mem;
mod instructions;

pub fn read_intcode_program(input: impl BufRead) -> Vec<i64> {
    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());

    line.split(',').map(|word| word.parse().unwrap()).collect()
}

type Input<'a> = &'a mut (dyn FnMut() -> i64 + Send);
type Output<'a> = &'a mut (dyn FnMut(i64) + Send);

pub struct IntcodeComputer<'i, 'o> {
    /// Instruction pointer.
    ip: i64,
    /// Relative base.
    rb: i64,

    memory: Memory,

    input: Option<Input<'i>>,
    output: Option<Output<'o>>,
}

impl<'i, 'o> IntcodeComputer<'i, 'o> {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            ip: 0,
            rb: 0,
            memory: Memory::new(program),
            input: None,
            output: None,
        }
    }

    pub fn run_noun_verb(&mut self, noun: i64, verb: i64) -> i64 {
        self.memory[1] = noun;
        self.memory[2] = verb;

        self.run_inner();

        self.memory[0]
    }

    /// Set input and output.
    pub fn io<'i2, 'o2>(self, input: Input<'i2>, output: Output<'o2>) -> IntcodeComputer<'i2, 'o2> {
        // The borrow checker doesn't like `IntcodeComputer { input, output, ..self }`,
        // otherwise we'd just write that.
        IntcodeComputer {
            ip: self.ip,
            rb: self.rb,
            memory: self.memory,
            input: Some(input),
            output: Some(output),
        }
    }

    pub fn run(mut self) {
        self.run_inner();
    }

    fn run_inner(&mut self) {
        while self.step() {}
    }

    /// Execute the current instruction and bump the program counter.
    /// Return false if the cpu should halt.
    fn step(&mut self) -> bool {
        let (op, args) = self.read_instruction();

        use Operation::*;
        match op {
            Add => {
                self.memory[args[2]] = args[0] + args[1];
            }
            Mul => {
                self.memory[args[2]] = args[0] * args[1];
            }
            Input => {
                self.memory[args[0]] = self.input.as_mut().unwrap()();
            }
            Output => {
                self.output.as_mut().unwrap()(args[0]);
            }
            JumpIfTrue => {
                if args[0] != 0 {
                    self.ip = args[1];
                }
            }
            JumpIfFalse => {
                if args[0] == 0 {
                    self.ip = args[1];
                }
            }
            LessThan => {
                self.memory[args[2]] = (args[0] < args[1]) as i64;
            }
            Equals => {
                self.memory[args[2]] = (args[0] == args[1]) as i64;
            }
            AdjustRelBase => {
                self.rb += args[0];
            }
            Halt => {
                return false;
            }
        }

        true
    }

    /// Updates the instruction pointer accordingly.
    fn read_instruction(&mut self) -> (Operation, Vec<i64>) {
        let opcode = self.memory[self.ip];
        self.ip += 1;

        let op = Operation::new(opcode);
        let param_types = op.param_types().into_iter();
        let param_modes = ParameterMode::parse_opcode(opcode);

        let args = param_types.zip(param_modes).map(|(type_, mode)| {
            let a = self.memory[self.ip];
            self.ip += 1;

            use ParameterType::*;
            use ParameterMode::*;
            match (type_, mode) {
                (Read, Position) => self.memory[a],
                (Read, Relative) => self.memory[a + self.rb],
                (Read, Immediate) => a,

                (Write, Position) => a,
                (Write, Relative) => a + self.rb,
                (Write, Immediate) => {
                    panic!("Write params can't be in immediate mode: {}", opcode)
                }
            }
        }).collect();

        (op, args)
    }
}

impl fmt::Debug for IntcodeComputer<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ip={} rb={} mem={:?}", self.ip, self.rb, self.memory)
    }
}
