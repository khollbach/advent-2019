use std::collections::HashSet;
use crate::solutions::day24::{Grid, should_live};

pub fn solve(grid: Grid) -> u32 {
    let mut seen = HashSet::new();

    let mut curr = grid;
    while !seen.contains(&curr) {
        seen.insert(curr.clone());
        curr.transform();
    }
    curr.biodiversity_rating()
}

impl Grid {
    pub const SIZE: usize = 5;

    fn transform(&mut self) {
        let mut tmp = Self::default();

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

        should_live(is_bug, num_adj)
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
