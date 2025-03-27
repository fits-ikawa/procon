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
    // 解説 AC
    input! {
        t: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars,
        }

        let mut ans = Mint::new(1);

        for (i, &chr) in s[0..(n + 1) / 2].iter().rev().enumerate() {
            ans += Mint::new(chr as u8 - b'A') * Mint::new(26).pow(i as u64);
        }

        let mut t = vec!['A'; n];

        for i in 0..(n + 1) / 2 {
            t[i] = s[i];
            t[n - 1 - i] = s[i];
        }

        if t > s {
            ans -= 1;
        }

        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn solve() {
    input! {
        t: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars,
        }

        for i in 0..n / 2 {
            let j = n - 1 - i;

            if s[i] < s[j] {
                s[j] = s[i];
            } else if s[i] > s[j] {
                let mut k = 1;

                while s[j - k] == 'A' {
                    k += 1;
                }

                s[j - k] = (s[j - k] as u8 - 1) as char;

                for l in 1..k {
                    s[j - l] = 'Z';
                }

                s[j] = s[i];
            }
        }

        let mut ans = Mint::new(1);

        for i in n / 2..n {
            ans += Mint::new(s[i] as u8 - b'A') * Mint::new(26).pow((i - n / 2) as u64);
        }

        println!("{}", ans);
    }
}
