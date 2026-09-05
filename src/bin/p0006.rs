// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=6
//!
//! Sum Square Difference
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//!    pow(1, 2) + pow(2, 2) + ... + pow(10, 2) = 385.
//!
//! The square of the sum of the first ten natural numbers is,
//!
//!   pow(1 + 2 + ... + 10, 2) = pow(55, 2) = 3025.
//!
//! Hence the difference between the sum of the squares of
//! the first ten natural numbers and the square of the sum is
//! 3025 - 385 = 2640.
//!
//! Find the difference between the sum of the squares of
//! the first one hundred natural numbers and the square of the sum.
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    let mut sum = 0;
    let mut sum_of_squares = 0;

    for a in 1..=100 {
        sum += a;
        sum_of_squares += a * a;
    }

    sum * sum - sum_of_squares
}
