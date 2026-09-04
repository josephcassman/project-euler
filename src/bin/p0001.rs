// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=1
//!
//! If we list all the natural numbers below $10$ that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!

fn main () {
    let mut sum: u32 = 0;

    let mut three: u32 = 3;
    while three < 1_000 {
        sum += three;
        three += 3;
    }

    let mut five: u32 = 5;
    while five < 1_000 {
        if five % 3 != 0 { sum += five; }
        five += 5;
    }

    println!("\n{}", sum);
}
