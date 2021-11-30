use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<u32> {
    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());

    line.split(',').map(|word| word.parse().unwrap()).collect()
}

pub fn main() {
    let nums = read_input(io::stdin().lock());

    let ans = part_1(nums);
    println!("{}", ans);
}

fn part_1(nums: Vec<u32>) -> u32 {
    let mut cpu = IntcodeComputer::new(nums);
    cpu.memory[1] = 12;
    cpu.memory[2] = 2;
    cpu.run();
    cpu.memory[0]
}

struct IntcodeComputer {
    /// "Program counter" a.k.a. instruction pointer.
    pc: usize,
    memory: Vec<u32>,
}

impl IntcodeComputer {
    fn new(program: Vec<u32>) -> Self {
        Self { pc: 0, memory: program }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    /// Execute the current instruction and bump the program counter.
    /// Return false if the cpu should halt.
    fn step(&mut self) -> bool {
        let opcode = self.memory[self.pc];
        if opcode == 99 { return false; }

        // Addresses.
        let a = self.memory[self.pc + 1] as usize;
        let b = self.memory[self.pc + 2] as usize;
        let c = self.memory[self.pc + 3] as usize;

        // Values.
        let x = self.memory[a];
        let y = self.memory[b];

        self.memory[c] = match opcode {
            1 => x + y,
            2 => x * y,
            _ => panic!("Invalid opcode: {}", opcode),
        };

        self.pc += 4;

        true
    }
}
