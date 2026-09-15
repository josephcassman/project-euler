// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=39
//!
//! Integer Right Triangles
//!
//! If p is the perimeter of a right angle triangle with integral length sides, {a, b, c},
//! there are exactly three solutions for p = 120.
//!
//!    {20,48,52}, {24,45,51}, {30,40,50}
//!
//! For which value of p ≤ 1000, is the number of solutions maximised?
//!

use crate::etc::discrete_math::gcd;

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

///
/// Euclid's algorithm can be used directly.
/// Iterate through 𝑚 and 𝑛, removing the coprime restriction
/// so that non-primitive triples are included.
///
///    1. 𝑚 > 𝑛 > 0
///    3. one value is odd and the other is even
///
/// Then the triple {𝑎, 𝑏, 𝑐} is calculated as follows:
///
///    𝑎 = 𝑚² - 𝑛²
///    𝑏 = 2·𝑚·𝑛
///    𝑐 = 𝑚² + 𝑛²
///
///    𝑝 = 𝑎 + 𝑏 + 𝑐
///      = 𝑚·𝑚 - 𝑛·𝑛 + 2·𝑚·𝑛 + 𝑚·𝑚 + 𝑛·𝑛
///      = 2·𝑚·(𝑚 + 𝑛)
///
fn iterative () -> u64 {
    // perimeter -> count
    let mut perimeter_counts = [0u32; 1001];

    for m in 2.. {
        let perimeter_m = 2 * m * (m + 1);
        if perimeter_m > 1000 { break; }

        for n in 1..m {
            let opposite_parity = (m + n) % 2 != 0;
            let coprime = gcd(m, n) == 1;

            if opposite_parity && coprime {
                let perimeter = 2 * m * (m + n);

                // scale the primitive triple by 𝑘
                let mut k = 1;
                while k * perimeter <= 1000 {
                    let p = (k * perimeter) as usize;
                    perimeter_counts[p] += 1;
                    k += 1;
                }
            }
        }
    }

    perimeter_counts.iter()
    .enumerate()
    .max_by_key(|&(_, &count)| count)
    .map(|(i, _)| i as u64)
    .unwrap()
}
