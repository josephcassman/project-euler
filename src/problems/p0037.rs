// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=37
//!
//! Truncatable Primes
//!
//! The number 3797 has an interesting property. Being prime itself, it is possible
//! to continuously remove digits from left to right, and remain prime at each stage:
//! 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
//!
//! Find the sum of the only eleven primes that are both truncatable
//! from left to right and right to left.
//!
//! NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
//!

use crate::etc::{join_digits, split_digits};
use crate::etc::sequences::primes::eratosthenes;

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let primes = eratosthenes(1_000_000);
    let mut count = 0;
    let mut r = 0;

    for &p in primes.iter().skip(4) {
        if is_truncatable(p, &primes) {
            r += p;
            count += 1;
            if count >= 11 { break; }
        }
    }

    r
}

fn is_truncatable (p: u64, primes: &[u64]) -> bool {
    let a = split_digits(p);
    let b = a.clone();

    let mut x = a.as_slice();
    while let [_, rest @ ..] = x {
        let q = join_digits(x);
        if !primes.binary_search(&q).is_ok() { return false; }
        x = rest;
    }

    let mut y = b.as_slice();
    while let [rest @ .., _] = y {
        let q = join_digits(y);
        if !primes.binary_search(&q).is_ok() { return false; }
        y = rest;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_truncatable () {
        let primes = eratosthenes(10_000);

        assert!(is_truncatable(3797, &primes));
    }
}
