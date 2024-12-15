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
        n: usize,
        a: [Usize1; n-1],
    }

    let mut adj = vec![vec![]; n];

    for i in 0..n - 1 {
        adj[a[i]].push(i + 1);
    }

    // dp[i]
    // 社員 i の部下数
    let mut dp = vec![0; n];

    for i in (0..n).rev() {
        for &sub in &adj[i] {
            dp[i] += dp[sub] + 1;
        }
    }

    println!("{}", dp.iter().join(" "));
}
