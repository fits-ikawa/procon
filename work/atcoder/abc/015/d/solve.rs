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
        w: usize,
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    // dp[j][l]
    // スクリーンショット j 枚以下を使って幅を l 以下にする時の最大重要度
    let mut dp = vec![vec![0; w + 1]; k + 1];

    for &(ai, bi) in &ab {
        for j in (1..=k).rev() {
            for l in ai..=w {
                dp[j][l] = dp[j][l].max(dp[j - 1][l - ai] + bi);
            }
        }
    }

    println!("{}", dp[k][w]);
}

#[allow(dead_code)]
fn solve() {
    // dp 配列をコンパクトにしない
    input! {
        w: usize,
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    // dp[i][j][l]
    // i 番目までのスクリーンショットのうち
    // j 枚以下を使って幅を l 以下にする時の最大重要度
    let mut dp = vec![vec![vec![0; w + 1]; k + 1]; n + 1];

    for i in 1..=n {
        let (ai, bi) = ab[i - 1];
        for j in 1..=k {
            for l in 1..=w {
                dp[i][j][l] = dp[i - 1][j][l];

                if l >= ai {
                    dp[i][j][l] = dp[i][j][l].max(dp[i - 1][j - 1][l - ai] + bi);
                }
            }
        }
    }

    println!("{}", dp[n][k][w]);
}
