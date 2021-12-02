use std::fmt;
use std::ops::Index;
use crate::solutions::day17::map::geometry::{Dir, Point, DIRS};
use crate::solutions::day17::map::Tile::{Water, Land};

mod build_map;
mod geometry;
mod part_1;
mod part_2;

pub use build_map::build_map;
pub use part_2::Step;

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    dir: Dir,
    pos: Point,
}

pub struct Map {
    grid: Vec<Vec<Tile>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Water,
    Land,
}

impl Map {
    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;

        Point { row, col }
    }

    fn in_range(&self, p: Point) -> bool {
        in_range(p, self.dims())
    }

    fn get(&self, p: Point) -> Option<Tile> {
        if self.in_range(p) {
            Some(self[p])
        } else {
            None
        }
    }

    fn all_points(&self) -> impl Iterator<Item=Point> {
        let Point { row, col } = self.dims();

        (0..row).flat_map(move |row| {
            (0..col).map(move |col| {
                Point { row, col }
            })
        })
    }

    fn neighbors(&self, p: Point) -> impl Iterator<Item=Point> {
        let dims = self.dims();

        DIRS.into_iter().map(move |dir| p + dir.to_point()).filter(move |&p2| in_range(p2, dims))
    }
}

fn in_range(p: Point, dims: Point) -> bool {
    0 <= p.row && p.row < dims.row &&
        0 <= p.col && p.col < dims.col
}

impl Index<Point> for Map {
    type Output = Tile;

    fn index(&self, p: Point) -> &Tile {
        let row: usize = p.row.try_into().unwrap();
        let col: usize = p.col.try_into().unwrap();

        &self.grid[row][col]
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            let s: String = row.iter().copied().map(Tile::to_char).collect();
            writeln!(f, "{}", s)?;
        }

        Ok(())
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
