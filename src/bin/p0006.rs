// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=6
//!
//! Sum Square Difference
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//!    1² + 2² + ... + 10² = 385.
//!
//! The square of the sum of the first ten natural numbers is,
//!
//!   (1 + 2 + ... + 10)² = 55² = 3025.
//!
//! Hence the difference between the sum of the squares of
//! the first ten natural numbers and the square of the sum is
//! 3025 - 385 = 2640.
//!
//! Find the difference between the sum of the squares of
//! the first one hundred natural numbers and the square of the sum.
//!

fn main () {
    println!("\niterative method: {}", iterative());
    println!("\nformula method: {}\n", formula());
}

fn iterative () -> u64 {
    let mut sum = 0;
    let mut sum_of_squares = 0;

    for a in 1..=100 {
        sum += a;
        sum_of_squares += a * a;
    }

    sum * sum - sum_of_squares
}

/// Here is one way to derive a closed form formula
/// to get the sum of squares. The key idea is to rearrange
/// terms from the cubic summation which allows us
/// to isolate a closed form for the quadratic series.
///
/// Let ∑[a, b, A] be the summation from a to b for the expression A.
///
///    n³ = ∑[1, n, k³] - ∑[1, n - 1, k³]
///       = ∑[1, n, k³] - ∑[2, n, (k - 1)³]                # 1: Shift the second term by one
///       = ∑[1, n, k³] - ∑[1, n, (k - 1)³]                # 2: Can include index 1 since (1 - 1)³ == 0
///       = ∑[1, n, k³ - (k - 1)³]                         # 3: Combine sigmas
///       = ∑[1, n, k³ - (k³ - 3·k² + 3·k - 1)]            # 4: Expand
///       = ∑[1, n, 3·k² - 3·k + 1]                        # 5: Simplify
///       = 3·∑[1, n, k²] - 3·∑[1, n, k] + ∑[1, n, 1]      # 6: Separate sigmas
///
/// Now we clean up the isolation of the quadratic form.
///
///    3·∑[1, n, k²] = n³ + 3·∑[1, n, k] - ∑[1, n, 1]      # 7: Rearrange terms
///    3·∑[1, n, k²] = n³ + 3·(n·(n+1)/2) - n              # 8: Replace the last two terms with their closed forms
///    3·∑[1, n, k²] = (2·n³ + 3·n·(n+1) - 2·n)/2          # 9: Combine into a single term
///
///    ∑[1, n, k²] = (2·n³ + 3·n·(n+1) - 2·n)/6            # 10: Divide by three
///    ∑[1, n, k²] = n·(n + 1)(2·n + 1)/6                  # 11: Factorize the numerator
///
fn formula () -> u64 {
    const N: u64 = 100;
    let sum_of_squares = (N * (N + 1) * (2 * N + 1)) / 6;
    let sum = N * (N + 1) / 2;

    sum * sum - sum_of_squares
}
