// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=25
//!
//! 1000-digit Fibonacci Number
//!
//! The Fibonacci sequence is defined by the recurrence relation:
//!
//!    F(n) = F(n - 1) + F(n - 2), where F(1) = 1 and F(2) = 1.
//!
//! Hence the first 12 terms will be:
//!
//!    F(1) = 1
//!    F(2) = 1
//!    F(3) = 2
//!    F(4) = 3
//!    F(5) = 5
//!    F(6) = 8
//!    F(7) = 13
//!    F(8) = 21
//!    F(9) = 34
//!    F(10) = 55
//!    F(11) = 89
//!    F(12) = 144
//!
//! The 12th term, F(12), is the first term to contain three digits.
//!
//! What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
//!

pub fn run () {
    println!("\ncalculation method: {:?}\n", calculation());
}

///
/// Binet's Formula
///
///    𝐹ₙ = (𝜑ⁿ − 𝜓ⁿ) ∕ √5
///
///    𝜑 = golden ratio = 1/2·(1 + √5)
///    𝜓 = conjugate = 1/2·(1 − √5)
///
/// Since 𝐹ₙ is asymptotic to 𝜑ⁿ ∕ √5 the number of digits
/// of 𝐹ₙ is asymptotic to 𝑛·log₁₀ 𝜑.
///
/// Let 𝘋 equal the number of digits of 𝐹ₙ.
///
///    𝘋 = ⌊𝑛·log₁₀ 𝜑 − log₁₀ √5⌋ + 1
///      = ⌊𝑛·log₁₀ 𝜑 − log₁₀ √5⌋ + 1
///
///    ⌊𝑛·log₁₀ 𝜑 − log₁₀ √5⌋ = 𝘋 − 1
///    𝑛·log₁₀ 𝜑 − log₁₀ √5 ≥ 𝘋 − 1
///    𝑛·log₁₀ 𝜑 ≥ 𝘋 − 1 + log₁₀ √5
///    𝑛 ≥ (𝘋 − 1 + log₁₀ √5) ∕ log₁₀ 𝜑
///    𝑛 = ⌈(𝘋 − 1 + log₁₀ √5) ∕ log₁₀ 𝜑⌉
///
fn calculation () -> u64 {
    let a = 999.0f64 + f64::log10(f64::sqrt(5.0f64));
    let b = f64::log10(core::f64::consts::GOLDEN_RATIO);

    f64::ceil(a / b) as u64
}
