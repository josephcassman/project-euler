// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

/// Sieve of Eratosthenes
pub fn eratosthenes (limit: usize) -> Vec<usize> {
    if limit < 2 { return Vec::new(); }

    let mut r = vec![true; limit + 1];
    r[0] = false;
    r[1] = false;

    let sqrt = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt {
        if r[p] {
            // Cross out multiples starting from p * p
            for multiple in (p * p..=limit).step_by(p) {
                r[multiple] = false;
            }
        }
    }

    (2..=limit).filter(|&n| r[n]).collect()
}

pub fn is_prime (a: usize) -> bool {
    eratosthenes(a).last() == Some(&a)
}

pub struct Primes {
    buf: Vec<usize>,
    start: usize,
    end: usize,
}

impl Primes {
    pub fn new (limit: usize) -> Self {
        let buf = eratosthenes(limit);
        let end = buf.len();
        Self { buf, start: 0, end, }
    }
}

impl Default for Primes {
    fn default () -> Self {
        Self::new(64)
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next (&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let prime = self.buf[self.start];
            self.start += 1;
            Some(prime)
        }
        else { None }
    }

    fn size_hint (&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for Primes {
    fn next_back (&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            Some(self.buf[self.end])
        }
        else { None }
    }
}

impl ExactSizeIterator for Primes {}

#[cfg(test)]
mod tests {
    use super::*;

    const KNOWN_PRIMES_50: &[usize] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
    ];

    #[test]
    fn test_known_small_primes () {
        let primes_res: Vec<_> = Primes::default().take(KNOWN_PRIMES_50.len()).collect();
        assert_eq!(primes_res, KNOWN_PRIMES_50);

        let eratosthenes_res: Vec<_> = eratosthenes(64).into_iter().take(KNOWN_PRIMES_50.len()).collect();
        assert_eq!(eratosthenes_res, KNOWN_PRIMES_50);
    }
}
