// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=33
//!
//! Digit Cancelling Fractions
//!
//! The fraction 49/98 is a curious fraction, as an inexperienced
//! mathematician in attempting to simplify it may incorrectly believe that
//! 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
//!
//! We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
//!
//! There are exactly four non-trivial examples of this type of fraction,
//! less than one in value, and containing two digits in the numerator and denominator.
//!
//! If the product of these four fractions is given in its lowest common terms,
//! find the value of the denominator.
//!

use crate::etc::split_digits;
use crate::etc::discrete_math::gcd;

pub fn run () {
    println!("\niterative method: {:?}\n", iterative());
}

fn iterative () -> u64 {
    let mut rn = 1;
    let mut dn = 1;

    // Setting the denominator to one more than the
    // numerator ensures the value is less than one.
    for numerator in 11..=98 {
    for denominator in (numerator + 1)..=99 {
        let n = split_digits(numerator);
        let d = split_digits(denominator);

        if zero(&n) || zero(&d) { continue; }

        let (numerator_2, denominator_2) =
            if n[1] == d[0] { (n[0] as u64, d[1] as u64) }
            else if n[0] == d[1] { (n[1] as u64, d[0] as u64) }
            else { continue; };

        // Testing for equality by reducing both fractions by their GCD
        // is the same as expanding both fractions by multiplying by the
        // common multiple of the denominators:
        //
        //    a   c     a         c
        //    ─ = ─  →  ─ ⨯ b·d = ─ ⨯ b·d  →  a·d = c·b
        //    b   d     b         d
        //
        if numerator * denominator_2 == denominator * numerator_2 {
            rn *= numerator;
            dn *= denominator;
        }
    }}

    dn / gcd(rn, dn)
}

#[inline(always)]
fn zero (digits: &[u8]) -> bool {
    digits.iter().any(|&x| x == 0)
}
