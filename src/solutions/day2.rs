use crate::intcode_computer::{read_intcode_program, IntcodeComputer};
use std::io;

pub fn main() {
    let nums = read_intcode_program(io::stdin().lock());

    let ans = part_1(nums.clone());
    println!("{}", ans);

    let (noun, verb) = part_2(nums);
    println!("{}", noun * 100 + verb);
}

fn part_1(nums: Vec<i64>) -> i64 {
    IntcodeComputer::new(nums).run_noun_verb(12, 2)
}

fn part_2(nums: Vec<i64>) -> (i64, i64) {
    for noun in 0..100 {
        for verb in 0..100 {
            if IntcodeComputer::new(nums.clone()).run_noun_verb(noun, verb) == 1969_07_20 {
                return (noun, verb);
            }
        }
    }

    panic!("No noun/verb pair found.");
}
