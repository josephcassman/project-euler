// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=4
//!
//! Largest Palindrome Product
//!
//! A palindromic number reads the same both ways.
//! The largest palindrome made from the product of
//! two 2-digit numbers is 9009 = 91 * 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> usize {
    use project_euler::is_palindrome;

    let mut r = 0;

    for a in (100..=999).rev() {
        // The inner loop is in the range [100, 'a].
        // 'a * 'a <= 'r means that 'a * 'b <= 'r since 'b <= 'a.
        // As a result, the inner loop can be skipped.
        if a * a <= r { break; }

        // By limiting the inner loop to the range [100, 'a]
        // we can avoid evaluating 'b * 'a which duplicates 'a * 'b.
        for b in (100..=a).rev() {
            let c = a * b;
            if c <= r { break; }
            if is_palindrome(&c.to_string()) {
                r = c;
            }
        }
    }

    r
}
