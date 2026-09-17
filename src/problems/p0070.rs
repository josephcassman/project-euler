// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=70
//!
//! Totient Permutation
//!
//! Euler's totient function, phi(n) [sometimes called the phi function], is defined as
//! the number of positive integers not exceeding n which are relatively prime to n.
//! For example, as 1, 2, 4, 5, 7, and 8, are all less than or equal to nine and
//! relatively prime to nine, phi(9) = 6.
//!
//! The number 1 is considered to be relatively prime to every positive number, so phi(1) = 1.
//!
//! Interestingly, phi(87109) = 79180, and it can be seen that 87109 is a permutation of 79180.
//!
//! Find the value of n, 1 < n < 10^7, for which phi(n) is a permutation of n and
//! the ratio n / phi(n) produces a minimum.
//!

use crate::etc::sequences::primes::eratosthenes;
use crate::etc::factorization::phi;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    let primes = eratosthenes(10_000_000);
    let mut min_ratio = f64::INFINITY;
    let mut r = 0;

    for n in 2..10_000_000 {
        let a = phi(n, &primes);
        if !is_permutation(&n.to_string(), &a.to_string()) { continue; }
        let b = (n as f64) / (a as f64);
        if b < min_ratio {
            min_ratio = b;
            r = n;
        }
    }

    r
}

fn is_permutation (a: &str, b: &str) -> bool {
    let mut x: Vec<char> = a.chars().collect();
    let mut y: Vec<char> = b.chars().collect();

    x.sort_unstable();
    y.sort_unstable();

    x == y
}
