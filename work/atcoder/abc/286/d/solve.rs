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
    // i 種類目までの硬貨を使ってちょうど j 円にできるかどうか
    let mut dp = vec![vec![false; x + 1]; n + 1];

    for k in 0..=ab[0].1 {
        if ab[0].0 * k <= x {
            dp[1][ab[0].0 * k] = true;
        }
    }

    for i in 2..=n {
        let (a, b) = ab[i - 1];
        for j in 0..=x {
            if dp[i - 1][j] {
                for k in 0..=b {
                    if j + a * k <= x {
                        dp[i][j + a * k] = true;
                    }
                }
            }
        }
    }

    println!("{}", if dp[n][x] { "Yes" } else { "No" });
}
