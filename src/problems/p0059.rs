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
    pub fn f () -> Result<(), Box<dyn Error>> {
        println!("\niterative method: {:?}\n", iterative());
        Ok(())
    }

    f().expect("Problem 59 failed with an error");
}

///
/// Try each password p progressively in length
/// from one character long to the length of the
/// buffer, and each character in p iterates over
/// the ASCII interval 33..=126 (b'!'..=b'~'),
///
///    Test the password against the cypher by
///    measuring a heuristic
///
///    Select as the password the one that
///    maximizes the heuristic score.
///
/// Heuristic = common words appear
///
fn iterative () -> Result<u64, Box<dyn Error>> {
    let data = import_data()?;

    let mut shannon_entropy = [0u64; 256];
    let mut passwords = Password::new(data.len());

    while let Some(p) = passwords.next() {
        let a = decode(&p, &data);
        if is_english_text(&a, &mut shannon_entropy) {
            return Ok(a.iter().map(|&x| x as u64).sum())
        }
    }

    Ok(0)
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

fn decode (password: &[u8], data: &[u8]) -> Vec<u8> {
    let mut r = data.to_vec();
    for (a, b) in r.iter_mut().zip(password.iter().cycle()) {
        *a ^= b;
    }
    r
}

fn is_english_text (data: &[u8], entropy: &mut [u64]) -> bool {
    let len = data.len() as f64;

    // Check for null values
    if data.iter().any(|&x| x == 0) { return false; }

    // Percentage of characters in the range 32 to 126
    let a = data.iter().filter(|&&x| 32 <= x && x <= 126).count() as f64;
    if a / len < 0.95 { return false; }

    // Shannon Entropy
    let mut b = 0.0;
    entropy.fill(0);
    for &x in data { entropy[x as usize] += 1; }
    for &mut x in entropy {
        if x > 0 {
            let probability = (x as f64) / len;
            b -= probability * f64::log2(probability);
        }
    }
    if b > 5.0 { return false; }

    // Bigrams
    if ((data.windows(2).filter(|&x| x == [b't', b'h']).count() as f64) / len) < 0.01 { return false; }
    if ((data.windows(2).filter(|&x| x == [b'h', b'e']).count() as f64) / len) < 0.01 { return false; }
    if ((data.windows(2).filter(|&x| x == [b'i', b'n']).count() as f64) / len) < 0.01 { return false; }
    if ((data.windows(2).filter(|&x| x == [b'e', b'r']).count() as f64) / len) < 0.01 { return false; }
    if ((data.windows(2).filter(|&x| x == [b'a', b'n']).count() as f64) / len) < 0.01 { return false; }
    if ((data.windows(2).filter(|&x| x == [b'r', b'e']).count() as f64) / len) < 0.01 { return false; }
    if ((data.windows(2).filter(|&x| x == [b'o', b'n']).count() as f64) / len) < 0.01 { return false; }

    true
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

pub struct Password { cur: Vec<u8>, view: Vec<u8>, limit: usize, done: bool }
impl Password {
    pub const MIN: u8 = 32; // space
    pub const MAX: u8 = 126; // ~

    pub fn new (limit: usize) -> Self {
        let mut cur = Vec::with_capacity(limit);
        cur.push(Self::MIN);
        let view = Vec::with_capacity(limit);
        Self { cur, view, limit, done: false }
    }

    fn next (&mut self) -> Option<&[u8]> {
        if self.done { return None; }

        self.view.clear();
        self.view.extend_from_slice(&self.cur);

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
        if !incremented {
            if len < self.limit {
                self.cur.resize(len + 1, Self::MIN);
            }
            else { self.done = true; }
        }

        Some(&self.view)
    }
}
