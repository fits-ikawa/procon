#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
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
        ng: [usize; 3],
    }

    let ng: Vec<_> = ng.into_iter().filter(|&x| x <= n).sorted().rev().collect();

    // dp[i]
    // 数 i に到達する最短手数
    // None は到達できない
    let mut dp = vec![None; n + 1];
    dp[n] = if ng.contains(&n) { None } else { Some(0) };

    for i in (1..=n).rev() {
        if dp[i].is_none() {
            continue;
        }
        for j in (i.saturating_sub(3)..=i - 1).rev() {
            if ng.contains(&j) {
                continue;
            }
            dp[j] = match dp[j] {
                Some(x) => dp[i].map(|y| x.min(y + 1)),
                None => dp[i].map(|y| y + 1),
            };
        }
    }

    println!(
        "{}",
        if dp[0].map_or(false, |x| x <= 100) {
            "YES"
        } else {
            "NO"
        }
    );
}
