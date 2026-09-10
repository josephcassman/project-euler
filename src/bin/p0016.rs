// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=16
//!
//! Power Digit Sum
//!
//! 2¹⁵ = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2¹⁰⁰⁰?
//!

use std::str::FromStr;
use num_bigint::BigUint;

fn main () {
    println!("\nbrute-force method: {}\n", brute_force());
}

fn brute_force () -> u64 {
    let a = BigUint::from_str("2").unwrap();
    let b = a.pow(1000);
    b.to_string()
     .bytes()
     .map(|x| (x - b'0') as u64)
     .sum()
}
