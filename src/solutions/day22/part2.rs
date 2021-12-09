use crate::solutions::day22::part2::math::inverse;
use crate::solutions::day22::input::Technique;
use crate::solutions::day22::input::Technique::{Cut, Deal, Reverse};

/*
  T   G   M   K   1
119_315_717_514_047 cards
101_741_582_076_661 shuffles

It seems like each step of the shuffle is just an affine transformation (ax + b) modulo some large, fixed prime.
In particular, since the slope `a` is always relatively prime to the modulus, these should be invertable.

If we let `f` be the shuffle, which is the composition of each of the affine transformations, then
`f` should also be an affine transformation. We should then be able to compute g := f^-1, which we want to
apply 100 trillion times to the value 2020. To do this efficiently, we'll first compute g^2, g^4, g^8, ..., g^(2^n),
and then compose those. This will work as long as affine transformations are associative.

Yes, it turns out they are. (Simple to check in 1-dimension, just by pushing symbols around.)
 */

mod math;

const DECK_SIZE: u128 = 119_315_717_514_047;
const NUM_SHUFFLES: u128 = 101_741_582_076_661;
const TARGET_POSITION: u128 = 2020;

pub fn part_2(shuffle: &[Technique]) -> u128 {
    let mut f = Transformation::identity();
    for &technique in shuffle {
        f = f.compose(technique.to_transformation());
    }

    let f_many_times = f.pow(NUM_SHUFFLES);

    f_many_times.invert().apply(TARGET_POSITION)
}

impl Technique {
    fn to_transformation(self) -> Transformation {
        let (a, b) = match self {
            Reverse => (-1, -1),
            Cut(i) => (1, -i),
            Deal(n) => (n as isize, 0),
        };

        let m = DECK_SIZE as isize;
        Transformation::new(a.rem_euclid(m) as u128, b.rem_euclid(m) as u128)
    }
}

/// An affine transformation, modulo DECK_SIZE.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Transformation {
    /// Multiplier.
    a: u128,
    /// Offset.
    b: u128,
}

impl Transformation {
    fn new(mut a: u128, mut b: u128) -> Self {
        a %= DECK_SIZE;
        b %= DECK_SIZE;

        Self { a, b }
    }

    /// The identity transformation, f(x) = x.
    fn identity() -> Self {
        Self::new(1, 0)
    }

    /// If f = self, then this computes y = f(x).
    fn apply(self, x: u128) -> u128 {
        let Self { a, b } = self;

        (a * x + b) % DECK_SIZE
    }

    /// If f = self, and g = other, then this computes h(x) = g(f(x)), i.e. f and then g.
    fn compose(self, other: Self) -> Self {
        let Self { a, b } = self;
        let Self { a: c, b: d } = other;

        Self::new(a * c, b * c + d)
    }

    /// If f = self, then this computes g(x) = f^-1(x).
    fn invert(self) -> Self {
        let Self { a, b } = self;
        let a_inv = inverse(a, DECK_SIZE);
        let neg_a_inv = DECK_SIZE - a_inv;

        Self::new(a_inv, neg_a_inv * b)
    }

    /// If f = self, then this computes g(x) = f^e(x); that is, f composed `e` many times.
    fn pow(self, e: u128) -> Self {
        let powers_of_2 = self.powers_of_2();

        let mut acc = Self::identity();

        for i in 0..128 {
            let bit = 1 << i;
            if e & bit != 0 {
                acc = acc.compose(powers_of_2[i]);
            }
        }

        acc
    }

    /// Pre-compute a table of f to the power of 2^n, for all n in 0..128.
    fn powers_of_2(self) -> [Self; 128] {
        let mut powers_of_2 = [Self::identity(); 128];
        powers_of_2[0] = self;

        for i in 1..128 {
            let prev = powers_of_2[i - 1];
            powers_of_2[i] = prev.compose(prev);
        }

        powers_of_2
    }
}
