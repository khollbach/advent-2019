use std::collections::VecDeque;
use std::io;
use std::io::BufReader;
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};

/*
The logic that worked for part 1 was:
j := !a || (!c && d)

Which I encoded as:
NOT C J
AND D J
NOT A T
OR T J

---

For part 2, the following ended up working:
j := !a || ( (!b || !c) && d && h )

Encoded as:
OR B J
AND C J
NOT J J
AND D J
AND H J
NOT A T
OR T J
 */

pub fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/21"));
    let prog = read_intcode_program(BufReader::new(input.as_bytes()));

    let mut input_buf = VecDeque::new();
    let input = || {
        // Grab a line of input all-at-once from stdin.
        if input_buf.is_empty() {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            input_buf.extend(buf.chars());
        }

        // Yield up chars one-at-a-time to the intcode program.
        input_buf.pop_front().unwrap() as i64
    };

    let output = |x| {
        if 0 <= x && x < 256 {
            print!("{}", x as u8 as char);
        } else {
            print!("\n*** NON-ASCII VALUE: {} ***\n", x);
        }
    };

    IntcodeComputer::new(prog).io(input, output).run();
}
