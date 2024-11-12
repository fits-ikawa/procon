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
        n: usize, k: usize,
        a: [usize; n],
    }

    // dp[i][j]
    // i 番目までのたこ焼きを自由に買ったときの j 番目に安い支払い金額
    let mut dp = vec![vec![usize::MAX; k + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][0] = 0;
    }

    for i in 1..=n {
        let ai = a[i - 1];
        let mut up_pos = 1;
        let mut cur_pos = 0;
        for j in 1..=k {
            match dp[i - 1][up_pos].cmp(&(dp[i][cur_pos] + ai)) {
                Equal => {
                    dp[i][j] = dp[i - 1][up_pos];
                    up_pos += 1;
                    cur_pos += 1;
                }
                Less => {
                    dp[i][j] = dp[i - 1][up_pos];
                    up_pos += 1;
                }
                Greater => {
                    dp[i][j] = dp[i][cur_pos] + ai;
                    cur_pos += 1;
                }
            }
        }
    }

    println!("{}", dp[n][k]);
}
