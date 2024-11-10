#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        for j in i + 1..=(i + 2).min(n - 1) {
            dp[j] = dp[j].min(dp[i] + a[i].abs_diff(a[j]));
        }
    }

    println!("{}", dp[n - 1]);
}
