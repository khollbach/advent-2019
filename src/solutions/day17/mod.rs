use std::{io, thread};
use std::time::Duration;
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};
use crate::solutions::day17::map::{Map, build_map, Robot, Step};
use crate::solutions::day17::map::Step::{MoveForward, TurnLeft, TurnRight};

mod map;

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());
    let (map, _robot) = build_map(prog.clone());

    println!("{}", part_1(&map));

    // part_2_traverse(&map, _robot);
    println!("{}", part_2_answer(prog, false));
}

fn part_1(map: &Map) -> isize {
    map.intersections().map(|p| p.row * p.col).sum()
}

#[allow(unused)]
fn part_2_traverse(map: &Map, robot: Robot) {
    let steps = map.traverse(robot);
    print_steps(&steps);
}

/// Dump the steps list, so I can solve the puzzle by hand :)
fn print_steps(steps: &[Step]) {
    let step_to_char = |s| match s {
        TurnLeft => '-',
        TurnRight => '+',
        MoveForward(6) => '1',
        MoveForward(8) => '2',
        MoveForward(10) => '3',
        _ => panic!(),
    };

    let line: String = steps.iter().copied().map(step_to_char).collect();
    println!("{}", line);
}

/*
Solving it by hand:
                                           C              C       C
              B        B                           B                      B
+3+2-3-3   +2-1-1   +2-1-1   +3+2-3-3   -3+3-1   +2-1-1 -3+3-1 -3+3-1  +2-1-1   +3+2-3-3
   A                             A                                                  A

main:
A,B,B,A,C,B,C,C,B,A

A:
R,10,R,8,L,10,L,10

B:
R,8,L,6,L,6

C:
L,10,R,10,L,6
*/

/// Note: there must be one more line of input after this: y or n (followed by a newline).
const ASCII_INPUT: &str = "\
A,B,B,A,C,B,C,C,B,A
R,10,R,8,L,10,L,10
R,8,L,6,L,6
L,10,R,10,L,6
";

/// Note that this is hard-coded to work with my input; it won't work in general.
///
/// `print_output` controls whether the entire traversal is printed as ascii. It's kind of like
/// an animation, but there might be a lot of screen-tearing (I'm guessing this depends on how fast
/// your terminal renders each screenfull of output text).
fn part_2_answer(mut ascii_prog: Vec<i64>, print_output: bool) -> i64 {
    assert_eq!(ascii_prog[0], 1);
    ascii_prog[0] = 2;

    let y_or_n = if print_output {
        "y\n"
    } else {
        "n\n"
    };

    let mut input = ASCII_INPUT.chars().chain(y_or_n.chars());
    let mut output_buf = String::new();
    let mut ans = None;

    let mut cpu = IntcodeComputer::new(ascii_prog);
    cpu.run_io(
        &mut || input.next().unwrap() as i64,
        &mut |x| {
            if 0 <= x && x < 256 {
                let c = x as u8 as char;

                // On every blank line:
                if c == '\n' && output_buf.chars().last() == Some('\n') {
                    // Flush output to the screen, then pause very briefly between frames.
                    if print_output {
                        println!("{}", output_buf);
                        thread::sleep(Duration::from_millis(50));
                    }

                    output_buf.clear();
                } else {
                    output_buf.push(c);
                }
            } else {
                assert!(ans.is_none());
                ans = Some(x);
            }
        }
    );

    // Any remaining output?
    if print_output {
        print!("{}", output_buf);
    }

    ans.unwrap()
}
