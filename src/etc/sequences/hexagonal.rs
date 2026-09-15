// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! Sequence of Hexagonal numbers
//!
//!    𝐻(1) = 1
//!    𝐻(2) = 5
//!    𝐻(𝑛) = 𝐻(𝑛 − 1) + 4·𝑛 − 3          ∀𝑛 ≥ 1
//!         = 2·𝐻(𝑛 − 1) − 𝐻(𝑛 − 2) + 4   ∀𝑛 ≥ 2
//!
//!    0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276,
//!    325, 378, 435, 496, 561, 630, 703, 780, 861, 946,
//!    ...
//!

/// nᵗʰ Hexagonal number using the closed-form formula
///
///    𝐻(𝑛) = 𝑛·(2·𝑛 − 1)
///
pub fn hexagonal (n: u64) -> u64 {
    let a = n as u128;
    (a * (2 * a - 1)) as u64
}

/// Sequence of Hexagonal numbers starting with 𝐻(1).
///
pub struct Hexagonal { cur: Option<u64>, delta: u64, limit: u64 }
impl Hexagonal { pub fn new (limit: u64) -> Self {
    Self {
        cur: Some(1), // 𝐻(1) = 1
        delta: 5,     // 𝐻(2) = 𝐻(1) + 4·2 − 3 = 𝐻(1) + 5
        limit,
    }
} }
impl Default for Hexagonal { fn default () -> Self { Self::new(u64::MAX) } }
impl Iterator for Hexagonal {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.cur?;

        if val > self.limit {
            self.cur = None;
            return None;
        }

        self.cur = val.checked_add(self.delta);
        self.delta = self.delta.checked_add(4).unwrap_or(u64::MAX);

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: [u64; 22] = [
        1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276,
        325, 378, 435, 496, 561, 630, 703, 780, 861, 946,
    ];


    #[test]
    fn test_formula () {
        let actual: Vec<u64> = (1..=22).map(|n| hexagonal(n)).collect();

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_iterator () {
        let a: Vec<u64> = Hexagonal::default().take(22).collect();
        let b: Vec<u64> = Hexagonal::new(946).take(22).collect();

        assert_eq!(a, EXPECTED);
        assert_eq!(b, EXPECTED);
    }
}
