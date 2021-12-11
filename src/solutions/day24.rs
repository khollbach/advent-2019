use std::io;
use std::collections::HashSet;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Grid {
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

pub fn main() {
    let grid = read_input(io::stdin().lock());

    println!("{}", part_1(grid).biodiversity_rating());
}

fn part_1(grid: Grid) -> Grid {
    let mut seen = HashSet::new();

    let mut curr = grid;
    while !seen.contains(&curr) {
        seen.insert(curr.clone());
        curr.transform();
    }
    curr
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Grid {
    /// True for bug, false for empty.
    grid: [[bool; Grid::SIZE]; Grid::SIZE],
}

impl Grid {
    const SIZE: usize = 5;

    fn transform(&mut self) {
        let mut tmp = Grid::default();

        for i in 0..Self::SIZE {
            for j in 0..Self::SIZE {
                tmp.grid[i][j] = self.should_live(i, j);
            }
        }

        *self = tmp;
    }

    fn should_live(&self, i: usize, j: usize) -> bool {
        let is_bug = self.grid[i][j];
        let num_adj = Self::neighbors(i, j).filter(|&(i2, j2)| self.grid[i2][j2]).count();

        if is_bug {
            num_adj == 1
        } else {
            num_adj == 1 || num_adj == 2
        }
    }

    fn neighbors(i: usize, j: usize) -> impl Iterator<Item=(usize, usize)> {
        let i = i as isize;
        let j = j as isize;

        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter_map(move |(di, dj)| {
            let i2 = i + di;
            let j2 = j + dj;

            if (i2, j2) != (i, j) && Self::in_range(i2, j2) {
                Some((i2 as usize, j2 as usize))
            } else {
                None
            }
        })
    }

    fn in_range(i: isize, j: isize) -> bool {
        let n = Self::SIZE as isize;

        0 <= i && i < n &&
            0 <= j && j < n
    }

    fn biodiversity_rating(&self) -> u32 {
        let to_char = |(i, j): (usize, usize)| if self.grid[i][j] { '1' } else { '0' };

        let n: String = Self::all_points().rev().map(to_char).collect();

        u32::from_str_radix(&n, 2).unwrap()
    }

    /// Increasing row-major order.
    fn all_points() -> impl DoubleEndedIterator<Item=(usize, usize)> {
        (0..Self::SIZE).flat_map(|i| {
            (0..Self::SIZE).map(move |j| {
                (i, j)
            })
        })
    }
}
