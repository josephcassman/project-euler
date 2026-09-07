// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

/// Simple Sieve of Eratosthenes
fn simple (limit: usize) -> Vec<usize> {
    if limit < 2 { return Vec::new(); }

    let mut r = vec![true; limit + 1];
    r[0] = false;
    r[1] = false;

    // Why is it necessary and sufficient to search for composites
    // up to a limit 𝑛 using primes up to √𝑛? This is a lemma
    // from number theory that can be shown as follows.
    //
    //   Assume 𝑛 is composite. Then 𝑛 = 𝑎·𝑏 for some integers
    //   𝑎 and 𝑏 that satisfy 1 < 𝑎 ≤ 𝑏 < 𝑛.
    //   Suppose that 𝑎 > √𝑛. Therefore,
    //
    //      √𝑛 < 𝑎 ≤ 𝑏       ⇒
    //      𝑎·𝑏 > √𝑛·√𝑛 = 𝑛  ⇒
    //      𝑛 > 𝑛
    //
    //   Since the assumption 𝑎 > √𝑛 resulted in a contradiction,
    //   combined with the fact that 𝑎 and 𝑏 both cannot be less
    //   than √𝑛 (as that would result in 𝑎·𝑏 = 𝑛 no longer being
    //   possible), it must be the case that 𝑎 ≤ √𝑛 ≤ 𝑏.
    //
    //   By the Fundamental Theorem of Arithmetic, either 𝑎 is prime
    //   or it has a prime factor 𝑝. We have 𝑎 ≤ √𝑛 for the first case
    //   and 𝑝 ≤ 𝑎 ≤ √𝑛 for the latter.
    //
    //   In either case, we see that there exists a prime factor
    //   of 𝑛 that is less than or equal to √𝑛. Or, stated conversely,
    //   a number 𝑛 with no prime factor less than or equal to √𝑛
    //   cannot be composite.

    let a = f64::sqrt(limit as f64) as usize;
    for p in 2..=a {
        if r[p] {
            // Cross out multiples starting from p * p
            for multiple in (p * p..=limit).step_by(p) {
                r[multiple] = false;
            }
        }
    }

    (2..=limit).filter(|&n| r[n]).collect()
}

/// Segmented Sieve of Eratosthenes
fn segmented (limit: usize) -> Vec<usize> {
    const CACHE_SIZE: usize = 65_536;

    match limit {
        0 | 1 => return Vec::new(),
        2 => return vec![2],
        3 => return vec![2, 3],
        _ => {}
    }

    // The simple sieve is used for values up to ⌊√limit⌋.
    // These primes are then used to identify subsequent composites.
    let base_size = f64::sqrt(limit as f64) as usize;
    let base = simple(base_size);

    // Use the Prime Number theorem to estimate
    // required storage capacity.
    //
    // Since 𝑛 / ln 𝑛 is less than 𝜋(𝑛) for 𝑛 ≥ 17,
    // use adjusted formulas that overestimate 𝜋.
    // The more accurate estimate for 𝑛 ≥ 67 comes from
    // the following inequality published in [1]:
    //
    //    𝑛 / (ln 𝑛 - 1/2) < 𝜋(𝑛) < 𝑛 / (ln 𝑛 - 3/2)
    //
    // [1] Rosser, J. Barkley, and Lowell Schoenfeld.
    //     "Approximate Formulas for Some Functions of Prime Numbers."
    //     Illinois Journal of Mathematics, vol. 6, no. 1, 1962, pp. 64–94.
    //     Project Euclid, https://doi.org/10.1215/ijm/1255631807.
    //
    let capacity = if limit < 67 { (limit / 2) + 1 } else {
        let x = limit as f64;
        (x / (f64::ln(x) - 1.5)).ceil() as usize
    };
    let mut r = Vec::with_capacity(capacity);
    r.extend_from_slice(&base);

    // segment represents the next sequence of values to sieve.
    // Keep it in L1 cache.
    let mut segment = vec![true; usize::max(CACHE_SIZE, base_size)];

    // a identifies the next candidate prime.
    // It will be incremented until it hits the limit.
    let mut a = base_size + 1;
    while a <= limit {
        // b is the largest candidate prime in the segment.
        let b = usize::min(a.saturating_add(segment.len() - 1), limit);

        // Reset the segment scratchpad.
        // Use a slice instead of accessing the segment directly
        // to keep the logic the same for the final iteration.
        let x = &mut segment[..(b - a + 1)];
        x.fill(true);

        // Use each prime in the necessary and sufficient base
        // to identify composite numbers greater than a.
        for &p in &base {
            //
            // The value of a mod p, the remainder, represents
            // how far a is passed the previous multiple of p.
            // So the value of a sits in the interval
            //
            //    [a - rem, a - rem + p]
            //    [a - rem, a + (p - rem)]
            //
            // Example. The current value to evaluate a is 23,
            //   that is, the segment starts at 23. Right now
            //   we are crossing out multiples of 7 (composites).
            //
            //    rem = 23 mod 7 = 2
            //    [23 - 2, 23 + (7 - 2)] = [21, 28]
            //    The value 28 is the next multiple of p = 7.
            //
            let rem = a % p;
            let mut m = if rem == 0 { a } else { a + (p - rem) };

            // We can skip past multiples of p less than p²
            // because a composite less than p² must have at least
            // one prime factor small than p. Smaller primes
            // processed up until this point would have already
            // identified such composites.
            if m < p * p { m = p * p; }

            // Mark each multiple of p as a composite.
            while m <= b {
                x[m - a] = false;
                m += p;
            }
        }

        // Add all primes from the current segment to the result.
        for (i, &is_prime) in x.iter().enumerate() {
            if is_prime {
                r.push(a + i);
            }
        }

        if b == limit { break; }

        // Skip to the next window.
        a = b + 1;
    }

    r
}

pub struct Primes {
    buf: Vec<usize>,
    start: usize,
    end: usize,
}

impl Primes {
    pub fn new (limit: usize) -> Self {
        let buf = segmented(limit);
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

    #[test]
    fn test_primes () {
        let expected = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31,
            37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
            83, 89, 97, 101, 103, 107, 109, 113, 127, 131,
            137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229,
            233, 239, 241, 251, 257, 263, 269, 271,
        ];

        let primes_res: Vec<_> = Primes::new(500).take(expected.len()).collect();
        assert_eq!(primes_res, expected);
    }
}
