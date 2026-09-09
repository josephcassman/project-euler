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


#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Use the following command to run all tests in this module:
    ///
    ///   cargo test --lib tests:: -- --nocapture
    ///

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
}
