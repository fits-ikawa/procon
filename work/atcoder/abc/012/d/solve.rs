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
        n: usize, m: usize,
        abt: [(usize, usize, usize); m],
    }

    // dp[i][j]
    // バス停 i から j までの最小移動時間
    let mut dp = vec![vec![usize::MAX; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
    }

    for (a, b, t) in abt {
        dp[a - 1][b - 1] = t;
        dp[b - 1][a - 1] = t;
    }

    // 全点対最短路を求める
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
            }
        }
    }

    // 各バス停ごとに最も遠いバス停への時間を求め、その最小が答え
    let ans = dp.iter().map(|to| *to.iter().max().unwrap()).min().unwrap();

    println!("{}", ans);
}
