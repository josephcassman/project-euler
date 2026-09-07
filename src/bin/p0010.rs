// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=10
//!
//! Summation of Primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> usize {
    use project_euler::sequences::primes::Primes;

    let mut sum = 0;

    for a in Primes::new(2_000_000) {
        sum += a;
    }

    sum
}
