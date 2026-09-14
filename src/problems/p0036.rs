// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=36
//!
//! Double-base Palindromes
//!
//! The decimal number, 585 = 1001001001_2 (binary), is palindromic in both bases.
//!
//! Find the sum of all numbers, less than one million, which are palindromic
//! in base 10 and base 2.
//!
//! (Please note that the palindromic number, in either base, may not include leading zeros.)
//!

use crate::etc::split_digits;

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let mut r = 0;

    for a in 1u64..1_000_000 {
        if is_palindrome(a) && a == reverse_bits(a) {
            r += a;
        }
    }

    r
}

///
/// Example showing the idea using a byte.
///
/// 0001 0110   # original
///             # leading zero count = 3
/// 0110 1000   # reversed bits
/// 0000 1101   # shr 3
///
#[inline(always)]
fn reverse_bits (a: u64) -> u64 {
    if a == 0 { return 0; }
    let b = a.leading_zeros();
    a.reverse_bits() >> b
}

fn is_palindrome (a: u64) -> bool {
    let x = split_digits(a);
    let mut y = x.clone();
    y.reverse();
    x == y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome () {
        assert!(is_palindrome(1));
        assert!(is_palindrome(2));
        assert!(is_palindrome(3));
        assert!(is_palindrome(4));
        assert!(is_palindrome(5));
        assert!(is_palindrome(6));
        assert!(is_palindrome(7));
        assert!(is_palindrome(8));
        assert!(is_palindrome(9));

        assert!(is_palindrome(12321));
        assert!(is_palindrome(123321));

        assert!(!is_palindrome(12));
        assert!(!is_palindrome(123123));
    }
}
