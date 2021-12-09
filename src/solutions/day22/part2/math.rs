/// Approach taken from wikipedia:
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Extended_Euclidean_algorithm
pub fn inverse(a: u128, modulus: u128) -> u128 {
    let a = a as i128;
    let modulus = modulus as i128;

    let (x, _y) = extended_gcd(a, modulus);

    x.rem_euclid(modulus) as u128
}

/// Cribbed straight from wikipedia.
/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd(a: i128, b: i128) -> (i128, i128) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);

    while r.1 != 0 {
        let quotient = r.0 / r.1;
        r = (r.1, r.0 - quotient * r.1);
        s = (s.1, s.0 - quotient * s.1);
        t = (t.1, t.0 - quotient * t.1);
    }

    let bezout_coefficients = (s.0, t.0);
    let _gcd = r.0;
    let _quotients_by_the_gcd = (t.1, s.1);

    bezout_coefficients
}

#[allow(unused)]
fn gcd(a: u128, b: u128) -> u128 {
    assert!(a != 0 || b != 0);
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        for (a, a_inv, modulus) in [
            // Trivial: 1 * 1 = 1.
            (1, 1, 2),
            (1, 1, 3),
            (1, 1, 5),

            (2, 2, 3),
            (2, 3, 5),
            (3, 2, 5),
            (4, 4, 5),
        ] {
            let actual = inverse(a, modulus);
            assert_eq!(a_inv, actual);
        }
    }
}
