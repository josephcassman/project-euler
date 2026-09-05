// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

/// Calculate the Greatest Common Divisor (GCD)
/// for two numbers using Stein's binary algorithm.
///
/// Identities used:
///  1. gcd(a, 0) = a
///  2. gcd(2 * a, 2 * b) = 2 * gcd(a, b)
///  3. gcd(a, 2 * b) = gcd(a, b) if a is odd
///  4. gcd(a, b) = gcd(a, b - a) if a <= b
///
/// Since gcd is commutative, the above identities still apply
/// if the operands are swapped.
///
pub const fn gcd (mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    // Pulling out powers of two
    // significantly improves performance.
    //
    // trailing_zeros() gives the largest power of two
    // that cleanly divides a value:
    //
    //   12 = 0000 1100
    //   12.trailing_zeros() == 2
    //   12 >> 2 == 3 == 12 / pow(2, 2)

    let k = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();

    // The following subtractive Euclidean algorithm
    // pseudocode helps explain the loop below:
    //
    //    while b != 0
    //       if a > b { swap(a, b) }
    //       b = b - a
    //
    // Stein's algorithm includes a key optimization:
    // because a and b are odd, b - a is always even.
    // This fact allows factors of 2 to be stripped
    // with an efficient right-shift instruction.

    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b { core::mem::swap(&mut a, &mut b); }
        b -= a;
    }

    // It is necessary to restore the power of two
    // removed above to get the correct answer.
    //
    // Example:
    // gcd(12, 8)
    //   12 == 2 * 2 * 3 (i = 2)
    //    8 == 2 * 2 * 2 (j = 3)
    //    k == min(2, 3) = 2
    //
    // Evaluating gcd(3, 1) is much faster
    // but requires multiplying by 4 (left shift by 2)
    // to restore the correct value:
    //   gcd(3, 1)  == 1
    //   gcd(12, 8) == 4 == 1 << 2 == 1 << k

    a << k
}

/// Calculate the Least Common Multiple (LCM)
/// for two numbers using Stein's binary algorithm.
///
/// The following identity is used:
///
///    lcm(a, b) = abs(a * b) / gcd(a, b)
///
/// NOTE: Does not protect against overflow
///
pub const fn lcm (a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    (a / gcd(a, b)) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_gcd () {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_binary_lcm () {
        assert_eq!(lcm(48, 18), 144);
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(4, 6), 12);
    }
}
