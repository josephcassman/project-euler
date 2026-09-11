// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=15
//!
//! Lattice Paths
//!
//! Starting in the top left corner of a 2 × 2 grid, and only being able
//! to move to the right and down, there are exactly 6 routes to the
//! bottom right corner.
//!
//! ╔═══╤═══╗      ╔═══╗───┐      ╔═══╗───┐
//! │   │   ║      │   ║   │      │   ║   │
//! ├───┼───╢      ├───╚═══╗      ├───║───┤
//! │   │   ║      │   │   ║      │   ║   │
//! └───┴───       └───┴───       └───╚═══
//!  (R-R-D-D)      (R-D-R-D)      (R-D-D-R)
//!
//! ╔───┬───┐      ╔───┬───┐      ╔───┬───┐
//! ║   │   │      ║   │   │      ║   │   │
//! ╚═══╤═══╗      ╚═══╗───┤      ╟───┼───┤
//! │   │   ║      │   ║   │      ║   │   │
//! └───┴───       └───╚═══       ╚═══╧═══
//!  (D-R-R-D)      (D-R-D-R)      (D-D-R-R)
//!
//! How many such routes are there through a 20 × 20 grid?
//!

use num_bigint::BigUint;
use crate::etc::discrete_math::factorial;

pub fn run () {
    println!("\nbrute-force method: {}", brute_force());
    println!("\niterative method: {}\n", iterative());
}

///
/// The number of paths is the binomial coefficient 𝐶(𝑛, 𝑘).
///
///    𝑑 = number of possible down steps
///    𝑟 = number of possible right steps
///    𝑛 = 𝑑 + 𝑟 = total number of steps
///
/// Since a path is chosen by choosing all 𝑑 or 𝑟,
/// the solution is
///
///    𝐶(𝑛, 𝑑) = 𝐶(𝑛, 𝑟)
///
/// The closed form formula is:
///
///    𝐶(𝑛, 𝑘) = 𝑛! ∕ 𝑘!·(𝑛 - 𝑘)!
///
fn brute_force () -> BigUint {
    let twenty = factorial(20);
    factorial(40) / (twenty.clone() * twenty)
}

///
/// The iterative approach to calculating the binomial coefficient
/// can be used to reduce the overall integral size needed.
///
///    𝐶(𝑛, 𝑘) = 𝑛 × (𝑛-1) × ⋯ × (𝑛-𝑘+1) ∕ 𝑘 × (𝑘-1) × ⋯ × 1
///            = 𝛱 𝑖 ∊ [1, 𝑘] (𝑛 + 1 - 𝑖) ∕ 𝑖
///
fn iterative () -> u64 {
    let mut r: u64 = 1;
    for i in 1..=20 {
        r = (r * (40 - 20 + i)) / i;
    }
    r
}
