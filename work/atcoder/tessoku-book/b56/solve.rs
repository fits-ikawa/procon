#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use rand::Rng;
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }

    let s_rev = s.iter().rev().copied().collect_vec();

    let base = rand::thread_rng().gen_range(256..1 << 32);
    let rh = mylib::RollingHash::from_chars(&s, base);
    let rh_rev = mylib::RollingHash::from_chars(&s_rev, base);

    for (l, r) in lr {
        if rh.get_hash(l - 1, r) == rh_rev.get_hash(n - r, n + 1 - l) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

pub mod mylib {
    /// A rolling hash implementation for sequences of numbers.
    ///
    /// This implementation uses a modulo of 2^61 - 1 to reduce hash collisions.
    /// For strings, use the `from_chars` constructor, which converts ASCII characters to u64.
    pub struct RollingHash {
        hash: Vec<u64>,
        power: Vec<u64>,
        length: usize,
    }

    impl RollingHash {
        const MOD: u64 = (1 << 61) - 1;
        const MASK30: u64 = (1 << 30) - 1;
        const MASK31: u64 = (1 << 31) - 1;
        const MASK61: u64 = Self::MOD;
        const POSITIVISER: u64 = Self::MOD * 4;

        /// Multiplies two numbers in a way that avoids overflow under modulo 2^61 - 1 arithmetic.
        ///
        /// **Note:** The result is not reduced modulo 2^61 - 1. To obtain the final value
        /// within the correct range, apply `calc_mod` on the result.
        fn mul(a: u64, b: u64) -> u64 {
            let au = a >> 31;
            let ad = a & Self::MASK31;
            let bu = b >> 31;
            let bd = b & Self::MASK31;
            let mid = ad * bu + au * bd;
            let midu = mid >> 30;
            let midd = mid & Self::MASK30;

            au * bu * 2 + midu + (midd << 31) + ad * bd
        }

        /// Reduces the given value modulo 2^61 - 1.
        fn calc_mod(x: u64) -> u64 {
            let xu = x >> 61;
            let xd = x & Self::MASK61;
            let ret = xu + xd;

            if ret >= Self::MOD {
                ret - Self::MOD
            } else {
                ret
            }
        }

        /// Creates a new `RollingHash` for a sequence of numbers.
        ///
        /// # Arguments
        ///
        /// * `seq` - A slice of `u64` representing the sequence.
        /// * `base` - The base used for the polynomial rolling hash.
        pub fn new(seq: &[u64], base: u64) -> Self {
            let length = seq.len();
            let mut hash = vec![0; length + 1];
            let mut power = vec![1; length + 1];

            for (i, &val) in seq.iter().enumerate() {
                hash[i + 1] = Self::calc_mod(Self::mul(hash[i], base) + val);
                power[i + 1] = Self::calc_mod(Self::mul(power[i], base));
            }

            Self {
                hash,
                power,
                length,
            }
        }

        /// Creates a new `RollingHash` for an ASCII string provided as a slice of `char`.
        ///
        /// # Arguments
        ///
        /// * `s` - A slice of `char` representing an ASCII string.
        /// * `base` - The base used for the polynomial rolling hash.
        pub fn from_chars(s: &[char], base: u64) -> Self {
            let length = s.len();
            let mut hash = vec![0; length + 1];
            let mut power = vec![1; length + 1];

            for (i, &ch) in s.iter().enumerate() {
                let c = ch as u64;
                hash[i + 1] = Self::calc_mod(Self::mul(hash[i], base) + c);
                power[i + 1] = Self::calc_mod(Self::mul(power[i], base));
            }

            Self {
                hash,
                power,
                length,
            }
        }

        /// Returns the hash value for the substring `s[l..r]`.
        ///
        /// The hash of the substring is computed in O(1) time using the precomputed values.
        ///
        /// # Arguments
        ///
        /// * `l` - The starting index (inclusive).
        /// * `r` - The ending index (exclusive).
        ///
        /// # Returns
        ///
        /// The hash value of the substring.
        pub fn get_hash(&self, l: usize, r: usize) -> u64 {
            Self::calc_mod(
                self.hash[r] + Self::POSITIVISER - Self::mul(self.hash[l], self.power[r - l]),
            )
        }

        /// Returns the length of the input string.
        pub fn len(&self) -> usize {
            self.length
        }

        /// Returns `true` if the input string is empty.
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
    }
}
