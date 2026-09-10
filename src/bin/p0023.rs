// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=23
//!
//! Non-Abundant Sums
//!
//! A perfect number is a number for which the sum of its proper divisors
//! is exactly equal to the number. For example, the sum of the proper divisors
//! of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
//!
//! A number n is called deficient if the sum of its proper divisors
//! is less than n and it is called abundant if this sum exceeds n.
//!
//! As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number
//! that can be written as the sum of two abundant numbers is 24. By mathematical analysis,
//! it can be shown that all integers greater than 28123 can be written as the sum of
//! two abundant numbers. However, this upper limit cannot be reduced any further by analysis
//! even though it is known that the greatest number that cannot be expressed as
//! the sum of two abundant numbers is less than this limit.
//!
//! Find the sum of all the positive integers which cannot be written as
//! the sum of two abundant numbers.
//!

fn main () {
    println!("\niterative method: {:?}\n", iterative());
}

/// for a in [4, 28123]
///     is a abundant?
fn iterative () -> usize {
    use project_euler::factorization::sigma_sieve;

    let sigma = sigma_sieve(28_124);
    let is_abundant = |a: u64| { (sigma[a as usize] - a) > a };
    let abundant: Vec<_> = (4..=28_123).filter(|&a| is_abundant(a as u64)).collect();
    let mut abundant_sums: [bool; 28_124] = [false; 28_124];

    for a in abundant.iter() {
    for b in abundant.iter() {
        let sum = a + b;
        if sum > 28_123 { continue; }
        abundant_sums[sum] = true;
    }}

    abundant_sums.iter()
    .enumerate()
    .skip(1)
    .filter(|&(_, &a)| !a)
    .map(|(i, _)| i)
    .sum()
}
