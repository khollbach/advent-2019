use crate::intcode_computer::{read_intcode_program, IntcodeComputer};
use std::io;

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    let mut cpu = IntcodeComputer::new(prog);
    cpu.run_io(Box::new(|| 1), Box::new(|x| println!("{}", x)));
}
