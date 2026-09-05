// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

pub mod discrete_math;
pub mod primes;

pub fn fib (n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

pub fn is_palindrome (a: &str) -> bool {
    let mut chars = a.chars();
    while let (Some(x), Some(y)) = (chars.next(), chars.next_back()) {
        if x != y {
            return false;
        }
    }
    true
}
