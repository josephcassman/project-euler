// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=28
//!
//! Number Spiral Diagonals
//!
//! Starting with the number 1 and moving to the right in a clockwise direction
//! a 5 by 5 spiral is formed as follows:
//!
//!    !21!  22   23   24  !25!
//!     20  ! 7!   8  ! 9!  10
//!     19    6  ! 1!   2   11
//!     18  ! 5!   4  ! 3!  12
//!    !17!  16   15   14  !13!
//!
//! It can be verified that the sum of the numbers on the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral
//! formed in the same way?
//!

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

///
/// let 𝑅(𝑛) be ring level 𝑛 and
/// 𝛿 the distance between corner values.
///
///    𝑅(0) = 1               ⇒ 𝛿 = 0
///    𝑅(1) = 3, 5, 7, 9      ⇒ 𝛿 = 2
///    𝑅(2) = 13, 17, 21, 25  ⇒ 𝛿 = 4
///    𝑅(3) = 31, 37, 43, 49  ⇒ 𝛿 = 6
///    𝑅(4) = ...             ⇒ 𝛿 = 8
///
/// So the delta 𝛿 between the endpoints of ring 𝑛 is 2·𝑛.
/// And therefore the distance from the first value in the ring
/// to the first endpoint of the ring is 𝛿 − 1.
///
/// A 1001 x 1001 spiral will occur at 𝑅(500).
///
fn iterative () -> u64 {
    let mut sum = 1u64 + 3 + 5 + 7 + 9 + 13 + 17 + 21 + 25;
    let mut corner = 25u64;
    let mut delta = 6u64;

    for _ in 3..=500 {
        sum += corner +     delta; // corner 0
        sum += corner + 2 * delta; // corner 1
        sum += corner + 3 * delta; // corner 2
        sum += corner + 4 * delta; // corner 3

        corner = corner + 4 * delta;
        delta += 2;
    }

    sum
}
