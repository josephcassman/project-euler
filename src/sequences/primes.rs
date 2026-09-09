// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

/// Estimate of the nth prime
///
/// Since 𝑛 / ln 𝑛 is less than 𝜋(𝑛) for 𝑛 ≥ 17,
/// use adjusted formulas that overestimate 𝜋.
/// The more accurate estimate for 𝑛 ≥ 67 comes from
/// the following inequality published in [1]:
///
///    𝑛 / (ln 𝑛 - 1/2) < 𝜋(𝑛) < 𝑛 / (ln 𝑛 - 3/2)
///
/// [1] Rosser, J. Barkley, and Lowell Schoenfeld.
///     "Approximate Formulas for Some Functions of Prime Numbers."
///     Illinois Journal of Mathematics, vol. 6, no. 1, 1962, pp. 64–94.
///     Project Euclid, https://doi.org/10.1215/ijm/1255631807.
///
fn pi (n: usize) -> usize {
    if n < 67 { (n / 2) + 1 }
    else {
        let x = n as f64;
        (x / (f64::ln(x) - 1.5)).ceil() as usize
    }
}

/// Simple Sieve of Eratosthenes
/// Uses a mod 2 wheel to identify composites.
///
fn simple (limit: usize) -> Vec<usize> {
    if limit < 2 { return Vec::new(); }

    //
    // 0 = prime, 1 = composite
    //
    // The algorithm uses a wheel mod 2:
    //
    //    number = { 2·𝑘 + 1 | 𝑘 ∈ ℤ ∧ 𝑘 ≥ 0 }
    //    k = number / 2 (integral division)
    //
    // Bits translate to numbers using 𝑘:
    //
    //    k / 64 → word index = index
    //    k % 64 → bit offset = offset
    //
    // These formulas are used to access the
    // individual bit of a number as shown below:
    //
    //    is bit clear?  ⇒  x[index] & (1 << offset) == 0
    //    set bit        ⇒  x[index] |= 1 << offset
    //
    let mut x = vec![0u64; (limit / 2) / 64 + 1];
    x[0] = 1; // k = 0 → number 1 is composite

    // Why is it sufficient to search for composites up to
    // a limit 𝑛 using primes up to √𝑛? This is a lemma
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

    for p in (3..=usize::isqrt(limit)).step_by(2) {
        let p_k = p / 2;

        // Is bit p clear? ⇒ Is p prime?
        if (x[p_k / 64] & (1u64 << (p_k % 64))) == 0 {
            // Cross out odd multiples.
            //
            // Iterating over k (indices of multiples)
            // instead of multiples of p allows a division
            // to be pulled out of the inner loop:
            //
            // Iterate over multiples of p:
            //
            //    multiple ∈ [p², limit] step by 2 * p
            //       k = multiple / 2
            //       set bit k
            //
            // Iterate over k (indices of multiples):
            //
            //    k ∈ [p² / 2, limit / 2] step by p
            //       set bit k
            //

            let alpha_k = (p * p) / 2;
            let beta_k = (limit - 1) / 2;

            for m_k in (alpha_k..=beta_k).step_by(p) {
                x[m_k / 64] |= 1u64 << (m_k % 64);
            }
        }
    }

    // Count the number of bits in x to get
    // the capacity for the result.
    //
    // A = the number of even primes = 1
    // B = odd number count = (limit - 1) / 2 + 1
    // C = odd non-primes count = sum of bit count of each word in x
    // D = odd primes = odd number count - odd non-primes count = B - C
    //
    // Capacity = A + D
    //          = 1 + ((limit - 1) / 2 + 1) - 𝛴 bit-count(x[i])
    //
    let odd_count = (limit - 1) / 2 + 1;
    let composite_count: usize = x.iter().map(|&a| a.count_ones() as usize).sum();
    let capacity = 1 + (odd_count - composite_count);
    let mut r = Vec::with_capacity(capacity);
    r.push(2);
    r.extend(
        (3..=limit)
        .step_by(2)
        .filter(|&n| {
            let k = n / 2;
            (x[k / 64] & (1u64 << (k % 64))) == 0
        })
    );
    r
}

