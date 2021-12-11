use std::io::BufRead;
use crate::solutions::day24::Grid;

pub fn read_input(input: impl BufRead) -> Grid {
    let grid: Vec<_> = input.lines().map(|line| {
        let row: Vec<_> = line.unwrap().chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid cell character: {}", c),
        }).collect();

        // Into an array.
        row.try_into().unwrap()
    }).collect();

    // Into an array of arrays.
    Grid { grid: grid.try_into().unwrap() }
}
