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
        xy: [(usize, isize); n],
    }

    // dp[i][j]
    // i 品目まで選んで食べて状態が j (0: 普通, 1: 毒)のときの美味しさの総和の最大
    let mut dp = vec![vec![None; 2]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let (xi, yi) = xy[i - 1];
        if xi == 0 {
            // 解毒剤入り料理
            dp[i][0] = dp[i - 1][0] // 下げてもらって普通状態を維持
                .max(dp[i - 1][0].map(|x| x + yi)) // 普通の状態から食べてそのまま
                .max(dp[i - 1][1].map(|x| x + yi)); // 毒状態から食べて普通状態になる
            dp[i][1] = dp[i - 1][1]; // 毒状態を維持するには下げてもらうしかない
        } else {
            // 毒入り料理
            dp[i][0] = dp[i - 1][0]; // 普通状態を維持するには下げてもらうしかない
            dp[i][1] = dp[i - 1][1] // 下げてもらって毒状態を維持
                .max(dp[i - 1][0].map(|x| x + yi)); // 普通状態から食べて毒状態になる
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]).unwrap());
}
