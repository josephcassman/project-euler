// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=67
//!
//! Maximum Path Sum II
//!
//! By starting at the top of the triangle below and moving to adjacent numbers
//! on the row below, the maximum total from top to bottom is 23.
//!
//! ```text
//!         !3!
//!       !7!  4
//!      2  !4!  6
//!    8   5  !9!  3
//! ```
//!
//! That is, 3 + 7 + 4 + 9 = 23.
//!
//! Find the maximum total from top to bottom in triangle.txt
//! (right click and 'Save Link/Target As...'), a 15K text file containing
//! a triangle with one-hundred rows.
//!
//! NOTE: This is a much more difficult version of Problem 18.
//!       It is not possible to try every route to solve this problem, as there are
//!       2^99 altogether! If you could check one trillion (10^12) routes every second
//!       it would take over twenty billion years to check them all. There is an efficient
//!       algorithm to solve it. ;o)
//!

use std::error::Error;
use std::fs;

pub fn run () {
    fn f () -> Result<(), Box<dyn Error>> {
        println!("\niterative method: {:?}\n", iterative());
        Ok(())
    }

    f().expect("Problem 67 failed with an error");
}

///
/// Traverse from the bottom up replacing each
/// node value with the sum of its children
/// along the way.
///
/// Example from the problem text:
///
///         3
///       7    4
///      2   4   6
///    8   5   9   3
///
///         3
///       7    4
///     10  13   15
///
///
///         3
///       20   19
///
///         23
///
fn iterative () -> Result<u64, Box<dyn Error>> {
    let mut data = import_data()?;

    for row in (0..data.len() - 1).rev() {
    for col in 0..data[row].len() {
        let left = data[row + 1][col];
        let right = data[row + 1][col + 1];

        data[row][col] += std::cmp::max(left, right);
    }}

    Ok(data[0][0])
}

fn import_data () -> Result<[Vec<u64>; 100], Box<dyn Error>> {
    let path = "data/0067_triangle.txt";
    let a: Vec<u64> = fs::read_to_string(path)?
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect(""))
        .collect();

    let mut r: [Vec<u64>; 100] = std::array::from_fn(|_| Vec::new());
    let mut offset = 0;

    for (i, row) in r.iter_mut().enumerate() {
        let len = i + 1;
        *row = a[offset..offset + len].to_vec();
        offset += len;
    }

    Ok(r)
}
