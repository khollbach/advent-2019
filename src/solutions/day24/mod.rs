use std::io;
use crate::solutions::day24::input::read_input;

mod input;
mod part_1;
mod part_2;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct Grid {
    /// True for bug, false for empty.
    pub grid: [[bool; Grid::SIZE]; Grid::SIZE],
}

fn should_live(is_bug: bool, num_adj: usize) -> bool {
    if is_bug {
        num_adj == 1
    } else {
        num_adj == 1 || num_adj == 2
    }
}

pub fn main() {
    let grid = read_input(io::stdin().lock());

    println!("{}", part_1::solve(grid.clone()));
    println!("{}", part_2::solve(grid));
}
