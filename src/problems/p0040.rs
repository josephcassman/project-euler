// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=40
//!
//! Champernowne's Constant
//!
//! An irrational decimal fraction is created by concatenating the positive integers:
//!
//!    0.12345678910 !1! 112131415161718192021
//!
//! It can be seen that the 12ᵗʰ digit of the fractional part is 1.
//!
//! If d(n) represents the nᵗʰ digit of the fractional part, find the value of the following expression.
//!
//!    d(1) ⨯ d(10) ⨯ d(100) ⨯ d(1000) ⨯ d(10000) ⨯ d(100000) ⨯ d(1000000)
//!

use crate::etc::bcd::Bcd;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    let mut r = [0u64; 7];
    let mut a = 1;
    let mut i = 1;

    while i <= 1_000_000 {
        let b = Bcd::from_u64(a).unwrap();
        for j in (0..b.len()).rev() {
            match i {
                1       => r[0] = b.digit(j).unwrap(),
                10      => r[1] = b.digit(j).unwrap(),
                100     => r[2] = b.digit(j).unwrap(),
                1000    => r[3] = b.digit(j).unwrap(),
                10000   => r[4] = b.digit(j).unwrap(),
                100000  => r[5] = b.digit(j).unwrap(),
                1000000 => r[6] = b.digit(j).unwrap(),
                _ => {}
            }

            i += 1;
        }
        a += 1;
    }

    r.iter().product()
}
