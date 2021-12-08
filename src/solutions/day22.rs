use std::io;
use crate::solutions::day22::input::read_input;
use crate::solutions::day22::Technique::{Cut, Deal, Reverse};

mod input;

pub fn main() {
    let shuffle = read_input(io::stdin().lock());

    println!("{}", part_1(shuffle));
}

const DECK_SIZE: u32 = 10_007;
const TARGET_CARD: u32 = 2019;

fn part_1(shuffle: Vec<Technique>) -> usize {
    let cards = (0..DECK_SIZE).collect();
    let mut deck = Deck { cards };

    for technique in shuffle {
        deck.apply(technique);
    }

    deck.cards.into_iter().position(|c| c == TARGET_CARD).unwrap()
}

pub enum Technique {
    Reverse,
    Cut(isize),
    Deal(usize),
}

#[derive(Debug)]
struct Deck {
    cards: Vec<u32>,
}

impl Deck {
    fn apply(&mut self, technique: Technique) {
        match technique {
            Reverse => self.reverse(),
            Cut(n) => self.cut(n),
            Deal(n) => self.deal(n),
        }
    }

    fn reverse(&mut self) {
        self.cards.reverse();
    }

    fn cut(&mut self, offset: isize) {
        if offset >= 0 {
            self.cards.rotate_left(offset as usize);
        } else {
            self.cards.rotate_right(offset.abs() as usize);
        }
    }

    fn deal(&mut self, step_size: usize) {
        for (i, c) in self.cards.clone().into_iter().enumerate() {
            let idx = i * step_size % self.cards.len();
            self.cards[idx] = c;
        }
    }
}
