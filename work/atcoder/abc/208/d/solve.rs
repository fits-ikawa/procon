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
        abc: [(Usize1, Usize1, usize); m],
    }

    // ワーシャルフロイド法
    let mut dp = vec![vec![usize::MAX; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
    }

    for (a, b, c) in abc {
        dp[a][b] = c;
    }

    let mut ans = 0;

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
                if dp[i][j] != usize::MAX {
                    ans += dp[i][j];
                }
            }
        }
    }

    println!("{}", ans);
}
