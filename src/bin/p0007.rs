// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=7
//!
//! 10001st Prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//! we can see that the 6th prime is 13.
//!
//! What is the 10001st prime number?
//!

fn main () {
    println!("\niterative method: {}", iterative());
    println!("\ntrial division method: {}\n", division());
}

fn iterative () -> usize {
    use project_euler::sequences::primes::Primes;

    let mut r = Some(0);
    let mut primes = Primes::new(10_000_000);

    for _i in 1..=10_001 { r = primes.next(); }

    r.unwrap()
}

/// Trial division is fast enough for this scale.
///
/// The number of values to test can be greatly reduced
/// by making use of the following observation:
///
///    only numbers of the form 6·k ± 1 can be prime
///
///    k ≡ 0 (mod 6) ⇒ multiples of 6 (divisible by 2 and 3)
///    k ≡ 1 (mod 6) ⇒ potential primes
///    k ≡ 2 (mod 6) ⇒ even numbers (divisible by 2)
///    k ≡ 3 (mod 6) ⇒ multiples of 3
///    k ≡ 4 (mod 6) ⇒ even numbers (divisible by 2)
///    k ≡ 5 (mod 6) ⇒ potential primes
///
/// Algorithm:
///    For k ≡ 1 or 5 (mod 6) divide k by all primes up to ⌊√k⌋.
///    Increment a counter whenever k has no divisors.
///    The answer is when the counter equals 10_001.
///
///    NOTE: It is sufficient to divide until ⌊√k⌋ because
///          composite numbers all have a factor
///          less than or equal to this value.
///
fn division () -> u64 {
    use project_euler::sequences::six_mod_one::SixModOne;

    const N: usize = 10_001;
    let mut primes: Vec<u64> = Vec::with_capacity(N);
    primes.push(2);
    primes.push(3);

    'outer: for k in SixModOne::new() {
        let limit = u64::isqrt(k);

        for &p in &primes[2..] {
            if p > limit { break; }
            if k % p == 0 { continue 'outer; }
        }

        primes.push(k);
        if primes.len() == N {
            return k;
        }
    }

    unreachable!()
}
