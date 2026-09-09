// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

/// tau(n)
///
/// Factorize by trial division.
/// req: primes contains primes at least up to √𝑛
///
pub fn tau (mut n: usize, primes: &[usize]) -> usize {
    assert!(n > 0);

    let mut r = 1;

    for &p in primes {
        // Only necessary to test 𝑝 ≤ √𝑛.
        if p.saturating_mul(p) > n { break; }

        // Most 𝑝 do not divide 𝑛 so skip 𝑝 for which 𝑝 ∤ 𝑛.
        if n % p == 0 {
            // Determine the largest 𝑏 such that 𝑝ᵇ ∣ 𝑛.
            let mut b = 1;
            n /= p;
            while n % p == 0 {
                b += 1;
                n /= p;
            }
            r *= b + 1;
        }
    }

    // Any value 𝑛 greater than 1 must be prime
    // so it has two factors: itself and 1.
    if n > 1 { r *= 2; }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Use the following command to run all tests in this module:
    ///
    ///   cargo test --lib factorization::tests -- --nocapture
    ///

    const TAU: [usize; 100] = [
        1, 2, 2, 3, 2, 4, 2, 4, 3, 4, 2, 6, 2, 4, 4, 5, 2, 6, 2, 6, 4, 4, 2, 8,
        3, 4, 4, 6, 2, 8, 2, 6, 4, 4, 4, 9, 2, 4, 4, 8, 2, 8, 2, 6, 6, 4, 2, 10,
        3, 6, 4, 6, 2, 8, 4, 8, 4, 4, 2, 12, 2, 4, 6, 7, 4, 8, 2, 6, 4, 8, 2, 12,
        2, 4, 6, 6, 4, 8, 2, 10, 5, 4, 2, 12, 4, 4, 4, 8, 2, 12, 4, 6, 4, 4, 4,
        12, 2, 6, 6, 9,
    ];

    #[test]
    fn test () {
        use crate::sequences::primes::eratosthenes;

        let primes = &eratosthenes(100);
        let actual: Vec<_> = (1..=100).map(|n| tau(n, primes)).collect();
        assert_eq!(actual, TAU);
    }
}
