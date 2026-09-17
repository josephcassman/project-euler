// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

//!
//! https://projecteuler.net/problem=59
//!
//! XOR Decryption
//!
//! Each character on a computer is assigned a unique code and the preferred standard
//! is ASCII (American Standard Code for Information Interchange). For example,
//! uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.
//!
//! A modern encryption method is to take a text file, convert the bytes to ASCII,
//! then XOR each byte with a given value, taken from a secret key. The advantage with
//! the XOR function is that using the same encryption key on the cipher text, restores
//! the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.
//!
//! For unbreakable encryption, the key is the same length as the plain text message,
//! and the key is made up of random bytes. The user would keep the encrypted message
//! and the encryption key in different locations, and without both "halves",
//! it is impossible to decrypt the message.
//!
//! Unfortunately, this method is impractical for most users, so the modified method
//! is to use a password as a key. If the password is shorter than the message, which
//! is likely, the key is repeated cyclically throughout the message. The balance for
//! this method is using a sufficiently long password key for security, but
//! short enough to be memorable.
//!
//! Your task has been made easy, as the encryption key
//! consists of three lower case characters.
//!
//! Using 0059_cipher.txt (right click and 'Save Link/Target As...'),
//! a file containing the encrypted ASCII codes, and the knowledge that the plain text
//! must contain common English words, decrypt the message and find the sum of
//! the ASCII values in the original text.
//!

use std::error::Error;
use std::fs;
use std::num::ParseIntError;

pub fn run () {
    fn f () -> Result<(), Box<dyn Error>> {
        println!("\niterative method: {:?}\n", iterative());
        Ok(())
    }

    f().expect("Problem 59 failed with an error");
}

fn iterative () -> Result<u64, Box<dyn Error>> {
    let data = import_data()?;
    let mut buf = vec![0u8; data.len()];
    let mut min_score = f64::INFINITY;
    let mut r = Vec::new();

    let mut passwords = Password::new();
    while let Some(p) = passwords.next() {
        decode(p, &data, &mut buf);
        let a = chi_squared(&buf);
        if a < min_score {
            min_score = a;
            r = buf.to_vec();
        }
    }

    Ok(r.iter().map(|&x| x as u64).sum())
}

fn import_data () -> Result<Vec<u8>, Box<dyn Error>> {
    let path = "data/0059_cipher.txt";
    let r = fs::read_to_string(path)?
        .split(',')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<u8>, ParseIntError>>()?;
    Ok(r)
}

fn decode (password: &[u8], data: &[u8], output: &mut [u8]) {
    for (x, (&a, &b)) in output.iter_mut().zip(data.iter().zip(password.iter().cycle())) {
        *x = a ^ b;
    }
}

const LETTER_FREQUENCY: [f64; 26] = [
    0.08167, // A ≈  8.167%
    0.01492, // B ≈  1.492%
    0.02782, // C ≈  2.782%
    0.04253, // D ≈  4.253%
    0.12702, // E ≈ 12.702%
    0.02228, // F ≈  2.228%
    0.02015, // G ≈  2.015%
    0.06094, // H ≈  6.094%
    0.06966, // I ≈  6.966%
    0.00153, // J ≈  0.153%
    0.00772, // K ≈  0.772%
    0.04025, // L ≈  4.025%
    0.02406, // M ≈  2.406%
    0.06749, // N ≈  6.749%
    0.07507, // O ≈  7.507%
    0.01929, // P ≈  1.929%
    0.00095, // Q ≈  0.095%
    0.05987, // R ≈  5.987%
    0.06327, // S ≈  6.327%
    0.09056, // T ≈  9.056%
    0.02758, // U ≈  2.758%
    0.00978, // V ≈  0.978%
    0.02360, // W ≈  2.360%
    0.00150, // X ≈  0.150%
    0.01974, // Y ≈  1.974%
    0.00074, // Z ≈  0.074%
];

fn chi_squared (data: &[u8]) -> f64 {
    let mut counts = [0u32; 26];
    let mut total = 0usize;

    for &a in data {
        if !a.is_ascii_graphic() && !a.is_ascii_whitespace() { return f64::INFINITY; }
        if a.is_ascii_alphabetic() {
            let i = (a.to_ascii_lowercase() - b'a') as usize;
            counts[i] += 1;
            total += 1;
        }
    }

    // natural text tends to be > 70% alphabetic
    if total < (data.len() * 6 / 10) { return f64::INFINITY; }

    let n = total as f64;
    let mut r = 0.0;

    for i in 0..26 {
        let expected = n * LETTER_FREQUENCY[i];
        let actual = counts[i] as f64;
        let delta = actual - expected;
        r += (delta * delta) / expected;
    }

    r
}

///
/// ASCII
///
/// 0   NUL    26 SUB    52  4    78   N    104  h
/// 1   SOH    27 ESC    53  5    79   O    105  i
/// 2   STX    28 FS     54  6    80   P    106  j
/// 3   ETX    29 GS     55  7    81   Q    107  k
/// 4   EOT    30 RS     56  8    82   R    108  l
/// 5   ENQ    31 US     57  9    83   S    109  m
/// 6   ACK    32        58  :    84   T    110  n
/// 7   BEL    33 !      59  ;    85   U    111  o
/// 8   BS     34 "      60  <    86   V    112  p
/// 9   HT     35 #      61  =    87   W    113  q
/// 10  LF     36 $      62  >    88   X    114  r
/// 11  VT     37 %      63  ?    89   Y    115  s
/// 12  FF     38 &      64  @    90   Z    116  t
/// 13  CR     39 '      65  A    91   [    117  u
/// 14  SO     40 (      66  B    92   \    118  v
/// 15  SI     41 )      67  C    93   ]    119  w
/// 16  DLE    42 *      68  D    94   ^    120  x
/// 17  DC1    43 +      69  E    95   _    121  y
/// 18  DC2    44 ,      70  F    96   `    122  z
/// 19  DC3    45 -      71  G    97   a    123  {
/// 20  DC4    46 .      72  H    98   b    124  |
/// 21  NAK    47 /      73  I    99   c    125  }
/// 22  SYN    48 0      74  J    100  d    126  ~
/// 23  ETB    49 1      75  K    101  e    127  DEL
/// 24  CAN    50 2      76  L    102  f
/// 25  EM     51 3      77  M    103  g
///

pub struct Password { cur: Vec<u8>, view: Vec<u8>, done: bool }
impl Password {
    pub const MIN: u8 = 97;  // a
    pub const MAX: u8 = 122; // z

    pub fn new () -> Self {
        let cur = vec![Self::MIN; 3];
        let view = vec![0u8; 3];
        Self { cur, view, done: false }
    }

    fn next (&mut self) -> Option<&[u8]> {
        if self.done { return None; }

        self.view.copy_from_slice(&self.cur);

        let len = self.cur.len();
        let mut incremented = false;
        let mut i = len;
        while i > 0 {
            i -= 1;
            if self.cur[i] < Self::MAX {
                self.cur[i] += 1;
                for j in (i + 1)..len {
                    self.cur[j] = Self::MIN;
                }
                incremented = true;
                break;
            }
        }

        // increase the length if all characters == Self::MAX
        if !incremented { self.done = true; }

        Some(&self.view)
    }
}
