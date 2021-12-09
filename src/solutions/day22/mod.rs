use std::io;
use crate::solutions::day22::input::read_input;
use crate::solutions::day22::part1::part_1;
use crate::solutions::day22::part2::part_2;

mod input;
mod part1;
mod part2;

pub fn main() {
    let shuffle = read_input(io::stdin().lock());

    println!("{}", part_1(&shuffle));
    println!("{}", part_2(&shuffle));
}
