// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=69
//!
//! Totient Maximum
//!
//! Euler's totient function, phi(n) [sometimes called the phi function], is defined as
//! the number of positive integers not exceeding n which are relatively prime to n.
//! For example, as 1, 2, 4, 5, 7, and 8, are all less than or equal to nine and
//! relatively prime to nine, phi(9)=6.
//!
//!    𝑛   Relatively Prime   𝜑(𝑛)   𝑛∕𝜑(𝑛)
//!    2     1                 1       2
//!    3     1,2               2       1.5
//!    4     1,3               2       2
//!    5     1,2,3,4           4       1.25
//!    6     1,5               2       3
//!    7     1,2,3,4,5,6       6       1.1666...
//!    8     1,3,5,7           4       2
//!    9     1,2,4,5,7,8       6       1.5
//!    10    1,3,7,9           4       2.5
//!
//! It can be seen that n = 6 produces a maximum n/phi(n) for n ≤ 10.
//!
//! Find the value of n ≤ 1,000,000 for which n/phi(n) is a maximum.
//!

use crate::etc::sequences::primes::eratosthenes;
use crate::etc::factorization::phi;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    let primes = eratosthenes(1_000_000);
    let mut max_ratio = f64::MIN;
    let mut r = 0;

    for n in 1..=1_000_000 {
        let a = (n as f64) / (phi(n, &primes) as f64);
        if a > max_ratio {
            max_ratio = a;
            r = n;
        }
    }

    r
}
