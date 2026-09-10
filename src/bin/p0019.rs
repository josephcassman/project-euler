// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=19
//!
//! Counting Sundays
//!
//! You are given the following information, but
//! you may prefer to do some research for yourself.
//!
//!    • 1 Jan 1900 was a Monday.
//!    • Thirty days has September,
//!      April, June and November.
//!      All the rest have thirty-one,
//!      Saving February alone,
//!      Which has twenty-eight, rain or shine.
//!      And on leap years, twenty-nine.
//!    • A leap year occurs on any year evenly divisible by 4,
//!      but not on a century unless it is divisible by 400.
//!
//! How many Sundays fell on the first of the month during
//! the twentieth century (1 Jan 1901 to 31 Dec 2000)?
//!

fn main () {
    println!("\niterative method: {}\n", iterative());
}

///
/// The months of the year have the following lengths:
///
///    Month      Length  Day 1  LY Length  LY Day 1
///    January     31      0      31         0
///    February    28      31     29         31
///    March       31      59     31         60
///    April       30      90     30         91
///    May         31      120    31         121
///    June        30      151    30         152
///    July        31      181    31         182
///    August      31      212    31         213
///    September   30      243    30         244
///    October     31      273    31         274
///    November    30      304    30         305
///    December    31      334    31         335
///
/// Days of the week index:
///
///    Sunday     0
///    Monday     1
///    Tuesday    2
///    Wednesday  3
///    Thursday   4
///    Friday     5
///    Saturday   6
///
/// The first day in 1901 was a Tuesday.
///
fn iterative () -> u64 {
    #[inline(always)]
    fn is_leap_year (year: u64) -> bool {
        year % 4 == 0 &&
        (year % 100 != 0 || year % 400 == 0)
    }

    const NORMAL_YEAR: [u64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const LEAP_YEAR: [u64; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

    let mut sum = 0;
    let mut day = 0;
    let mut weekday = 2;
    let mut year = 1901;

    loop {
        if weekday == 0 {
            let is_first = if is_leap_year(year) { LEAP_YEAR.contains(&day) } else { NORMAL_YEAR.contains(&day) };
            if is_first { sum += 1; }
        }

        weekday += 1;
        weekday = (weekday + 1) % 7; // wrap weekday

        day += 1;

        let total_days = if is_leap_year(year) { 366 } else { 365 };
        if day > total_days {
            day = 0;
            year += 1;
            if year > 2000 { break; }
            continue;
        }
    }

    sum
}
