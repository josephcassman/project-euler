// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! Sequence of Fibonacci numbers
//!
//!    F(0) = 0
//!    F(1) = 1
//!    F(n) = F(n - 1) + F(n - 2)
//!
//!    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89,
//!    144, 233, 377, 610, 987, 1597, 2584, 4181,
//!    ...
//!
pub struct Fibonacci {
    cur: Option<u64>,
    next: Option<u64>,
}

impl Fibonacci { pub fn new () -> Self {
    Self {
        cur: Some(0),
        next: Some(1),
    }}
}
impl Default for Fibonacci { fn default () -> Self { Self::new() } }
impl Iterator for Fibonacci {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.cur?;

        let next = match (self.cur, self.next) {
            (Some(a), Some(b)) => a.checked_add(b),
            _ => None,
        };

        self.cur = self.next;
        self.next = next;

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        let expected = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89,
            144, 233, 377, 610, 987, 1597, 2584, 4181,
        ];
        let actual: Vec<u64> = Fibonacci::new().take(20).collect();

        assert_eq!(actual, expected);
    }
}
