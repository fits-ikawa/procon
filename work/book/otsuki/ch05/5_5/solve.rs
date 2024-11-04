#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![usize::MAX; t.len() + 1]; s.len() + 1];
    dp[0][0] = 0;

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            if i > 0 && j > 0 {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 1);
                }
            }

            if i > 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }

            if j > 0 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
