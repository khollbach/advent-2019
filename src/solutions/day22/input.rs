use std::io::BufRead;
use Technique::{Cut, Deal, Reverse};

pub fn read_input(input: impl BufRead) -> Vec<Technique> {
    input.lines().map(|line| Technique::new(&line.unwrap())).collect()
}

#[derive(Debug, Copy, Clone)]
pub enum Technique {
    Reverse,
    Cut(isize),
    Deal(usize),
}

impl Technique {
    fn new(line: &str) -> Self {
        if line == "deal into new stack" {
            Reverse
        } else {
            let last_word = line.split(' ').rev().next().unwrap();

            if line.starts_with("cut ") {
                let i: isize = last_word.parse().unwrap();
                Cut(i)
            } else {
                assert!(line.starts_with("deal with increment "));

                let n: usize = last_word.parse().unwrap();
                assert_ne!(n, 0);
                Deal(n)
            }
        }
    }
}
