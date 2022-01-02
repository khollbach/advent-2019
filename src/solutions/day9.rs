use std::io;
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    solve(1, prog.clone());
    solve(2, prog);
}

fn solve(part_number: i64, prog: Vec<i64>) {
    assert!(part_number == 1 || part_number == 2);

    IntcodeComputer::new(prog).io(move || part_number, |x| println!("{}", x)).run();
}