/// Segmented Sieve of Eratosthenes
/// Uses a mod 2 wheel to identify composites.
///
pub fn eratosthenes (limit: usize) -> Vec<usize> {
    match limit {
        0 | 1 => return Vec::new(),
        2 => return vec![2],
        3 => return vec![2, 3],
        _ => {}
    }

    // The simple sieve is used for values up to ⌊√limit⌋.
    let base_size = usize::isqrt(limit);
    let base = simple(base_size);

    let mut r = Vec::with_capacity(pi(limit));
    r.extend_from_slice(&base);

    // Values 𝑎 and 𝑏 identify the boundaries of the segment
    // in terms of the subset of integers it currently represents.
    //
    //    𝑎 identifies the next number to process.
    //    It is incremented until it hits the limit.
    //
    //    𝑏 is the largest number represented by the segment.
    //
    // Bits translate to numbers as shown below (the key difference
    // from the simple sieve case is that numbers must be shifted into
    // the segment by subtracting 𝑎):
    //
    //    number = { 2·𝑘 + 1 | 𝑘 ∈ ℤ ∧ 𝑘 ≥ 0 }
    //    k = (number - a) / 2 (integral division)
    //    k / 64 → word index = index
    //    k % 64 → bit offset = offset
    //
    // These formulas are used to access the
    // individual bit of a number as shown below:
    //
    //    is bit clear?  ⇒  segment[index] & (1 << offset) == 0
    //    set bit        ⇒  segment[index] |= 1 << offset
    //
    let mut a = (base_size + 1) | 1;

    // segment is a scratchpad that stores a sequence of values to sieve.
    // It is a sliding window over the integers which partititions values
    // remaining to be processed into subsets which can fit in L1 cache.
    //
    // 0 = prime, 1 = composite
    //
    let limit_odd = if limit % 2 == 0 { limit - 1 } else { limit };
    let target_bit_size = usize::max(1024, cache_size::l1_cache_size().unwrap_or(32 * 1024) / 2) * 8;
    let required_bit_size = usize::max(limit_odd.saturating_sub(a) / 2 + 1, 1);
    let segment_bit_size = usize::min(target_bit_size, required_bit_size);
    let segment_word_size = (segment_bit_size + 63) / 64;
    let mut segment = vec![0u64; segment_word_size];

    let segment_capacity = segment_word_size * 64; // counts odd numbers

    while a <= limit_odd {
        let b = usize::min(a.saturating_add(2 * (segment_capacity - 1)), limit_odd);
        let odd_count = (b - a) / 2 + 1;
        let delta = (odd_count + 63) / 64;

        // Reset the segment scratchpad.
        // Use a slice instead of accessing the segment directly
        // to keep the logic the same for the final iteration.
        let x = &mut segment[..delta];
        x.fill(0);

        for &p in &base[1..] { // skip 2
            // 𝑚 is the first odd multiple of 𝑝 ≥ 𝑎
            // starting at 𝑝² or higher.
            let m = if a <= p * p { p * p } else {
                //
                // When 𝑎 is not a multiple of 𝑝, it is in the
                // interval shown below (rem = 𝑎 mod 𝑝):
                //
                //    [preceding multiple, next multiple]  ⇒
                //    [𝑎 - rem, 𝑎 - rem + 𝑝]               ⇒
                //    [𝑎 - rem, 𝑎 + (𝑝 - rem)]
                //
                // Example. Let 𝑎 = 23 and 𝑝 = 7. This means
                //   we are starting at 23 and crossing out
                //   multiples of 7 (composites).
                //
                //    rem = 23 mod 7 = 2
                //    [23 - 2, 23 + (7 - 2)] = [21, 28]
                //
                // The value 28 is the next multiple of 𝑝 = 7.
                // Because 28 is even, 𝑝 is added to start at 35,
                // the first odd multiple of 7 greater than 23.
                //
                let rem = a % p;
                let mut m = if rem == 0 { a } else { a + (p - rem) };
                if m % 2 == 0 {
                    m += p; // ensure m is odd
                }
                m
            };

            if m > b { continue; }

            // Mark each multiple of 𝑝 as a composite.
            let mut k = (m - a) / 2;
            while k < odd_count {
                x[k / 64] |= 1u64 << (k % 64);
                k += p;
            }
        }

        // Extract primes from the current segment.
        for (i, &word) in x.iter().enumerate() {
            let mut prime_bits = !word;
            let base_k = i * 64;

            while prime_bits != 0 {
                let lsb = prime_bits.trailing_zeros() as usize;
                let k = base_k + lsb;
                if k >= odd_count { break; }
                r.push(a + 2 * k);
                prime_bits &= prime_bits - 1; // clsb
            }
        }

        if b == limit_odd { break; }

        // Skip to the next window.
        a = b + 2;
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

    ///
    /// Use the following command to run all tests in this module:
    ///
    ///   cargo test --lib sequences::primes::tests -- --nocapture
    ///

    const PRIMES: [usize; 58] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31,
        37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
        83, 89, 97, 101, 103, 107, 109, 113, 127, 131,
        137, 139, 149, 151, 157, 163, 167, 173, 179,
        181, 191, 193, 197, 199, 211, 223, 227, 229,
        233, 239, 241, 251, 257, 263, 269, 271,
    ];

    #[test]
    fn test_simple () {
        let actual: Vec<_> = simple(500).into_iter().take(PRIMES.len()).collect();
        assert_eq!(actual, PRIMES);
    }

    #[test]
    fn test_segmented () {
        let actual: Vec<_> = eratosthenes(500).into_iter().take(PRIMES.len()).collect();
        assert_eq!(actual, PRIMES);
    }

    #[test]
    fn test_iterator () {
        let actual: Vec<_> = Primes::new(500).take(PRIMES.len()).collect();
        assert_eq!(actual, PRIMES);
    }
}
