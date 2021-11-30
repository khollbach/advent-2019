use std::io;
use crate::intcode_computer::read_intcode_program;
use map::Map;

mod map;

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());
    let map = Map::new(prog);

    let ans = part_1(&map);
    println!("{}", ans);
}

fn part_1(map: &Map) -> usize {
    map.intersections().map(|(i, j)| i * j).sum()
}
