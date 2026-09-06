// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=9
//!
//! Special Pythagorean Triplet
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//!
//!    a² + b² = c².
//!
//! For example, 3² + 4² = 9 + 16 = 25 = 5².
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

/// Nine to the power of thirteen is less than u64::MAX
/// so the maximum product can fit.
fn iterative () -> u64 {
    use project_euler::sequences::pythagorean_triples::PythagoreanTriples;

    for (a, b, c) in PythagoreanTriples::default() {
        let sum = a + b + c;

        if sum == 1000 {
            return a * b * c;
        }
    }

    0
}
