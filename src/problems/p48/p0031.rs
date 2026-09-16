// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=31
//!
//! Coin Sums
//!
//! In the United Kingdom the currency is made up of pound (£) and pence (p).
//! There are eight coins in general circulation:
//!
//!    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
//!
//! It is possible to make £2 in the following way:
//!
//!    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//!
//! How many different ways can £2 be made using any number of coins?
//!

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let mut r: u64 = 0;

    for pound in 0..=1 {
    for fifty in 0..=4 {
    for twenty in 0..=10 {
    for ten in 0..=20 {
    for five in 0..=40 {
    for two in 0..=100 {
    for one in 0..=200 {
        let sum = pound * 100
                + fifty * 50
                + twenty * 20
                + ten * 10
                + five * 5
                + two * 2
                + one;

        if sum == 200 { r += 1 }
    }}}}}}}

    r + 2 // two 1 pound notes + one 2 pound note
}
