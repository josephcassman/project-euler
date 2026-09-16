// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=43
//!
//! Sub-string Divisibility
//!
//! The number, 1406357289, is a 0 to 9 pandigital number because it is made up of
//! each of the digits 0 to 9 in some order, but it also has a rather interesting
//! sub-string divisibility property.
//!
//! Let d(1) be the 1ˢᵗ digit, d(2) be the 2ⁿᵈ digit, and so on. In this way, we note the following:
//!
//!    d(2)d(3)d(4) = 406 is divisible by 2
//!    d(3)d(4)d(5) = 063 is divisible by 3
//!    d(4)d(5)d(6) = 635 is divisible by 5
//!    d(5)d(6)d(7) = 357 is divisible by 7
//!    d(6)d(7)d(8) = 572 is divisible by 11
//!    d(7)d(8)d(9) = 728 is divisible by 13
//!    d(8)d(9)d(10) = 289 is divisible by 17
//!
//! Find the sum of all 0 to 9 pandigital numbers with this property.
//!

use crate::etc::permute;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    const VALID: [u64; 7] = [0; 7];
    let mut r = 0;

    let mut digits: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    while permute(&mut digits) {
        let a = [
            d2n(&digits[1..=3]) % 2,  // d(2)d(3)d(4)
            d2n(&digits[2..=4]) % 3,  // d(3)d(4)d(5)
            d2n(&digits[3..=5]) % 5,  // d(4)d(5)d(6)
            d2n(&digits[4..=6]) % 7,  // d(5)d(6)d(7)
            d2n(&digits[5..=7]) % 11, // d(6)d(7)d(8)
            d2n(&digits[6..=8]) % 13, // d(7)d(8)d(9)
            d2n(&digits[7..=9]) % 17, // d(8)d(9)d(10)
        ];

        if a == VALID { r += d2n(&digits); }
    }

    r
}

#[inline(always)]
fn d2n (digits: &[u8]) -> u64 {
    digits.iter().fold(0u64, |acc, &x| acc * 10 + x as u64)
}
