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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, x: usize,
        ab: [(usize, usize); n],
    }

    // dp[i][j]
    // i 番目のジャンプ後に j の位置にいることができるか
    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let (a, b) = ab[i - 1];
        for j in 1..=x {
            if (j >= a && dp[i - 1][j - a]) || (j >= b && dp[i - 1][j - b]) {
                dp[i][j] = true;
            }
        }
    }

    println!("{}", if dp[n][x] { "Yes" } else { "No" });
}
