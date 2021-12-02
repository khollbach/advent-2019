use std::iter;
use crate::solutions::day17::map::{Map, Robot};
use crate::solutions::day17::map::part_2::Step::{MoveForward, TurnLeft, TurnRight};
use crate::solutions::day17::map::Tile::Land;

#[derive(Debug, Copy, Clone)]
pub enum Step {
    MoveForward(u32),
    TurnRight,
    TurnLeft,
}

impl Map {
    /// Wander until you hit a dead-end.
    ///
    /// Based on eye-balling the input, this simple traversal should walk the whole map.
    pub fn traverse(&self, mut robot: Robot) -> Vec<Step> {
        let steps: Vec<_> = iter::from_fn(|| self.step(&mut robot)).collect();
        compress_steps(&steps)
    }

    fn step(&self, robot: &mut Robot) -> Option<Step> {
        debug_assert_eq!(self[robot.pos], Land);

        // Try walking forwards.
        let dir = robot.dir;
        let front = robot.pos + dir.to_point();
        if self.get(front) == Some(Land) {
            robot.pos = front;
            return Some(MoveForward(1));
        }

        // Try turning left.
        let dir = robot.dir.ccw();
        let left = robot.pos + dir.to_point();
        if self.get(left) == Some(Land) {
            robot.dir = dir;
            return Some(TurnLeft);
        }

        // Try turning right.
        let dir = robot.dir.cw();
        let right = robot.pos + dir.to_point();
        if self.get(right) == Some(Land) {
            robot.dir = dir;
            return Some(TurnRight);
        }

        None
    }
}

/// Helper for Map::traverse.
///
/// Combine mutiple consecutive MoveForwards into a single one.
fn compress_steps(steps: &[Step]) -> Vec<Step> {
    let mut ret = Vec::with_capacity(steps.len());

    // Add a dummy at the end for easier parsing.
    let steps = steps.iter().copied().chain(iter::once(TurnRight));

    let mut curr_distance_forward = 0;

    for step in steps {
        if let MoveForward(n) = step {
            curr_distance_forward += n;
        } else {
            if curr_distance_forward != 0 {
                ret.push(MoveForward(curr_distance_forward));
                curr_distance_forward = 0;
            }
            ret.push(step);
        }
    }

    // Remove the dummy.
    ret.pop();

    ret.shrink_to_fit();
    ret
}
