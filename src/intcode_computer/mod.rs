use instructions::{Operation, ParameterMode, ParameterType};
use std::io::BufRead;

mod instructions;

pub fn read_intcode_program(input: impl BufRead) -> Vec<i32> {
    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());

    line.split(',').map(|word| word.parse().unwrap()).collect()
}

type In = Box<dyn FnMut() -> i32>;
type Out = Box<dyn FnMut(i32)>;

pub struct IntcodeComputer {
    /// Instruction pointer.
    ip: usize,
    memory: Vec<i32>,

    input: Option<In>,
    output: Option<Out>,
}

impl IntcodeComputer {
    pub fn new(program: Vec<i32>) -> Self {
        Self {
            ip: 0,
            memory: program,
            input: None,
            output: None,
        }
    }

    pub fn run_noun_verb(&mut self, noun: i32, verb: i32) -> i32 {
        self.memory[1] = noun;
        self.memory[2] = verb;

        self.run();

        self.memory[0]
    }

    pub fn run_io(&mut self, input: In, output: Out) {
        self.input = Some(input);
        self.output = Some(output);

        self.run();
    }

    fn run(&mut self) {
        while self.step() {}
    }

    /// Execute the current instruction and bump the program counter.
    /// Return false if the cpu should halt.
    fn step(&mut self) -> bool {
        let (op, args) = self.read_instruction();

        use Operation::*;
        match op {
            Add => {
                self.memory[args[2] as usize] = args[0] + args[1];
            }
            Mul => {
                self.memory[args[2] as usize] = args[0] * args[1];
            }
            Input => {
                self.memory[args[0] as usize] = self.input.as_mut().unwrap()();
            }
            Output => {
                self.output.as_mut().unwrap()(args[0]);
            }
            Halt => {
                return false;
            }
        }

        true
    }

    /// Updates the instruction pointer accordingly.
    fn read_instruction(&mut self) -> (Operation, Vec<i32>) {
        let opcode = self.memory[self.ip];
        self.ip += 1;

        let op = Operation::new(opcode);
        let param_types = op.param_types().into_iter();
        let param_modes = ParameterMode::parse_opcode(opcode);

        let args = param_types
            .zip(param_modes)
            .map(|(type_, mode)| {
                let a = self.memory[self.ip];
                self.ip += 1;

                use ParameterMode::*;
                use ParameterType::*;
                match (type_, mode) {
                    (Read, Position) => self.memory[a as usize],
                    (Read, Immediate) => a,
                    (Write, Position) => a,
                    (Write, Immediate) => {
                        panic!("Write params can't be in immediate mode: {}", opcode)
                    }
                }
            })
            .collect();

        (op, args)
    }
}
