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
        n: usize, k: usize, w:usize,
        nums: [usize; n],
    }

    // dp[i][j]
    // i 番目の数字まで使って総和を j にするときの最小個数
    let mut dp = vec![vec![usize::MAX; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        let num = nums[i - 1];
        for j in 0..num {
            dp[i][j] = dp[i - 1][j];
        }
        for j in num..=w {
            dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - num].saturating_add(1));
        }
    }

    println!("{}", if dp[n][w] <= k { "YES" } else { "NO" });
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, k: usize, w:usize,
        nums: [usize; n],
    }

    // dp[i][j]
    // i 番目の数字まで使って総和を j にするときの最小個数
    // None のとき総和は作れない
    let mut dp = vec![vec![None; w + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let num = nums[i - 1];
        for j in 0..num {
            dp[i][j] = dp[i - 1][j];
        }
        for j in num..=w {
            // num を選ばない
            dp[i][j] = dp[i - 1][j];

            // num を選ぶ。ただし i - 1 個目までで 総和 - num を作れている場合のみ
            if let Some(min_n) = dp[i - 1][j - num] {
                let new_min = min_n + 1;
                dp[i][j] = Some(dp[i][j].map_or(new_min, |x| x.min(new_min)));
            }
        }
    }

    if let Some(min_n) = dp[n][w] {
        println!("{}", if min_n <= k { "YES" } else { "NO" });
    } else {
        println!("NO");
    }
}
