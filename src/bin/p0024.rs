// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=24
//!
//! Lexicographic Permutations
//!
//! A permutation is an ordered arrangement of objects.
//! For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
//! If all of the permutations are listed numerically or alphabetically, we call
//! it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
//!
//!    012   021   102   120   201   210
//!
//! What is the millionth lexicographic permutation of the digits
//! 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
//!

fn main () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> [u8; 10] {
    let mut a: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for _ in 1..1_000_000 {
        permute(&mut a);
    }

    a
}

///
/// Transform the provided array into its next
/// lexicographic permutation.
///
/// 1. find the pivot index i from the end where
///    the sequence stops increasing, thus marking
///    the start of the descending suffix
/// 2. find the next larger element to the right
///    that is greater than a[i]
/// 3. swap these two elements
/// 4. reverse the order of all values from i + 1
///    to the end to minimize it
///
fn permute (a: &mut [u8]) -> bool {
    // Find the first element from the right
    // that breaks the descending order.
    if let Some(i) = a.windows(2).rposition(|x| x[0] < x[1]) {

        // Searching from the right to find the element
        // ensures we select the rightmost valid element.
        let j = a.iter().rposition(|y| y > &a[i]).unwrap();
        a.swap(i, j);
        a[i + 1..].reverse();
        true
    }
    else {
        // The array is already in its final descending permutation
        // so return the array to its initial permutation and
        // stop the sequence.
        a.reverse();
        false
    }
}
