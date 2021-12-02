use crate::intcode_computer::IntcodeComputer;
use crate::solutions::day17::map::{Map, Robot, Tile};
use crate::solutions::day17::map::Tile::{Land, Water};
use crate::solutions::day17::map::geometry::Dir::{Up, Down, Left, Right};
use crate::solutions::day17::map::geometry::Point;

/// Run the "ASCII" program to get the initial state of the map and robot.
pub fn build_map(ascii_prog: Vec<i64>) -> (Map, Robot) {
    let mut builder = MapBuilder {
        grid: vec![vec![]],
        robot: None,
    };

    let mut input = || panic!();
    let mut output = |x| {
        assert!(0 <= x && x < 256);
        builder.update(x as u8 as char);
    };

    let mut cpu = IntcodeComputer::new(ascii_prog);
    cpu.run_io(&mut input, &mut output);

    builder.finish()
}

/// Helper struct for build_map.
struct MapBuilder {
    grid: Vec<Vec<Tile>>,
    robot: Option<Robot>,
}

impl MapBuilder {
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
                    self.add_robot(c);
                }
            }
        }
    }

    fn add_robot(&mut self, c: char) {
        assert!(self.robot.is_none());

        let dir = match c {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => panic!("Invalid direction character: {}", c),
        };

        // The last valid indices are those of the most recently added tile.
        let row = self.grid.len() as isize - 1;
        let col = self.grid.last().unwrap().len() as isize - 1;

        self.robot = Some(Robot { dir, pos: Point { row, col } });
    }

    /// Account for trailing newlines by popping empty rows.
    ///
    /// Panics if the final grid is empty or jagged, or if there's no robot.
    fn finish(mut self) -> (Map, Robot) {
        while self.grid.last() == Some(&vec![]) {
            self.grid.pop();
        }

        // Is the grid empty?
        let row_len = self.grid[0].len();
        assert_ne!(row_len, 0);

        // Jagged?
        assert!(self.grid.iter().all(|row| row.len() == row_len));

        (Map { grid: self.grid }, self.robot.unwrap())
    }
}
