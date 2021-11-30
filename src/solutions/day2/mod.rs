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

    let ans = part_1(nums.clone());
    println!("{}", ans);

    let (noun, verb) = part_2(nums);
    println!("{}", noun * 100 + verb);
}

fn part_1(nums: Vec<u32>) -> u32 {
    IntcodeComputer::new(nums).run(12, 2)
}

fn part_2(nums: Vec<u32>) -> (u32, u32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = IntcodeComputer::new(nums.clone());
            if cpu.run(noun, verb) == 1969_07_20 {
                return (noun, verb);
            }
        }
    }

    panic!("No noun/verb pair found.");
}

struct IntcodeComputer {
    /// Instruction pointer.
    ip: usize,
    memory: Vec<u32>,
}

impl IntcodeComputer {
    fn new(program: Vec<u32>) -> Self {
        Self { ip: 0, memory: program }
    }

    fn run(&mut self, noun: u32, verb: u32) -> u32 {
        self.memory[1] = noun;
        self.memory[2] = verb;
        while self.step() {}
        self.memory[0]
    }

    /// Execute the current instruction and bump the program counter.
    /// Return false if the cpu should halt.
    fn step(&mut self) -> bool {
        let opcode = self.memory[self.ip];
        if opcode == 99 { return false; }

        // Addresses.
        let a = self.memory[self.ip + 1] as usize;
        let b = self.memory[self.ip + 2] as usize;
        let c = self.memory[self.ip + 3] as usize;

        // Values.
        let x = self.memory[a];
        let y = self.memory[b];

        self.memory[c] = match opcode {
            1 => x + y,
            2 => x * y,
            _ => panic!("Invalid opcode: {}", opcode),
        };

        self.ip += 4;

        true
    }
}
