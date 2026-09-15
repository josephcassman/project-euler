// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

pub fn is_pandigital_str (a: &str) -> bool {
    if a.len() != 9 || a.contains('0') { return false; }
    let mut b = a.as_bytes().to_vec();
    b.sort_unstable();
    b == b"123456789"
}

pub fn is_n_pandigital (a: u64) -> bool {
    let mut b = a.to_string().as_bytes().to_vec();
    b.sort_unstable();
    match b.len() {
        1 => b == b"1",
        2 => b == b"12",
        3 => b == b"123",
        4 => b == b"1234",
        5 => b == b"12345",
        6 => b == b"123456",
        7 => b == b"1234567",
        8 => b == b"12345678",
        9 => b == b"123456789",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pandigital_str () {
        assert!(is_pandigital_str("123456789"));
        assert!(is_pandigital_str("724159683"));
        assert!(is_pandigital_str("987654321"));

        assert!(!is_pandigital_str("1"));
        assert!(!is_pandigital_str("12"));
        assert!(!is_pandigital_str("123"));
        assert!(!is_pandigital_str("1234"));
        assert!(!is_pandigital_str("12345"));
        assert!(!is_pandigital_str("123456"));
        assert!(!is_pandigital_str("1234567"));
        assert!(!is_pandigital_str("12345678"));
        assert!(!is_pandigital_str("1234567890"));
    }

    #[test]
    fn test_is_n_pandigital () {
        assert!(is_n_pandigital(123456789));
        assert!(is_n_pandigital(724159683));
        assert!(is_n_pandigital(987654321));

        assert!(is_n_pandigital(1));
        assert!(is_n_pandigital(12));
        assert!(is_n_pandigital(123));
        assert!(is_n_pandigital(1234));
        assert!(is_n_pandigital(12345));
        assert!(is_n_pandigital(123456));
        assert!(is_n_pandigital(1234567));
        assert!(is_n_pandigital(12345678));

        assert!(!is_n_pandigital(1234567890));
    }
}
