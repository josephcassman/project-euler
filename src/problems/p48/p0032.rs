// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=32
//!
//! Pandigital Products
//!
//! We shall say that an n-digit number is pandigital if
//! it makes use of all the digits 1 to n exactly once; for example,
//! the 5-digit number, 15234, is 1 through 5 pandigital.
//!
//! The product 7254 is unusual, as the identity, 39 ⨯ 186 = 7254, containing
//! multiplicand, multiplier, and product is 1 through 9 pandigital.
//!
//! Find the sum of all products whose multiplicand/multiplier/product identity
//! can be written as a 1 through 9 pandigital.
//!
//! HINT: Some products can be obtained in more than one way so be sure
//! to only include it once in your sum.
//!

use std::collections::HashSet;
use crate::etc::{join_digits, permute};

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let mut digits: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut numbers = HashSet::new();
    while permute(&mut digits) {
        // iterate through all combinations of two pivots
        //
        //     i        j
        //     ↓        ↓
        // [1, 2, 3, 4, 5, 6, 7, 8, 9]
        //
        for i in 0..=9 {
        for j in i..=9 {
            let left = &digits[..i];
            let middle = &digits[i..j];
            let right = &digits[j..];

            let a = join_digits(left);
            let b = join_digits(middle);
            let c = join_digits(right);

            if a * b == c {
                numbers.insert(c);
            }
        }}
    }

    numbers.iter().sum()
}
