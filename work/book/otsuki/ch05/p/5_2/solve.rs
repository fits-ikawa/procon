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
        n: usize, w: usize,
        nums: [usize; n],
    }

    let mut dp = vec![vec![false; w + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let num = nums[i - 1];
        for j in 0..num {
            dp[i][j] = dp[i - 1][j];
        }
        for j in num..=w {
            dp[i][j] = dp[i - 1][j] || dp[i - 1][j - num];
        }
    }

    println!("{}", if dp[n][w] { "YES" } else { "NO" });
}
