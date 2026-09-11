// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimeFactor {
    pub p: u64,
    pub count: usize,
}

/// Prime factors of 𝑛
/// req: primes contains primes at least up to √𝑛
///
pub fn factorize (mut n: u64, primes: &[u64]) -> Vec<PrimeFactor> {
    assert!(n > 0);

    let mut r = Vec::new();

    for &p in primes {
        // Only necessary to test 𝑝 ≤ √𝑛.
        if p.saturating_mul(p) > n { break; }

        // Most 𝑝 do not divide 𝑛 so skip 𝑝 for which 𝑝 ∤ 𝑛.
        if n % p == 0 {
            // Determine the largest 𝑏 such that 𝑝ᵇ ∣ 𝑛.
            let mut count = 0;
            while n % p == 0 {
                count += 1;
                n /= p;
            }
            r.push(PrimeFactor { p, count });
        }
    }

    // A remaining value of 𝑛 greater than 1 must be prime
    if n > 1 {
        r.push(PrimeFactor { p: n, count: 1 });
    }

    r
}

/// 𝜏(𝑛) or 𝜎₀(𝑛)
/// the number of divisors of 𝑛
///
/// Factorize by trial division.
/// req: primes contains primes at least up to √𝑛
///
pub fn tau (n: u64, primes: &[u64]) -> usize {
    assert!(n > 0);
    factorize(n, primes).iter().fold(1, |acc, x| acc * (x.count + 1))
}

/// 𝜎(𝑛) or 𝜎₁(𝑛)
/// the sum of the divisors of 𝑛
///
/// This sieve determines which numbers an integer 𝑑
/// divides and then adds 𝑑 to all of its multiples.
///
pub fn sigma_sieve (limit: usize) -> Vec<u64> {
    let mut r = vec![0u64; limit + 1];

    for a in 1..=limit {
        for b in (a..=limit).step_by(a) {
            r[b] += a as u64;
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::etc::sequences::primes::eratosthenes;

    #[test]
    fn test_factorize () {
        fn pf (p: u64, count: usize) -> PrimeFactor { PrimeFactor { p, count } }

        let primes = &eratosthenes(100);
        assert_eq!(factorize(2, primes), vec![pf(2, 1)]);
        assert_eq!(factorize(3, primes), vec![pf(3, 1)]);
        assert_eq!(factorize(12, primes), vec![pf(2, 2), pf(3, 1)]);
        assert_eq!(factorize(30, primes), vec![pf(2, 1), pf(3, 1), pf(5, 1)]);
        assert_eq!(factorize(1960, primes), vec![pf(2, 3), pf(5, 1), pf(7, 2)]);
    }

    #[test]
    fn test_tau () {
        let expected = [
            1, 2, 2, 3, 2, 4, 2, 4, 3, 4, 2, 6, 2, 4, 4, 5, 2, 6, 2, 6, 4, 4, 2, 8,
            3, 4, 4, 6, 2, 8, 2, 6, 4, 4, 4, 9, 2, 4, 4, 8, 2, 8, 2, 6, 6, 4, 2, 10,
            3, 6, 4, 6, 2, 8, 4, 8, 4, 4, 2, 12, 2, 4, 6, 7, 4, 8, 2, 6, 4, 8, 2, 12,
            2, 4, 6, 6, 4, 8, 2, 10, 5, 4, 2, 12, 4, 4, 4, 8, 2, 12, 4, 6, 4, 4, 4,
            12, 2, 6, 6, 9,
        ];

        let primes = &eratosthenes(100);
        let actual: Vec<_> = (1u64..=100u64).map(|n| tau(n, primes)).collect();
        assert_eq!(actual, expected);
    }
}
