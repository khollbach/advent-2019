use std::io;
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    part_1(prog);
}

fn part_1(prog: Vec<i64>) {
    let mut cpu = IntcodeComputer::new(prog);

    cpu.run_io(Box::new(|| 1), Box::new(|x| println!("{}", x)));
}
