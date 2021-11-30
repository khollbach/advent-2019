use std::fmt;
use crate::intcode_computer::IntcodeComputer;
use Tile::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Water,
    Land,
}

pub struct Map {
    grid: Vec<Vec<Tile>>,
    robot: Option<(usize, usize)>,
}

impl Map {
    pub fn new(ascii_prog: Vec<i64>) -> Self {
        let mut map = Self {
            grid: vec![vec![]],
            robot: None,
        };

        let mut input = || panic!();
        let mut output = |x| {
            assert!(0 <= x && x < 256);
            map.update(x as u8 as char);
        };

        let mut cpu = IntcodeComputer::new(ascii_prog);
        cpu.run_io(&mut input, &mut output);

        map.finish();
        map
    }

    /// Helper function for Map::new.
    fn update(&mut self, c: char) {
        match c {
            '\n' => {
                // New row.
                self.grid.push(vec![]);
            }
            '.' => {
                self.grid.last_mut().unwrap().push(Water);
            }
            _ => {
                assert!("#^v<>".contains(c));
                self.grid.last_mut().unwrap().push(Land);

                // We found the robot.
                if c != '#' {
                    assert!(self.robot.is_none());
                    let i = self.grid.len();
                    let j = self.grid.last().unwrap().len();
                    self.robot = Some((i, j));
                    // robot = Some(Robot::new(c, i, j)); // todo: directions and stuff, probably
                }
            }
        }
    }

    /// Helper function for Map::new.
    ///
    /// Account for trailing newlines by popping empty rows.
    ///
    /// Panics if the final grid is empty or jagged.
    fn finish(&mut self) {
        while self.grid.last() == Some(&vec![]) {
            self.grid.pop();
        }

        // Is the grid empty?
        let row_len = self.grid[0].len();
        assert_ne!(row_len, 0);

        // Jagged?
        assert!(self.grid.iter().all(|row| row.len() == row_len));
    }

    pub fn intersections<'a>(&'a self) -> impl Iterator<Item=(usize, usize)> + 'a {
        self.all_points().filter(|&(i, j)| self.is_intersection(i, j))
    }

    fn all_points(&self) -> impl Iterator<Item=(usize, usize)> {
        let num_rows = self.grid.len();
        let num_cols = self.grid[0].len();

        (0..num_rows).flat_map(move |i| {
            (0..num_cols).map(move |j| {
                (i, j)
            })
        })
    }

    fn is_intersection(&self, i: usize, j: usize) -> bool {
        let num_adj_land = self.neighbors(i, j).filter(|&(i2, j2)| self.grid[i2][j2] == Land).count();

        self.grid[i][j] == Land && num_adj_land >= 3
    }

    fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item=(usize, usize)> {
        let num_rows = self.grid.len() as isize;
        let num_cols = self.grid[0].len() as isize;

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        dirs.into_iter().filter_map(move |(di, dj)| {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;

            if 0 <= i2 && i2 < num_rows &&
                0 <= j2 && j2 < num_cols
            {
                Some((i2 as usize, j2 as usize))
            } else {
                None
            }
        })
    }
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Water => '.',
            Land => '#',
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "robot={:?}", self.robot)?; // todo: show on map, with proper direction
        for row in &self.grid {
            let s: String = row.iter().copied().map(Tile::to_char).collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}
