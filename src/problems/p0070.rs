// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=70
//!
//! Totient Permutation
//!
//! Euler's totient function, phi(n) [sometimes called the phi function], is defined as
//! the number of positive integers not exceeding n which are relatively prime to n.
//! For example, as 1, 2, 4, 5, 7, and 8, are all less than or equal to nine and
//! relatively prime to nine, phi(9) = 6.
//!
//! The number 1 is considered to be relatively prime to every positive number, so phi(1) = 1.
//!
//! Interestingly, phi(87109) = 79180, and it can be seen that 87109 is a permutation of 79180.
//!
//! Find the value of n, 1 < n < 10^7, for which phi(n) is a permutation of n and
//! the ratio n / phi(n) produces a minimum.
//!

use crate::etc::sequences::primes::eratosthenes;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

///
/// The ratio 𝑛 ∕ 𝜑(𝑛) is minimized when 𝑛 is close to 𝜑(𝑛).
/// 𝑛 cannot be prime since 𝜑(𝑛) = 𝑛 − 1.
/// Semiprimes, however, will work. Two is best since using
/// more will just make the ratio larger.
///
///    𝑛 ∕ 𝜑(𝑛) ⇒ 𝑝₁·𝑝₂ ∕ 𝜑(𝑝₁·𝑝₂)
///                𝑝₁·𝑝₂ ∕ (𝑝₁ − 1)·(𝑝₂ − 1)
///                𝑝₁ ∕ (𝑝₁ − 1) ⨯ 𝑝₂ ∕ (𝑝₂ − 1)
///
/// To make the two ratios as close to 1 as possible,
/// 𝑝 must be as large as possible. Here are a few examples
/// to show that this is the case.
///
///    (3, 5)      ⇒   3 ∕ 2 ⨯ 5 ∕ 4 = 1.875
///    (17, 19)    ⇒   17 ∕ 16 ⨯ 19 ∕ 18 = 1.122
///    (251, 257)  ⇒   251 ∕ 250 ⨯ 257 ∕ 256 = 1.008
///
/// The largest primes that keep the ratio under 10⁷
/// are those near √10⁷ ≈ 3162.
///
fn iterative () -> u64 {
    let primes: Vec<_> = eratosthenes(5000).into_iter().filter(|&x| x > 2000).collect();
    let mut min_phi = 1u64;
    let mut r = 0;

    for (i, &p) in primes.iter().enumerate() {
    for &q in &primes[i + 1..] {
        let n = p * q;
        if n >= 10_000_000 { break; }

        let phi = (p - 1) * (q - 1);

        // We need to test whether 𝑛 ∕ 𝜑(𝑛) is less than the current minimum.
        // This can be done using cross-multiplication so as to avoid division.
        //
        //    𝑛 ∕ 𝜑(𝑛) < min-n ∕ min-phi
        //    𝑛 ⨯ min-phi < min-n ⨯ 𝜑(𝑛)
        //
        let b = r != 0 && (n as u128 * min_phi as u128) >= (r as u128 * phi as u128);
        if b { continue; }

        if !is_permutation(n, phi) { continue; }
        min_phi = phi;
        r = n;
    }}

    r
}

#[inline(always)]
fn is_permutation (a: u64, b: u64) -> bool {
    ///
    /// Divide up u64 into 10 6-bit fields
    /// which store the counts of the digits of 𝑛.
    ///
    #[inline(always)]
    fn hash (mut n: u64) -> u64 {
        let mut r = 0u64;
        while n > 0 {
            r += 1 << ((n % 10) * 6);
            n /= 10;
        }
        r
    }

    // Permutations have the same digit sum.
    //
    //    𝑎 ≡ 𝑏 (mod 9)
    //
    if a.abs_diff(b) % 9 != 0 { return false; }

    hash(a) == hash(b)
}
