// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=35
//!
//! Circular Primes
//!
//! The number, 197, is called a circular prime because all rotations of
//! the digits: 197, 971, and 719, are themselves prime.
//!
//! There are thirteen such primes below 100:
//!    2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//!
//! How many circular primes are there below one million?
//!

use crate::etc::{join_digits, split_digits};
use crate::etc::sequences::primes::eratosthenes;

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let primes = eratosthenes(1_000_000);
    let mut r = 0;

    'outer: for &p in primes.iter() {
        let mut rotate = rol();
        let mut digits = split_digits(p);
        while rotate(&mut digits) {
            let a = join_digits(&digits);
            if !primes.contains(&a) { continue 'outer; }
        }
        r += 1;
    }

    r
}

/// rotate left
/// ret: back at the beginning?
fn rol () -> impl FnMut (&mut [u8]) -> bool {
    let mut count = 0;
    move |a: &mut [u8]| {
        let len = a.len();
        if len == 0 || count >= len { return false; }
        a.rotate_left(1);
        count += 1;
        count < len
    }
}
