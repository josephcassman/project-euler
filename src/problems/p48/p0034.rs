// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=34
//!
//! Digit Factorials
//!
//! 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//!
//! Find the sum of all numbers which are equal to
//! the sum of the factorial of their digits.
//!
//! Note: As 1! = 1 and 2! = 2 are not sums they are not included.
//!

use crate::etc::split_digits;
use crate::etc::discrete_math::factorial;

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

///
/// Running some numbers shows there's a limit to where
/// it is possible for the factorial digit sum to reach
/// the number itself.
///
///    digit #   max factorial sum   min d-digt number
///     1         1 ⨯ 9! =  362880    1e0 = 1
///     2         2 ⨯ 9! =  725760    1e1 = 10
///     3         3 ⨯ 9! = 1088640    1e2 = 100
///     4         4 ⨯ 9! = 1451520    1e3 = 1000
///     5         5 ⨯ 9! = 1814400    1e4 = 10000
///     6         6 ⨯ 9! = 2177280    1e5 = 100000
///     7         7 ⨯ 9! = 2540160    1e6 = 1000000
///     8         8 ⨯ 9! = 2903040    1e7 = 10000000
///
/// Since the minimum 8-digit number has 8 digits
/// and the maximum factorial digit sum only has
/// 7 digits it is not possible for them to be equal.
///
fn iterative () -> u64 {
    let mut r = 0;

    for n in 3..10_000_000 {
        if n == factorial_digit_sum(n) {
            r += n;
        }
    }

    r
}

#[inline(always)]
fn factorial_digit_sum (n: u64) -> u64 {
    split_digits(n).iter().fold(0u64, |acc, &x| acc + factorial(x as u64))
}
