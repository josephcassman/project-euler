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
    println!("\niterative method: {:?}", iterative());
    println!("\ncalculation method: {:?}\n", calculation());
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

///
/// A Lehmer code is a bijection between the integer range [0, 𝑛! − 1]
/// and the symmetric group 𝑆ₙ (the group of all permutations of a set
/// containing 𝑛 distinct elements).
///
///    Given a permutation 𝜎 = (𝜎₁, 𝜎₂, ⋯, 𝜎ₙ) of the set {1, 2, ⋯, 𝑛},
///    the Lehmer code 𝐿(𝜎) = (𝑐₁, 𝑐₂, ⋯, 𝑐ₙ) defines each entry 𝑐ᵢ as
///    the number of elements to the right of 𝜎ᵢ that are smaller than 𝜎ᵢ:
///
///          𝑐ᵢ = # { 𝑗 > 𝑖 | 𝜎ⱼ < 𝜎ᵢ }
///
/// Given this definition, the rank of a permutation is defined as
/// its index in the factorial number space:
///
///    Rank(𝜎) = ∑ 𝑐ᵢ·(𝑛 − 𝑖)!  ∀𝑖 ∊ [1, 𝑛]
///
/// The Lehmer code bijection has the effect of translating
/// the base 10 representation of a number
///
///    𝑁 = 𝑑ₖ·10ᵏ + ⋯ + 𝑑₁·10¹ + 𝑑₀·10⁰  (0 ≤ 𝑑ᵢ < 10)
///
/// to a number in the factorial number system
///
///    𝐿(𝜎) = 𝑐₁·(𝑛−1)! + 𝑐₂·(𝑛−2)! + ⋯ + 𝑐ₙ₋₁·1! + 𝑐ₙ·0!  (0 ≤ 𝑐ᵢ ≤ 𝑛 − 𝑖)
///
/// The above information helps us see the two step process
/// to translate Rank(𝜎) to its corresponding array of values
/// (the permutation of rank Rank(𝜎)):
///
///    1. Translate Rank(𝜎) to 𝐿(𝜎) using long division.
///    2. Use the values of 𝑐ᵢ in 𝐿(𝜎) to select
///       values in the array.
///
fn calculation () -> Vec<u64> {
    use project_euler::discrete_math::factorial;
    use num_traits::ToPrimitive;

    let mut a = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut r = Vec::with_capacity(10);
    let mut rem = 1_000_000 - 1;

    for i in 1..=10 {
        let w = factorial(10 - i).to_usize().unwrap();
        let c = rem / w;
        rem %= w;

        r.push(a.remove(c));
    }

    r
}
