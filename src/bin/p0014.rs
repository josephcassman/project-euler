// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=14
//!
//! Longest Collatz Sequence
//!
//! The following iterative sequence is defined for the set of positive integers:</p>
//!
//!    n → n/2 (n is even)
//!    n → 3n + 1 (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//!
//!    13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1.
//!
//! It can be seen that this sequence (starting at 13 and finishing at 1)
//! contains 10 terms. Although it has not been proved yet (Collatz Problem),
//! it is thought that all starting numbers finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! NOTE: Once the chain starts the terms are allowed to go above one million.
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn collatz (n: u64) -> u64 {
    if n % 2 == 0 { n / 2 }
    else { 3 * n + 1 }
}

fn iterative () -> u64 {
    let mut r: u64 = 0;
    let mut max: u64 = 0;

    for n in 2..1_000_000 {
        let mut a = n;
        let mut reps = 0;
        while a != 1 {
            a = collatz(a);
            reps += 1;
        }
        if reps > max {
            r = n;
            max = reps;
        }
    }

    r
}
