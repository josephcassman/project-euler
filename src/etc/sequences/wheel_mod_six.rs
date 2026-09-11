// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! Sequence of residues modulo six
//!
//! Since the number six has the prime factors of two and three,
//! using this sequence of numbers in the search for primes reduces
//! effort compared to the default case of considering all numbers
//! greater than or equal to two. Since only numbers coprime to six
//! need to be considered, the search for composites is reduced by
//! 66.7% (a 100%/33.33% = 3x speedup).
//!
//!    k ≡ 0 (mod 6) ⇒ divisible by 2 and 3
//!    k ≡ 1 (mod 6) ⇒ coprime
//!    k ≡ 2 (mod 6) ⇒ divisible by 2
//!    k ≡ 3 (mod 6) ⇒ divisible by 3
//!    k ≡ 4 (mod 6) ⇒ divisible by 2
//!    k ≡ 5 (mod 6) ⇒ coprime
//!
pub struct WheelModSix {
    cur: Option<u64>,
    step: u64,
}

impl WheelModSix { pub fn new () -> Self {
    Self {
        cur: Some(5),
        step: 2,
    }}
}
impl Default for WheelModSix { fn default () -> Self { Self::new() } }
impl Iterator for WheelModSix {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.cur?;

        // Alternate adding two or four.
        self.cur = val.checked_add(self.step);
        self.step = 6 - self.step;

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        let expected = [5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49];
        let actual: Vec<u64> = WheelModSix::new().take(16).collect();

        assert_eq!(actual, expected);
    }
}
