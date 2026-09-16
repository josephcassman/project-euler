// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=41
//!
//! Pandigital Prime
//!
//! We shall say that an n-digit number is pandigital if it makes use of
//! all the digits 1 to n exactly once. For example, 2143 is a 4-digit
//! pandigital and is also prime.
//!
//! What is the largest n-digit pandigital prime that exists?
//!

use crate::etc::sequences::primes::Primes;
use crate::etc::pandigital::is_n_pandigital;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    let mut r = 0;

    for p in Primes::new(987654321).filter(|&x| x > 2143) {
        if is_n_pandigital(p) { r = p; }
    }

    r
}
