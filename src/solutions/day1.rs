use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<u32> {
    input.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

pub fn main() {
    let nums = read_input(io::stdin().lock());

    let ans = part_1(nums.into_iter());
    println!("{}", ans);
}

fn part_1(nums: impl Iterator<Item=u32>) -> u32 {
    nums.map(fuel_required).sum()
}

fn fuel_required(mass: u32) -> u32 {
    mass / 3 - 2
}
