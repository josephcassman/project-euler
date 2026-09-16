// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=38
//!
//! Pandigital Multiples
//!
//! Take the number 192 and multiply it by each of 1, 2, and 3:
//!
//!    192 ⨯ 1 = 192
//!    192 ⨯ 2 = 384
//!    192 ⨯ 3 = 576
//!
//! By concatenating each product we get the 1 to 9 pandigital, 192384576.
//! We will call 192384576 the concatenated product of 192 and (1,2,3).
//!
//! The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
//! giving the pandigital, 918273645, which is the concatenated product of
//! 9 and (1,2,3,4,5).
//!
//! What is the largest 1 to 9 pandigital 9-digit number that can be formed as
//! the concatenated product of an integer with (1,2, ..., n) where n > 1?
//!

use crate::etc::pandigital::is_pandigital_str;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

///
/// To beat the value in the definition, 918273645, 𝑚 must start
/// with 9 and the concatenated product must total exactly 9 digits.
///
///    𝑚 is 2 digits  ⇒ 8 digits for 𝑛 = 3 and 11 for 𝑛 = 4
///    𝑚 is 3 digits  ⇒ 7 digits for 𝑛 = 2 and 11 for 𝑛 = 3
///    𝑚 is 5+ digits ⇒ 10+ digits for any 𝑛 ≥ 2
///    𝑚 is 4 digits  ⇒ 𝑚 ⨯ 1 (4 digits) + 𝑚 ⨯ 2 (5 digits)
///
/// So 𝑚 must be 4 digits and start with a 9, and 𝑛 = (1, 2).
///
fn iterative () -> String {
    for m in (9124..=9876).rev() {
        let a = format!("{}{}", m, 2 * m);
        if is_pandigital_str(&a) {
            return a;
        }
    }
    String::new()
}
