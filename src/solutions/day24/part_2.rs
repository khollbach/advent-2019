use std::collections::HashSet;
use std::iter;
use crate::solutions::day24::{Grid, should_live};

const NUM_ITERS: usize = 200;

pub fn solve(grid: Grid) -> usize {
    let mut planet = Planet::new(grid);

    for _ in 0..NUM_ITERS {
        planet.transform();
    }

    planet.bugs.len()
}

#[derive(Default)]
struct Planet {
    bugs: HashSet<Point>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    level: i32,
    row: i32,
    col: i32,
}

impl Point {
    fn new(level: i32, row: i32, col: i32) -> Self {
        assert!(Planet::in_range_not_mid(row, col));

        Self { level, row, col }
    }
}

impl Planet {
    const GRID_SIZE: i32 = Grid::SIZE as i32;
    const MID_1D: i32 = Self::GRID_SIZE / 2;
    const MID_2D: (i32, i32) = (Self::MID_1D, Self::MID_1D);

    fn new(grid: Grid) -> Self {
        let num_levels = NUM_ITERS * 2 + 1;
        let mut bugs = HashSet::with_capacity(Grid::SIZE * Grid::SIZE * num_levels);

        for row in 0..Grid::SIZE {
            for col in 0..Grid::SIZE {
                if grid.grid[row][col] {
                    let p = Point::new(0, row as i32, col as i32);
                    bugs.insert(p);
                }
            }
        }

        Self { bugs }
    }

    fn transform(&mut self) {
        let relevant_points: HashSet<_> = self.bugs.iter().copied().flat_map(|p| {
            iter::once(p).chain(Self::neighbors(p))
        }).collect();

        self.bugs = relevant_points.into_iter().filter(|&p| self.should_live(p)).collect();
    }

    fn should_live(&self, p: Point) -> bool {
        let is_bug = self.bugs.contains(&p);
        let num_adj = Self::neighbors(p).filter(|p2| self.bugs.contains(&p2)).count();

        should_live(is_bug, num_adj)
    }

    /// This function is a bit of a mess, and it would be nice to implement it without allocations.
    /// todo: how?
    fn neighbors(p: Point) -> impl Iterator<Item=Point> {
        let n = Self::GRID_SIZE;
        let Point { level, row, col } = p;

        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().flat_map(move |(dr, dc)| {
            let r = row + dr;
            let c = col + dc;

            if Self::in_range_not_mid(r, c) {
                // Same level.
                vec![Point::new(level, r, c)]
            } else if (r, c) == Self::MID_2D {
                // Inner level.
                let rc_coords: Vec<_> = if dr == 1 {
                    (0..n).map(|c| (0, c)).collect() // Top row
                } else if dr == -1 {
                    (0..n).map(|c| (n-1, c)).collect() // Bot row
                } else if dc == 1 {
                    (0..n).map(|r| (r, 0)).collect() // Left col
                } else {
                    assert_eq!(dc, -1);
                    (0..n).map(|r| (r, n-1)).collect() // Right col
                };

                rc_coords.into_iter().map(|(r, c)| Point::new(level + 1, r, c)).collect()
            } else {
                // Outer level.
                let mid = Self::MID_1D;
                let (r, c) = if r < 0 {
                    (mid - 1, mid) // Up
                } else if r >= n {
                    (mid + 1, mid) // Down
                } else if c < 0 {
                    (mid, mid - 1) // Left
                } else {
                    assert!(c >= n);
                    (mid, mid + 1) // Right
                };

                vec![Point::new(level - 1, r, c)]
            }.into_iter()
        })
    }

    fn in_range_not_mid(row: i32, col: i32) -> bool {
        let n = Self::GRID_SIZE;

        (row, col) != Planet::MID_2D &&
            0 <= row && row < n &&
            0 <= col && col < n
    }
}
