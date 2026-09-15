// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

///
/// Little-endian BCD type.
///
///    Digit   Bits
///     0       0..=3
///     1       4..=7
///     2       8..=11
///     3       12..=15
///     4       16..=19
///     5       20..=23
///     6       24..=27
///     7       28..=31
///     8       32..=35
///     9       36..=39
///     10      40..=43
///     11      44..=47
///     12      48..=51
///     13      52..=55
///     14      56..=59
///     15      60..=63
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bcd(u64);

impl Bcd {
    pub const MAX_DIGITS: usize = 16;

    pub fn from_u64 (mut a: u64) -> Option<Self> {
        if a == 0 { return Some(Self(0)); }

        let mut x = 0u64;
        let mut shift = 0;

        while a > 0 {
            if shift >= 64 { return None; }
            let d = a % 10;
            x |= d << shift;
            shift += 4;
            a /= 10;
        }

        Some(Self(x))
    }

    pub const fn len (&self) -> usize {
        if self.0 == 0 { 1 }
        else {
            // (63 − msb) / 4 = (15 − ms_nibble)
            16 - (self.0.leading_zeros() as usize / 4)
        }
    }

    pub const fn is_empty (&self) -> bool { false }

    pub const fn digit (&self, i: usize) -> Option<u64> {
        if i >= self.len() { None }
        else { Some((self.0 >> (i * 4)) & 0x0F) }
    }

    pub const fn bits (&self) -> u64 { self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd () {
        let a = Bcd::from_u64(12345).unwrap();

        assert_eq!(a.bits(), 0x12345);
        assert_eq!(a.len(), 5);

        assert_eq!(a.digit(0), Some(5));
        assert_eq!(a.digit(1), Some(4));
        assert_eq!(a.digit(2), Some(3));
        assert_eq!(a.digit(3), Some(2));
        assert_eq!(a.digit(4), Some(1));
        assert_eq!(a.digit(5), None);

        let b = Bcd::from_u64(0).unwrap();

        assert_eq!(b.bits(), 0);
        assert_eq!(b.len(), 1);

        assert_eq!(b.digit(0), Some(0));
        assert_eq!(b.digit(1), None);
    }
}
