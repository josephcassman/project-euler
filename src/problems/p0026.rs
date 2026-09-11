// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=26
//!
//! Reciprocal Cycles
//!
//! A unit fraction contains 1 in the numerator. The decimal representation of
//! the unit fractions with denominators 2 to 10 are given:
//!
//!    1/2 = 0.5
//!    1/3 = 0.(3)
//!    1/4 = 0.25
//!    1/5 = 0.2
//!    1/6 = 0.1(6)
//!    1/7 = 0.(142857)
//!    1/8 = 0.125
//!    1/9 = 0.(1)
//!    1/10 = 0.1
//!
//! Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle.
//! It can be seen that 1/7 has a 6-digit recurring cycle.
//!
//! Find the value of d < 1000 for which 1/d contains the longest recurring cycle
//! in its decimal fraction part.
//!

use num_bigint::BigUint;
use num_traits::Pow;

use crate::etc::factorization::factorize;
use crate::etc::sequences::primes::eratosthenes;

pub fn run () {
    println!("\ncalculation method: {:?}\n", calculation());
}

///
/// Computing 1/𝑑 using long division involves successively multiplying
/// remainders by 10, the base of the decimal number system. Since the
/// fraction has a 1 as the numerator, the fractional cycle repeats when
/// a remainder of 1 is obtained. The length of the cycle is thus
/// equivalent to how many times a remainder is multiplied by 10.
///
/// Consider the example of 1/7. We can see it takes six remainders, so
/// multiplying by 10 six times to reach a 1. Thus the recurring cycle
/// of 7 is six.
///
///     0 . 1 4 2 8 5 7
///    ────────────────────
/// 7 │ 1.0 0 0 0 0 0 0
///     − 0
///     ───
///       1 0  ← ⨯ 10
///       − 7
///       ───
///         3 0  ← ⨯ 10
///       − 2 8
///         ───
///           2 0  ← ⨯ 10
///         − 1 4
///           ───
///             6 0  ← ⨯ 10
///           − 5 6
///             ───
///               4 0  ← ⨯ 10
///             − 3 5
///               ───
///                 5 0  ← ⨯ 10
///               − 4 9
///                 ───
///                   1  ← repeats
///
/// The above gives the intuition for the fact that long division
/// on 1/𝑑 repeatedly calculates remainders 𝑟ₖ ≡ 10ᵏ (mod 𝑑).
/// Therefore, the decimal cycle length is defined as the smallest
/// positive integer 𝑘 such that:
///
///    10ᵏ ≡ 1 (mod 𝑑)   where gcd(10, 𝑑) = 1
///
/// This relation brings to mind Fermat's Little theorem which
/// states that the following holds for prime 𝑝 where gcd(𝑎, 𝑝) = 1:
///
///    𝑎ᵖ⁻¹ ≡ 1 (mod 𝑝)
///
/// To combine the two formulas above, we can divide 𝑝 - 1 by 𝑘,
/// which involves finding a quotient 𝑚 and remainder 𝑟 such that:
///
///    𝑝 − 1 = 𝑚·𝑘 + 𝑟     (0 ≤ 𝑟 < 𝑘)
///
///    10ᵖ⁻¹ = 10ᵐ˙ᵏ⁺ʳ = (10ᵏ)ᵐ·10ʳ              # 1: apply modular arithmetic
///          ≡ (1)ᵐ·10ʳ (mod 𝑝)                  # 2: substitute
///          ≡ 10ʳ (mod 𝑝)                       # 3: reduce
///    10ᵖ⁻¹ ≡ 10ʳ ≡ 1 (mod 𝑝)                   # 4: by Fermat's theorem
///
/// Because 𝑘 is the smallest positive power that equals 1 (mod 𝑝),
/// and 𝑟 < 𝑘, 𝑟 cannot be 1, 2, ⋯, 𝑘 − 1. The only possibility is 𝑟 = 0.
/// Since 𝑟 must be 0, we now have 𝑝 − 1 = 𝑚·𝑘. Thus, Fermat's Little theorem
/// has helped us deduce that any valid cycle length 𝑘 must divide 𝑝 − 1.
///
///
/// With the above information in mind, to solve the original problem,
/// it suffices to search through prime numbers 𝑝 looking for the
/// largest cycle 𝑘.
///
///   1. Set 𝑝 to the largest prime strictly less than 1000.
///   2. Factor 𝑝 − 1 into its distinct prime factors {𝑞₁, 𝑞₂, ⋯ 𝑞ₘ}.
///   3. For each 𝑞ᵢ compute 𝑄ᵢ = 10⁽ᵖ⁻¹⁾ᐟ𐞥ⁱ (mod 𝑝). Let 𝑄 equal
///      the set of all such 𝑄ᵢ.
///   4. Define A as ∃𝑄ᵢ = 1 for any 𝑄ᵢ in 𝑄.
///   5. If A then 𝑘 < 𝑝 − 1 which means that this prime has a shorter
///      reptend cycle. Move to the next prime below 𝑝 and go to step 2. §
///   6. If ¬A then 𝑘 = 𝑝 − 1 and we have found the answer 𝑝.
///
/// § NOTE: Since 𝑘 divides 𝑝 − 1, the maximum cycle length for a non-primitive
///   prime is at most (𝑝 − 1) ∕ 2. This means that any 𝑝 close to 1000 which is
///   discarded may compete for the longest cycle with numbers below 500.
///
fn calculation () -> u64 {
    let primes = eratosthenes(1_000);
    let ten = BigUint::from(10u32);

    'outer: for &p in primes.iter().rev() {
        for factor in factorize(p - 1, &primes) {
            let e = (p - 1) / factor.p;
            let q = ten.clone().pow(e as u32) % p;
            if q == BigUint::ONE { continue 'outer; }
        }

        return p;
    }

    0
}
