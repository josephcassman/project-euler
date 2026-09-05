// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

/// 6k ± 1 for k > 1
pub struct SixModOne {
    current: Option<u64>,
    step: u64,
}

impl SixModOne {
    pub fn new () -> Self {
        Self {
            current: Some(5),
            step: 2,
        }
    }
}

impl Default for SixModOne {
    fn default () -> Self {
        Self::new()
    }
}

impl Iterator for SixModOne {
    type Item = u64;

    fn next (&mut self) -> Option<Self::Item> {
        let val = self.current?;

        // Alternate adding two or four.
        self.current = val.checked_add(self.step);
        self.step = 6 - self.step;

        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_six_mod_one () {
        let expected = [5, 7, 11, 13, 17, 19, 23, 25, 29, 31];
        let actual: Vec<u64> = SixModOne::new().take(10).collect();

        assert_eq!(actual, expected);
    }
}
