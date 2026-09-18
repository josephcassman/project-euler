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
    let mut min_phi = 1u64;
    let mut r = 0;

    for n in 2..10_000_000 {
        let a = phi(n, &primes);

        // We need to test whether 𝑛 ∕ 𝜑(𝑛) is less than the current minimum.
        // This can be done using cross-multiplication so as to avoid division.
        //
        //    𝑛 ∕ 𝜑(𝑛) < min-n ∕ min-phi
        //    𝑛 ⨯ min-phi < min-n ⨯ 𝜑(𝑛)
        //
        let b = r != 0 && (n as u128 * min_phi as u128) >= (r as u128 * a as u128);
        if b { continue; }

        if !is_permutation(n, a) { continue; }
        min_phi = a;
        r = n;
    }

    r
}

#[inline(always)]
fn is_permutation (a: u64, b: u64) -> bool {
    ///
    /// Divide up u64 into 10 6-bit fields
    /// which store the counts of the digits of 𝑛.
    ///
    #[inline(always)]
    fn hash (mut n: u64) -> u64 {
        let mut r = 0u64;
        while n > 0 {
            r += 1 << ((n % 10) * 6);
            n /= 10;
        }
        r
    }

    // Permutations have the same digit sum.
    //
    //    𝑎 ≡ 𝑏 (mod 9)
    //
    if a.abs_diff(b) % 9 != 0 { return false; }

    hash(a) == hash(b)
}
