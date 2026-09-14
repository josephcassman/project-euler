// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

pub mod discrete_math;
pub mod factorization;
pub mod sequences;

pub fn is_palindrome (a: &str) -> bool {
    let mut chars = a.chars();
    while let (Some(x), Some(y)) = (chars.next(), chars.next_back()) {
        if x != y {
            return false;
        }
    }
    true
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
pub fn permute (a: &mut [u8]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome () {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("abba"));
        assert!(is_palindrome("noon"));
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("madam"));

        assert!(!is_palindrome("ab"));
        assert!(!is_palindrome("abc"));
    }

    #[test]
    fn test_permute () {
        let mut a: [u8; 0] = [];
        assert!(!permute(&mut a));
        assert_eq!(a, []);

        let mut b = [1];
        assert!(!permute(&mut b));
        assert_eq!(b, [1]);

        let mut c = [1, 2];
        assert!(permute(&mut c));
        assert_eq!(c, [2, 1]);

        assert!(!permute(&mut c));
        assert_eq!(c, [1, 2]);

        let mut d = [1, 1, 1, 1, 1];
        assert!(!permute(&mut d));
        assert_eq!(d, [1, 1, 1, 1, 1]);

        let mut e = [1, 2, 3];
        let expected = [
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];

        for &x in expected.iter() {
            assert!(permute(&mut e));
            assert_eq!(e, x);
        }

        assert!(!permute(&mut e));
        assert_eq!(e, [1, 2, 3]);




    }
}
