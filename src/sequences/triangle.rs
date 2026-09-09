// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! Sequence of Triangle numbers
//!
//!    T(0) = 0
//!    T(1) = 0 + 1
//!    T(n) = 0 + 1 + ... + (n - 1) + n
//!
//!    0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66,
//!    78, 91, 105, 120, 136, 153, 171, 190, 210,
//!    ...
//!

/// nᵗʰ Triangle number using the closed-form formula
pub fn triangle (n: u64) -> u64 {
    // Widen to 128 bits to handle the overflow
    // from multiplying two 64-bit values.
    let a = n as u128;
    ((a * (a + 1)) >> 1) as u64
}

pub struct Triangle { i: u64, cur: Option<u64>, limit: u64 }
impl Triangle { pub fn new (limit: u64) -> Self { Self { i: 1, cur: Some(0), limit } } }
impl Default for Triangle { fn default () -> Self { Self::new(u64::MAX) } }
impl Iterator for Triangle {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.cur?;

        if val > self.limit {
            self.cur = None;
            return None;
        }

        self.cur = val.checked_add(self.i);
        self.i = self.i.checked_add(1).unwrap_or(u64::MAX);

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Use the following command to run all tests in this module:
    ///
    ///   cargo test --lib sequences::triangle::tests -- --nocapture
    ///

    const TRIANGLE: [u64; 21] = [
        0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66,
        78, 91, 105, 120, 136, 153, 171, 190, 210,
    ];


    #[test]
    fn test_formula () {
        let actual: Vec<u64> = (0..21).map(|n| triangle(n)).collect();

        assert_eq!(actual, TRIANGLE);
    }

    #[test]
    fn test_iterator () {
        let a: Vec<u64> = Triangle::default().take(21).collect();
        let b: Vec<u64> = Triangle::new(210).take(21).collect();

        assert_eq!(a, TRIANGLE);
        assert_eq!(b, TRIANGLE);
    }
}
