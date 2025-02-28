#![allow(clippy::comparison_chain)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    // 最大操作回数を考察して普通にダブリング
    input! {
        s: Chars,
    }

    const KLOG: usize = 17; // 100000 < 2^17 回移動すれば十分

    // ダブリング
    let mut dp = vec![vec![0; s.len()]; KLOG + 1];

    dp[0] = (0..s.len())
        .map(|j| if s[j] == 'L' { j - 1 } else { j + 1 })
        .collect_vec();

    for i in 1..=KLOG {
        for j in 0..s.len() {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = vec![0; s.len()];

    for j in 0..s.len() {
        ans[dp[KLOG][j]] += 1;
    }

    println!("{}", ans.iter().join(" "));
}

#[fastout]
fn solve() {
    // 思考停止で 10 倍ダブリング拳
    input! {
        s: Chars,
    }

    // ダブリング（10 倍単位でまとめる）
    let mut dp = vec![vec![0; s.len()]; 101];

    dp[0] = (0..s.len())
        .map(|j| if s[j] == 'L' { j - 1 } else { j + 1 })
        .collect_vec();

    for i in 1..=100 {
        for j in 0..s.len() {
            let mut cur = j;
            for _ in 0..10 {
                cur = dp[i - 1][cur];
            }
            dp[i][j] = cur;
        }
    }

    let mut ans = vec![0; s.len()];

    for j in 0..s.len() {
        ans[dp[100][j]] += 1;
    }

    println!("{}", ans.iter().join(" "));
}
