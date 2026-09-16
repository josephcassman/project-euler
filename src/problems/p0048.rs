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

/// Keep only the last ten digits by reducing
/// the arithmetic modulo 10¹⁰.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Mod10(pub u64);

impl Mod10 {
    pub const MOD: u64 = 1_00000_00000;

    pub fn new (a: u64) -> Self { Self(a % Self::MOD) }

    pub fn add (&mut self, a: Self) {
        self.0 = (self.0 + a.0) % Self::MOD;
    }

    pub fn mul (&mut self, a: Self) {
        let b = (self.0 as u128) * (a.0 as u128);
        self.0 = (b % (Self::MOD as u128)) as u64;
    }

    pub fn pow (self, a: u64) -> Self {
        let mut r = Self::new(1);
        for _ in 0..a {
            r.mul(self);
        }
        r
    }

    pub fn add_pow (&mut self, a: u64) {
        let b = Self::new(a).pow(a);
        self.add(b);
    }
}
