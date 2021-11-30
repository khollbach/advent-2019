use crate::intcode_computer::{read_intcode_program, IntcodeComputer};
use std::io;

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());
    part_1(prog.clone());
    part_2(prog);
}

fn part_1(prog: Vec<i64>) {
    let mut cpu = IntcodeComputer::new(prog);
    cpu.run_io(&mut || 1, &mut |x| if x != 0 {
            println!("{}", x)
    });
}

fn part_2(prog: Vec<i64>) {
    let mut cpu = IntcodeComputer::new(prog);
    cpu.run_io(&mut || 5, &mut |x| println!("{}", x));
}
