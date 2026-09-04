// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=3
//!
//! Largest Prime Factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143?
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> usize {
    use project_euler::primes::Primes;

    const N: usize = 600_851_475_143;
    let mut r = 0;

    let a = (N as f64).sqrt() as usize;
    let primes = Primes::new(a);
    for p in primes.rev() {
        if N % p == 0 {
            r = p;
            break;
        }
    }

    r
}
