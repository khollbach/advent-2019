use std::ops::Add;
use crate::solutions::day17::map::geometry::Dir::{Down, Left, Right, Up};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(mut self, other: Point) -> Point {
        self.row += other.row;
        self.col += other.col;
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/// In no particular order.
pub const DIRS: [Dir; 4] = [Up, Down, Left, Right];

impl Dir {
    pub fn to_point(self) -> Point {
        let (row, col) = match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        };

        Point { row, col }
    }

    pub fn cw(self) -> Dir {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn ccw(self) -> Dir {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}
