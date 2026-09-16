// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=48
//!
//! Self Powers
//!
//! The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
//!
//! Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
//!

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> String {
    let mut r = Mod10(1);

    for a in 2..=1000 {
        r.add_pow(a);
    }

    r.0.to_string().chars()
    .rev()
    .take(10)
    .collect::<Vec<char>>().into_iter()
    .rev()
    .collect()
}

use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Keep only the last ten digits by reducing
/// the arithmetic modulo 10¹⁰.
///
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Mod10(pub u64);

impl Mod10 {
    pub const MOD: u64 = 1_00000_00000;

    pub fn new (a: u64) -> Self { Self(a % Self::MOD) }

    ///
    /// Reduce exponential multiplications by
    /// iterating over powers of 2.
    ///
    /// Example:
    ///
    ///    𝑥¹⁵ = 𝑥⁸ ⨯ 𝑥⁴ ⨯ 𝑥² ⨯ 𝑥¹
    ///
    pub fn pow (self, mut a: u64) -> Self {
        let mut x = self;
        let mut r = Self::new(1);
        while a > 0 {
            if a % 2 == 1 { r *= x; }
            x *= x;
            a /= 2;
        }
        r
    }

    pub fn add_pow (&mut self, a: u64) {
        *self += Self::new(a).pow(a);
    }
}

impl Add for Mod10 {
    type Output = Self;
    fn add (self, a: Self) -> Self {
        Self((self.0 + a.0) % Self::MOD)
    }
}

impl AddAssign for Mod10 {
    fn add_assign (&mut self, a: Self) {
        *self = *self + a;
    }
}

impl Mul for Mod10 {
    type Output = Self;
    fn mul (self, a: Self) -> Self {
        let b = (self.0 as u128) * (a.0 as u128);
        Self((b % (Self::MOD as u128)) as u64)
    }
}

impl MulAssign for Mod10 {
    fn mul_assign (&mut self, a: Self) {
        *self = *self * a;
    }
}

impl fmt::Display for Mod10 {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:010}", self.0)
    }
}
