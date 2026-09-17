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
        if !is_permutation(n, a) { continue; }
        let b = (n as f64) / (a as f64);
        if b < min_ratio {
            min_ratio = b;
            r = n;
        }
    }

    r
}

fn is_permutation (mut a: u64, mut b: u64) -> bool {
    let mut count_a = [0u8; 10];
    let mut count_b = [0u8; 10];

    while a > 0 {
        count_a[(a % 10) as usize] += 1;
        a /= 10;
    }

    while b > 0 {
        count_b[(b % 10) as usize] += 1;
        b /= 10;
    }

    count_a == count_b
}
