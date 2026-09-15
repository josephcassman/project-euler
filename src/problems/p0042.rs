// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=42
//!
//! Coded Triangle Numbers
//!
//! The nᵗʰ term of the sequence of triangle numbers is given by, t(n) = 1/2·n·(n+1);
//! so the first ten triangle numbers are:
//!
//!    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//!
//! By converting each letter in a word to a number corresponding to its
//! alphabetical position and adding these values we form a word value.
//! For example, the word value for SKY is 19 + 11 + 25 = 55 = t(10).
//! If the word value is a triangle number then we shall call the word a triangle word.
//!
//! Using words.txt (right click and 'Save Link/Target As...'), a 16K text file
//! containing nearly two-thousand common English words, how many are triangle words?
//!

use std::error::Error;
use std::fs::File;

pub fn run () {
    pub fn f () -> Result<(), Box<dyn Error>> {
        println!("\niterative method: {:?}\n", iterative());
        Ok(())
    }

    f().expect("Problem 42 failed with an error");
}

fn iterative () -> Result<u64, Box<dyn Error>> {
    const TRIANGLE: [u64; 32] = [
        0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78,
        91, 105, 120, 136, 153, 171, 190, 210, 231, 253,
        276, 300, 325, 351, 378, 406, 435, 465, 496,
    ];

    let mut r = 0;

    let words = import_words()?;
    for word in words.iter() {
        let a = word.bytes().map(|x| (x - b'A' + 1) as u64).sum::<u64>();
        match TRIANGLE.binary_search(&a) {
            Ok(_) => r += 1,
            Err(_) => {}
        }
    }

    Ok(r)
}

fn import_words () -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("data/0042_words.txt")?;
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let r = csv.records()
    .filter_map(|a| a.ok()) // ignore bad lines
    .flat_map(|a| { a.iter().map(|b| b.to_string()).collect::<Vec<String>>() })
    .collect();
    Ok(r)
}
