// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

pub mod primes;

pub fn fib (n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}
