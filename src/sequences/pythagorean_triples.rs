// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

pub type Triple = (u64, u64, u64);

/// Use Euclid's algorithm to generate primitive Pythagorean triples
/// using two integers m and n where the following conditions hold:
///
///    1. m > n > 0
///    2. gcd(m, n) = 1
///    3. one value is odd and the other is even
///
/// Then the triple (a, b, c) is calculated as follows:
///
///    a = m² - n²
///    b = 2·m·n
///    c = m² + n²
///
pub struct PrimitivePythagoreanTriples { m: u64, n: u64 }
impl PrimitivePythagoreanTriples { pub fn new () -> Self { Self { m: 2, n: 1 } } }
impl Default for PrimitivePythagoreanTriples { fn default () -> Self { Self::new() } }

impl Iterator for PrimitivePythagoreanTriples {
    type Item = Triple;

    fn next (&mut self) -> Option<Self::Item> {
        use crate::discrete_math::gcd;

        loop {
            let m = self.m;
            let n = self.n;

            // Advance m and n while maintaining constraint 1.
            self.n += 1;
            if self.n >= self.m {
                self.m += 1;
                self.n = 1;
            }

            // Constraints 2 and 3 ensure the triple is primitive.
            if ((m ^ n) & 1 == 0) || gcd(m, n) != 1 {
                // Parity check: (m ^ n) & 1 == 0
                // The code equates to m ≡ n (mod 2).

                continue;
            }

            return Some((
                m * m - n * n,  // a
                2 * m * n,      // b
                m * m + n * n,  // c
            ));
        }
    }
}

/// All triples can be generated from the primitive triples
/// by multiplying by k = 2, ...
///
pub struct PythagoreanTriples {
    inner: PrimitivePythagoreanTriples,
    cur: Option<Triple>,
    k: u64,
    a_limit: u64,
}

impl PythagoreanTriples {
    /// req: a_limit > 3
    pub fn new (a_limit: u64) -> Self {
        let mut inner = PrimitivePythagoreanTriples::new();
        let cur = inner.next();

        Self {
            inner,
            cur,
            k: 1,
            a_limit,
        }
    }
}

impl Default for PythagoreanTriples { fn default () -> Self { Self::new(1_000) } }

impl Iterator for PythagoreanTriples {
    type Item = Triple;

    fn next (&mut self) -> Option<Self::Item> {
        let Some((a, b, c)) = self.cur else { return None; };

        let k = self.k;
        if a * k > self.a_limit {
            // Set to 2 instead of 1 since the k == 1
            // case is returned on this pass.
            self.k = 2;

            while let Some(x) = self.inner.next() {
                let (next_a, ..) = x;
                if next_a < self.a_limit {
                    self.cur = Some(x);
                    break;
                }
            }

            self.cur
        }
        else {
            self.k += 1;
            Some((k * a, k * b, k * c))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_pythagorean_triples () {
        let expected = [
            (3, 4, 5),
            (5, 12, 13),
            (15, 8, 17),
            (7, 24, 25),
            (21, 20, 29),
            (9, 40, 41),
            (35, 12, 37),
            (11, 60, 61),
            (45, 28, 53),
            (33, 56, 65),
        ];
        let actual: Vec<Triple> = PrimitivePythagoreanTriples::default().take(10).collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_non_primitive_pythagorean_triples () {
        let expected = [
            (3, 4, 5),     // k == 1
            (6, 8, 10),    // k == 2
            (9, 12, 15),   // k == 3
            (12, 16, 20),  // k == 4
            (15, 20, 25),  // k == 5
            (18, 24, 30),  // k == 6

            (5, 12, 13),   // k == 1
            (10, 24, 26),  // k == 2
            (15, 36, 39),  // k == 3
            (20, 48, 52),  // k == 4

            (15, 8, 17),   // k == 1

            (7, 24, 25),   // k == 1
            (14, 48, 50),  // k == 1

            // (21, 20, 29) excluded since a == 21 > 20

            (9, 40, 41),   // k == 1
            (18, 80, 82),  // k == 2
        ];
        let actual: Vec<Triple> = PythagoreanTriples::new(20).take(15).collect();

        assert_eq!(actual, expected);
    }
}
