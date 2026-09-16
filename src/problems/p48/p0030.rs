// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=30
//!
//! Digit Fifth Powers
//!
//! Surprisingly there are only three numbers that can be written as
//! the sum of fourth powers of their digits:
//!
//!    1634 = 1^4 + 6^4 + 3^4 + 4^4
//!    8208 = 8^4 + 2^4 + 0^4 + 8^4
//!    9474 = 9^4 + 4^4 + 7^4 + 4^4
//!
//! As 1 = 1^4 is not a sum it is not included.
//!
//! The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//!
//! Find the sum of all the numbers that can be written as
//! the sum of fifth powers of their digits.
//!

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let mut r: u64 = 0;

    for n in 2..10_000_000 {
        if n == pow5(n) {
            r += n;
        }
    }

    r
}

#[inline(always)]
fn pow5 (n: u64) -> u64 {
    n.to_string()
     .chars()
     .map(|x| x.to_digit(10).unwrap() as u64)
     .fold(0, |acc, x| acc + x.pow(5))
}
