// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=5
//!
//! Smallest Multiple
//!
//! 2520 is the smallest number that can be divided by
//! each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is __evenly divisible__
//! by all of the numbers from 1 to 20?
//!

use crate::etc::discrete_math::lcm;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    let numbers: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    match numbers {
        [] => 0,
        [first, rest @ ..] => rest.iter().fold(*first, |acc, &x| lcm(acc, x)),
    }
}
