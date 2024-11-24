#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise_map;
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
        n: usize, k: usize,
        a: [usize; n],
        b: [usize; n],
    }

    // dp[i][j]
    // i - 1 番目に j (0: A, 1: B) の数字を選べるか？
    let mut dp = vec![vec![false; 2]; n];
    dp[0][0] = true;
    dp[0][1] = true;

    for i in 1..n {
        // 前の数字が選べて、差が K 以下なら選べる
        dp[i][0] = (dp[i - 1][0] && a[i - 1].abs_diff(a[i]) <= k)
            || (dp[i - 1][1] && b[i - 1].abs_diff(a[i]) <= k);
        dp[i][1] = (dp[i - 1][0] && a[i - 1].abs_diff(b[i]) <= k)
            || (dp[i - 1][1] && b[i - 1].abs_diff(b[i]) <= k);
    }

    println!(
        "{}",
        if dp[n - 1][0] || dp[n - 1][1] {
            "Yes"
        } else {
            "No"
        }
    );
}

#[allow(dead_code)]
fn solve() {
    // メモ化再帰で解く
    input! {
        n: usize, k: usize,
        ab: [[usize; n]; 2],
    }

    let ans = dfs(0, 0, n, k, &ab) || dfs(1, 0, n, k, &ab);

    println!("{}", if ans { "Yes" } else { "No" });
}

#[allow(dead_code)]
#[memoise_map(i, j)]
fn dfs(i: usize, j: usize, n: usize, k: usize, ab: &[Vec<usize>]) -> bool {
    // i (0: A, 1: B) の j 番目の数字を選べるか？
    if j >= n - 1 {
        // 最後の数字は必ず選べる
        return true;
    }

    let mut ret = false;

    // 次の数字との差が K 以内ならその先を選べるか調べる
    if ab[i][j].abs_diff(ab[0][j + 1]) <= k {
        ret |= dfs(0, j + 1, n, k, ab);
    }
    if ab[i][j].abs_diff(ab[1][j + 1]) <= k {
        ret |= dfs(1, j + 1, n, k, ab);
    }

    ret
}
