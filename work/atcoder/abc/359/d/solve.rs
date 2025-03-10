#![allow(clippy::comparison_chain)]
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
    input! {
        n: usize, k: usize,
        s: Chars,
    }

    use ac_library::ModInt998244353 as Mint;

    let mut dp = hashmap! {};
    dp.insert(vec![], Mint::new(1));

    for si in s {
        let mut next_dp = hashmap! {};

        for (mut t, v) in dp {
            if si != 'B' {
                let mut t = t.clone();
                t.push('A');
                if t.len() < k {
                    next_dp.insert(t, v);
                } else if !is_parlindrome(&t) {
                    let value = next_dp.entry(t[1..].to_vec()).or_insert(Mint::new(0));
                    *value += v;
                }
            }
            if si != 'A' {
                t.push('B');
                if t.len() < k {
                    next_dp.insert(t, v);
                } else if !is_parlindrome(&t) {
                    let value = next_dp.entry(t[1..].to_vec()).or_insert(Mint::new(0));
                    *value += v;
                }
            }
        }

        dp = next_dp;
    }

    println!("{}", dp.values().sum::<Mint>());
}

fn is_parlindrome(s: &[char]) -> bool {
    let n = s.len();
    (0..n / 2).all(|i| s[i] == s[n - 1 - i])
}
