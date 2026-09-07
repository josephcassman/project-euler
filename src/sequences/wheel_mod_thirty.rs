// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! Sequence of residues modulo thirty
//!
//! Since the number thirty has the prime factors of two, three,
//! and five, using this sequence of numbers in the search for primes
//! reduces effort compared to the default case of considering all
//! numbers greater than or equal to two. Since only numbers coprime
//! to thirty need to be considered, the search for composites is reduced
//! by 73.3% (a 100%/26.7% = 3.75x speedup).
//!
//!    k ≡ 0  (mod 30) ⇒ multiples of 30 (divisible by 2, 3 and 5)
//!    k ≡ 1  (mod 30) ⇒ coprime
//!    k ≡ 2  (mod 30) ⇒ divisible by 2
//!    k ≡ 3  (mod 30) ⇒ divisible by 3
//!    k ≡ 4  (mod 30) ⇒ divisible by 2
//!    k ≡ 5  (mod 30) ⇒ divisible by 5
//!    k ≡ 6  (mod 30) ⇒ divisible by 2 and 3
//!    k ≡ 7  (mod 30) ⇒ coprime
//!    k ≡ 8  (mod 30) ⇒ divisible by 2
//!    k ≡ 9  (mod 30) ⇒ divisible by 3
//!    k ≡ 10 (mod 30) ⇒ divisible by 2 and 5
//!    k ≡ 11 (mod 30) ⇒ coprime
//!    k ≡ 12 (mod 30) ⇒ divisible by 2 and 3
//!    k ≡ 13 (mod 30) ⇒ coprime
//!    k ≡ 14 (mod 30) ⇒ divisible by 2 and 7
//!    k ≡ 15 (mod 30) ⇒ divisible by 3 and 5
//!    k ≡ 16 (mod 30) ⇒ divisible by 2
//!    k ≡ 17 (mod 30) ⇒ coprime
//!    k ≡ 18 (mod 30) ⇒ divisible by 2 and 3
//!    k ≡ 19 (mod 30) ⇒ coprime
//!    k ≡ 20 (mod 30) ⇒ divisible by 2 and 5
//!    k ≡ 21 (mod 30) ⇒ divisible by 3 and 7
//!    k ≡ 22 (mod 30) ⇒ divisible by 2 and 11
//!    k ≡ 23 (mod 30) ⇒ coprime
//!    k ≡ 24 (mod 30) ⇒ divisible by 2 and 3
//!    k ≡ 25 (mod 30) ⇒ divisible by 5
//!    k ≡ 26 (mod 30) ⇒ divisible by 2 and 13
//!    k ≡ 27 (mod 30) ⇒ divisible by 3
//!    k ≡ 28 (mod 30) ⇒ divisible by 2 and 7
//!    k ≡ 29 (mod 30) ⇒ coprime
//!
pub struct WheelModThirty {
    cur: Option<u64>,
    step_index: usize,
}

/// The sequence starts at seven (one is trivial).
///    7 → 11      = add 4
///    11 → 13     = add 2
///    13 → 17     = add 4
///    17 → 19     = add 2
///    19 → 23     = add 4
///    23 → 29     = add 6
///    29 → 31 = 1 = add 2
///    1 → 7       = add 6
///
const STEPS: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];

impl WheelModThirty { pub fn new () -> Self {
    Self {
        cur: Some(7),
        step_index: 0,
    }}
}
impl Default for WheelModThirty { fn default () -> Self { Self::new() } }
impl Iterator for WheelModThirty {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.cur?;

        self.cur = val.checked_add(STEPS[self.step_index]);

        // Increment to the next index and
        // modulo 8 to wrap around to the beginning.
        self.step_index = (self.step_index + 1) & 7;

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Use the following command to run all tests in this module:
    ///
    ///   cargo test --lib sequences::wheel_mod_thirty::tests -- --nocapture
    ///

    #[test]
    fn test () {
        let expected = [7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];
        let actual: Vec<u64> = WheelModThirty::new().take(15).collect();

        assert_eq!(actual, expected);
    }
}
