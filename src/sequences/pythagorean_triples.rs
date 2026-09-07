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
pub struct PrimitivePythagoreanTriples {
    m: u64,
    n: u64,
    limit: u64,
    max_m: u64,
}

impl PrimitivePythagoreanTriples {
    // Keep both a and b less than or equal to limit.
    pub fn new (limit: u64) -> Self {
        // Cut off the sequence when a or b is above limit.
        //
        // To perform this check efficiently, we can observe
        // that a is small if n is large and conversely
        // b is small if n is small. This means max(a, b)
        // is found when a and b are roughly equal:
        //
        //    m² - n² ≈ 2·m·n                          # approximation
        //
        //    m² - n² = 2·m·n                          # 1: Consider the approximation as an equation
        //    n² + 2·m·n - m² = 0                      # 2: Rearrange terms
        //    n²/m² + 2·m·n/m² - m²/m² = 0             # 3: Multiply by 1/m²
        //    n²/m² + 2·n/m - 1 = 0                    # 4: Simplify
        //    x² + 2·x - 1 = 0                         # 5: Let x = n/m
        //    x = -1 ± √2                              # 6: Solve using the quadratic formula
        //    x = -1 + √2                              # 7: Discard negative root since m > 0 and n ≥ 1
        //    n/m = √2 - 1                             # 8: Substitute back in for x
        //    n = (√2 - 1)·m                           # 9: Solve for n
        //
        //    n ≈ (√2 - 1)·m ≈ 0.41421·m               # 10: Approximate n
        //
        //    b = 2·m·n                                # 11: Definition of b
        //      ≈ 2·(√2 - 1)·m² ≡ A ≈ 0.82843·m²       # 12: Use the approximation from 10
        //
        // So once A is greater than or equal to the limit value,
        // no value for n can keep a and b under the limit.
        // That is, a value of m that satisfies the following
        // inequalities means b is greater than the limit.
        //
        //    A > LIMIT
        //    2·(√2 - 1)·m² > LIMIT
        //    m > √(LIMIT/(2·(√2 - 1))) ≈ 1.09868·√LIMIT
        //
        let max_m = ((limit as f64) / (2.0 * (core::f64::consts::SQRT_2 - 1.0))).sqrt().floor() as u64;

        Self {
            m: 2,
            n: 1,
            limit,
            max_m,
        }
    }
}

impl Default for PrimitivePythagoreanTriples { fn default () -> Self { Self::new(1_000) } }

impl Iterator for PrimitivePythagoreanTriples {
    type Item = Triple;

    fn next (&mut self) -> Option<Self::Item> {
        use crate::discrete_math::gcd;

        let limit = self.limit;
        loop {
            if self.m > self.max_m { return None; }

            let m = self.m;
            let n = self.n;

            // Filter out b above the limit.
            // Performing this check here saves effort and
            // is possible since any subsequent value of n
            // will also produce a value above the limit.
            let b = 2 * m * n;
            if b > limit {
                self.m += 1;
                self.n = 1;
                continue;
            }

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

            // Now filter out values of a greater the limit.
            let a = m * m - n * n;
            if a > limit { continue; }

            return Some((a, b, m * m + n * n));
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
    limit: u64,
}

impl PythagoreanTriples {
    pub fn new (limit: u64) -> Self {
        let mut inner = PrimitivePythagoreanTriples::new(limit);
        let cur = inner.next();

        Self {
            inner,
            cur,
            k: 1,
            limit,
        }
    }
}

impl Default for PythagoreanTriples { fn default () -> Self { Self::new(1_000) } }

impl Iterator for PythagoreanTriples {
    type Item = Triple;

    fn next (&mut self) -> Option<Self::Item> {
        let Some((a, b, c)) = self.cur else { return None; };

        let k = self.k;
        if a * k > self.limit || b * k > self.limit {
            // Set to 2 instead of 1 since the k == 1
            // case is returned on this pass.
            self.k = 2;
            self.cur = self.inner.next();
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
            // (15, 36, 39),  // k == 3, not included because b > 25

            (15, 8, 17),   // k == 1
            // (30, 16, 34),   // k == 2, not included because a > 25

            (7, 24, 25),   // k == 1
            // (14, 48, 50),  // k == 2, not included because b > 25

            (21, 20, 29)   // k == 1
            // (42, 40, 58)   // k == 2, not included because a and b > 25

            // (9, 40, 41),   // k == 1, not included because b > 25
            // (18, 80, 82),  // k == 1, not included because b > 25

            // ... not more values satisfy a and b <= 25
        ];
        let actual: Vec<Triple> = PythagoreanTriples::new(25).take(11).collect();

        assert_eq!(actual, expected);
    }
}
