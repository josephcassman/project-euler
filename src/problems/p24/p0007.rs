// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=7
//!
//! 10001st Prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//! we can see that the 6th prime is 13.
//!
//! What is the 10001st prime number?
//!

use crate::etc::sequences::primes::Primes;

pub fn run () {
    println!("\niterative method: {}\n", sieve());
}

fn sieve () -> u64 {
    // Get the number at index 10_000 since
    // Iterator::nth uses zero-based indexing.
    Primes::new(10_000_000).nth(10_000).unwrap()
}
