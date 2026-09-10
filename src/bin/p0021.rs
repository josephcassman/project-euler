// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=21
//!
//! Amicable Numbers
//!
//! Let d(n) be defined as the sum of proper divisors of n
//! (numbers less than n which divide evenly into n).
//!
//! If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair
//! and each of a and b are called amicable numbers.
//!
//! For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22,
//! 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are
//! 1, 2, 4, 71 and 142; so d(284) = 220.
//!
//! Evaluate the sum of all the amicable numbers under 10000.
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    use project_euler::factorization::sigma_sieve;

    let sigma = sigma_sieve(1_000_000);

    // proper divisor sum (n) = sigma(n) - n
    let proper_divisor_sum = |x: usize| -> usize {
        if x >= sigma.len() { 0 }
        else { (sigma[x] as usize).saturating_sub(x) }
    };

    let mut sum = 0;

    for a in 2..10_000 {
        let b = proper_divisor_sum(a);

        // a is a perfect, not an amicable number
        if a == b { continue; }

        let c = proper_divisor_sum(b);
        if c == a {
            sum += a as u64;
        }
    }

    sum
}
