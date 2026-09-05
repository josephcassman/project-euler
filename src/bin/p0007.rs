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

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> usize {
    use project_euler::primes::Primes;

    let mut r = Some(0);
    let mut primes = Primes::new(10_000_000);

    for _i in 1..=10_001 { r = primes.next(); }

    r.unwrap()
}
