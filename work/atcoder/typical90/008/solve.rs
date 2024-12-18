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
        s: Chars,
    }

    use ac_library::ModInt1000000007 as Mint;

    let t = "atcoder".chars().collect_vec();

    // dp[i][j]
    // 文字列 s の i 文字目までで atcoder の j 文字目までを作る通り数
    let mut dp = vec![vec![Mint::new(0); t.len() + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = Mint::new(1);
    }

    for i in 1..=n {
        let si = s[i - 1];
        for j in 1..=t.len() {
            dp[i][j] = dp[i - 1][j];
            if si == t[j - 1] {
                dp[i][j] = dp[i][j] + dp[i - 1][j - 1];
            }
        }
    }

    println!("{}", dp[n][t.len()]);
}
