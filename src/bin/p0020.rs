// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=20
//!
//! Factorial Digit Sum
//!
//! n! means n ⨯ (n - 1) ⨯ ... ⨯ 3 ⨯ 2 ⨯ 1.
//!
//! For example, 10! = 10 ⨯ 9 ⨯ ... ⨯ 3 ⨯ 2 ⨯ 1 = 3628800,
//! and the sum of the digits in the number 10! is
//! 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!.
//!

use num_bigint::BigUint;
use num_traits::One;

fn main () {
    println!("\ncalculation method: {}\n", calculation());
}

fn calculation () -> u64 {
    factorial(100)
    .to_string()
    .bytes()
    .map(|x| (x - b'0') as u64)
    .sum::<u64>()
}

fn factorial (n: u64) -> BigUint {
    (1..=n).fold(BigUint::one(), |acc, x| acc * x)
}
