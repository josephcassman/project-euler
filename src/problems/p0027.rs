// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=27
//!
//! Quadratic Primes
//!
//! Euler discovered the remarkable quadratic formula:
//!
//!    n^2 + n + 41
//!
//! It turns out that the formula will produce 40 primes for the consecutive integer values
//! 0 ≤ n ≤ 39. However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41,
//! and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
//!
//! The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for
//! the consecutive values 0 ≤ n ≤ 79. The product of the coefficients, -79 and 1601, is -126479.
//!
//! Considering quadratics of the form:
//!
//!    n^2 + an + b, where |a| < 1000 and |b| ≤ 1000
//!
//!    where |n| is the modulus/absolute value of n
//!    e.g. |11| = 11 and |-4| = 4
//!
//! Find the product of the coefficients, a and b, for the quadratic expression
//! that produces the maximum number of primes for consecutive values of n,
//! starting with n = 0.
//!

use crate::etc::sequences::primes::eratosthenes;

pub fn run () {
    println!("\nbrute-force method: {:?}\n", brute_force());
}

///
/// run with release mode on:
///
///    cargo run --release -- 27
///
fn brute_force () -> i64 {
    #[inline(always)]
    fn f (n: i64, a: i64, b: i64) -> i64 {
        // 𝑛·(𝑛 + 𝑎) + 𝑏
        n * (n + a) + b
    }

    let primes = eratosthenes(100_000);
    let mut max_length = 0;
    let mut max_a = 0;
    let mut max_b = 0;

    for a in -999..=999 {
    for b in -1000..=1000 {
        let mut delta = 0;
        for n in 0..1_000 {
            let x = f(n, a, b);
            if x < 2 || !primes.contains(&(x as u64)) { break; }
            delta += 1;
        }
        if delta > max_length {
            max_length = delta;
            max_a = a;
            max_b = b;
        }
    }}

    max_a * max_b
}
