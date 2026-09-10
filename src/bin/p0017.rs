// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=17
//!
//! Number Letter Counts
//!
//! If the numbers 1 to 5 are written out in words: one, two, three, four, five,
//! then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//!
//! If all the numbers from 1 to 1000 (one thousand) inclusive were
//! written out in words, how many letters would be used?
//!
//! NOTE: Do not count spaces or hyphens. For example,
//!       342 (three hundred and forty-two) contains 23 letters and
//!       115 (one hundred and fifteen) contains 20 letters.
//!       The use of "and" when writing out numbers is in compliance
//!       with British usage.
//!

use std::collections::HashMap;

fn main () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    // number → letter count
    let count = HashMap::from([
        (0, 0),

        (1, 3), // one
        (2, 3), // two
        (3, 5), // three
        (4, 4), // four
        (5, 4), // five
        (6, 3), // six
        (7, 5), // seven
        (8, 5), // eight
        (9, 4), // nine

        (10, 3), // ten
        (11, 6), // eleven
        (12, 6), // twelve
        (13, 8), // thirteen
        (14, 8), // fourteen
        (15, 7), // fifteen
        (16, 7), // sixteen
        (17, 9), // seventeen
        (18, 8), // eighteen
        (19, 8), // nineteen

        (20, 6), // twenty
        (30, 6), // thirty
        (40, 5), // forty
        (50, 5), // fifty
        (60, 5), // sixty
        (70, 7), // seventy
        (80, 6), // eighty
        (90, 6), // ninety
    ]);

    let mut sum: u64 = 0;

    for x in 1..=19 { sum += count[&x]; }
    for x in 20..=99 {
        sum += count[&(x - (x % 10))]; // two's digit
        if x % 10 != 0 {
            sum += count[&(x % 10)]; // one's digit
        }
    }

    sum *= 10; // repeat 1..=99 for 100 through to 900

    // three's digit
    sum += (1..=9).map(|a| count[&a] * 100).sum::<u64>();

    // "hundred" is used on x >= 100
    sum += 7 * 900;

    // "and" is used on x % 100 != 0 for x >= 100
    // 1000 - 100 ⇒ 900 - 9 ⇒ 891
    sum += (900 - 9) * 3;

    sum + 11 // one thousand
}
