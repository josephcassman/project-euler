// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! Sequence of Pentagonal numbers
//!
//!    𝑃(1) = 1
//!    𝑃(2) = 5
//!    𝑃(𝑛) = 𝑃(𝑛 − 1) + 3·𝑛 − 2          ∀𝑛 ≥ 2
//!         = 2·𝑃(𝑛 − 1) - 𝑃(𝑛 − 2) + 3   ∀𝑛 ≥ 3
//!
//!    0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247,
//!    287, 330, 376, 425, 477, 532, 590, 651, 715, 782, 852, 925,
//!    ...
//!

/// nᵗʰ Pentagonal number using the closed-form formula
///
///    𝑃(𝑛) = 𝑛·(3·𝑛 - 1) / 2
///
pub fn pentagonal (n: u64) -> u64 {
    let a = n as u128;
    ((a * (3 * a - 1)) >> 1) as u64
}

/// Sequence of Pentagonal numbers starting with 𝑃(1).
///
pub struct Pentagonal { cur: Option<u64>, delta: u64, limit: u64 }
impl Pentagonal { pub fn new (limit: u64) -> Self {
    Self {
        cur: Some(1), // 𝑃(1) = 1
        delta: 4,     // 𝑃(2) = 𝑃(1) + 3·2 − 2 = 𝑃(1) + 4
        limit,
    }
} }
impl Default for Pentagonal { fn default () -> Self { Self::new(u64::MAX) } }
impl Iterator for Pentagonal {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.cur?;

        if val > self.limit {
            self.cur = None;
            return None;
        }

        self.cur = val.checked_add(self.delta);
        self.delta = self.delta.checked_add(3).unwrap_or(u64::MAX);

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: [u64; 25] = [
        1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287,
        330, 376, 425, 477, 532, 590, 651, 715, 782, 852, 925,
    ];


    #[test]
    fn test_formula () {
        let actual: Vec<u64> = (1..=25).map(|n| pentagonal(n)).collect();

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_iterator () {
        let a: Vec<u64> = Pentagonal::default().take(25).collect();
        let b: Vec<u64> = Pentagonal::new(925).take(25).collect();

        assert_eq!(a, EXPECTED);
        assert_eq!(b, EXPECTED);
    }
}
