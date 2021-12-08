use std::io::BufRead;
use crate::solutions::day22::Technique;
use crate::solutions::day22::Technique::{Cut, Deal, Reverse};

pub fn read_input(input: impl BufRead) -> Vec<Technique> {
    input.lines().map(|line| Technique::new(&line.unwrap())).collect()
}

impl Technique {
    fn new(line: &str) -> Self {
        if line == "deal into new stack" {
            Reverse
        } else {
            let last_word = line.split(' ').rev().next().unwrap();

            if line.starts_with("cut ") {
                Cut(last_word.parse().unwrap())
            } else {
                assert!(line.starts_with("deal with increment "));
                Deal(last_word.parse().unwrap())
            }
        }
    }
}
