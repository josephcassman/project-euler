// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=1
//!
//! Multiples of 3 or 5
//!
//! If we list all the natural numbers below $10$ that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!

fn main () {
    println!("\niterative method: {}", iterative());
    println!("\nformula method: {}\n", formula());
}

/// Straightforward iterative method, 𝛰(n)
fn iterative () -> u32 {
    let mut sum: u32 = 0;
    for n in 1..1_000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

/// We can use the triangle numbers as a base.
/// The following formula gets the sum from 1 to 'n:
///
///    T(n) = n * (n + 1) / 2
///
/// Multiplying T(n) by three or five shifts the
/// sequence to the sum of each multiple sequence.
/// However, in doing so the total will be greater
/// than 1_000, so 'n must be reduced accordingly.
///
/// Since we want to find all multiples of 'k less than 'n,
/// we have the inequality m * k < n. Since these values
/// are integers, solving for 'm, we have the following:
///
///    m * k ≤ n - 1
///    m ≤ (n - 1) / k
///    m = floor((n - 1) / k)
///
/// Then call 'U the triangle function shifted by 'k:
///
///    U(n, k) = k * m * (m + 1) / 2
///
/// Finally, common multiples must be removed since
/// they have already been included:
///
///    sum = U(1000, 3) + U(1000, 5) - U(1000, 15)
///
fn formula () -> u32 {
    fn u (k: u32) -> u32 {
        let m = (1_000 - 1) / k;
        k * m * (m + 1) / 2
    }

    u(3) + u(5) - u(15)
}
