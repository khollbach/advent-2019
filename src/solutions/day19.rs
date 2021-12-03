use std::io;
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    println!("{}", part_1(prog.clone())); // 211

    // Just brute force it ... :)
    // Takes about 30 seconds to run on my pc.
    let grid = Grid::new(prog, 1_000, 1_500);
    dbg!(grid.num_trues());
    dbg!(grid.row_num_trues(grid.grid.len() - 1));
    dbg!(grid.col_num_trues(grid.grid[0].len() - 1));

    // We're probably supposed to do this 'on-line' instead of building the whole grid greedily
    // up-front. Specifically, we should traverse the bottom-left edge of the beam, and for each point on that
    // edge, check the diagonal's length.
    dbg!(grid.find_large_square(100)); // 807, 1006
}

fn part_1(prog: Vec<i64>) -> usize {
    Grid::new(prog, 50, 50).num_trues()
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(prog: Vec<i64>, num_rows: usize, num_cols: usize) -> Self {
        let mut grid = vec![vec![false; num_cols]; num_rows];

        for i in 0..num_rows {
            for j in 0..num_cols {
                let mut input = [i as i64, j as i64].into_iter();
                let mut output = |x| {
                    debug_assert!(x == 0 || x == 1);
                    if x != 0 {
                        grid[i][j] = true;
                    }
                };

                let mut cpu = IntcodeComputer::new(prog.clone());
                cpu.run_io(&mut || input.next().unwrap(), &mut output);
            }
        }

        Self { grid }
    }

    fn find_large_square(&self, side_len: usize) -> Option<(usize, usize)> {
        let delta = side_len - 1;

        for i in delta..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let bot_left = (i, j);
                let top_left = (i - delta, j);
                let top_right = (i - delta, j + delta);

                if top_right.1 < self.grid[0].len() && self.diag_filled(bot_left, side_len) {
                    return Some(top_left);
                }
            }
        }

        None
    }

    fn diag_filled(&self, bot_left: (usize, usize), diag_len: usize) -> bool {
        let (mut i, mut j) = bot_left;

        for _ in 0..diag_len {
            if !self.grid[i][j] {
                return false;
            }

            i -= 1;
            j += 1;
        }

        true
    }

    fn num_trues(&self) -> usize {
        count_trues(self.grid.iter().flat_map(|row| row.iter().copied()))
    }

    fn row_num_trues(&self, i: usize) -> usize {
        let row = (0..self.grid[0].len()).map(|j| self.grid[i][j]);
        count_trues(row)
    }

    fn col_num_trues(&self, j: usize) -> usize {
        let col = (0..self.grid.len()).map(|i| self.grid[i][j]);
        count_trues(col)
    }
}

fn count_trues(iter: impl Iterator<Item=bool>) -> usize {
    iter.filter(|&b| b).count()
}
