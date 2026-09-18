// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=78
//!
//! Coin Partitions
//!
//! Let p(n) represent the number of different ways in which n coins
//! can be separated into piles. For example, five coins can be separated into
//! piles in exactly seven different ways, so p(5) = 7.
//!
//!    OOOOO
//!    OOOO O
//!    OOO OO
//!    OOO O O
//!    OO OO O
//!    OO O O O
//!    O O O O O
//!
//! Find the least value of n for which p(n) is divisible by one million.
//!

pub fn run () {
    println!("\niterative method: {}\n", iterative());
}

fn iterative () -> u64 {
    for n in 6.. {
        if p(n) % 1_000_000 == 0 {
            return n;
        }
    }
    unreachable!()
}

/// Size of the Integer Partition of 𝑛
fn p (n: u64) -> u64 {
    let a = n as usize;
    let mut r = vec![0u64; a + 1];

    // There is one way to partition 0, the empty set.
    r[0] = 1;

    // Iterate through each integer to be partitioned.
    for i in 1..=a {

        // Update the partition counts for 𝑖 to 𝑛.
        // The number of ways to partition 𝑗 is
        // the count of 𝑖 plus the count of the remainder.
        //
        // Example: 𝑛 = 4
        //
        //     Initialize 𝑟 = [1 0 0 0 0]
        //
        //     i  j   r           intuition
        //     1  1  [1 1 0 0 0]   Ways to partition 1 to 4 using 1s,
        //     1  2  [1 1 1 0 0]   so add r += [0, 1, 1, 1, 1].
        //     1  3  [1 1 1 1 0]
        //     1  4  [1 1 1 1 1]
        //
        //     2  2  [1 1 2 1 1]   Ways to partition 2 to 4 using 2s,
        //     2  3  [1 1 2 2 1]   so r += [0, 0, 1, 1, 2].
        //     2  4  [1 1 2 2 3]
        //
        //     3  3  [1 1 2 3 3]   Ways to partition 3 to 4 using 3s,
        //     3  4  [1 1 2 3 4]   so r += [0, 0, 0, 1, 1].
        //
        //     4  4  [1 1 2 3 5]   Ways to partition 4 using 4s,
        //                         so r += [0, 0, 0, 0, 1] and the
        //                         result is r[4] = 5.
        //
        for j in i..=a {
            r[j] += r[j - i];
        }
    }

    r[a]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_partition () {
        assert_eq!(p(0), 1);
        assert_eq!(p(1), 1);
        assert_eq!(p(2), 2);
        assert_eq!(p(3), 3);
        assert_eq!(p(4), 5);
        assert_eq!(p(5), 7);
        assert_eq!(p(40), 37_338);
    }
}
