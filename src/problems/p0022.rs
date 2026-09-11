// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=22
//!
//! Names Scores
//!
//! Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
//! containing over five-thousand first names, begin by sorting it into alphabetical order.
//! Then working out the alphabetical value for each name, multiply this value by
//! its alphabetical position in the list to obtain a name score.
//!
//! For example, when the list is sorted into alphabetical order, COLIN, which
//! is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
//! So, COLIN would obtain a score of 938 ⨯ 53 = 49714.
//!
//! What is the total of all the name scores in the file?
//!

use std::error::Error;
use std::fs::File;

pub fn run () -> Result<(), Box<dyn Error>> {
    println!("\niterative method: {:?}\n", iterative());
    Ok(())
}

/// import names from open file
/// sort into alphabetical order
/// for each name
///     get ASCII character sum for name
///     multiple this sum by its 1-based index
/// sum all these scores
fn iterative () -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;

    let mut names = import_names()?;
    names.sort();
    for (i, name) in names.iter().enumerate() {
        let ascii_sum = name.bytes().map(|x| (x - b'A' + 1) as u64).sum::<u64>();
        sum += ascii_sum * (i as u64 + 1);
    }

    Ok(sum)
}

fn import_names () -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("data/0022_names.txt")?;
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let r = csv.records()
    .filter_map(|a| a.ok()) // ignore bad lines
    .flat_map(|a| { a.iter().map(|b| b.to_string()).collect::<Vec<String>>() })
    .collect();
    Ok(r)
}
