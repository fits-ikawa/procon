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
        s: Chars,
    }

    use ac_library::ModInt1000000007 as Mint;

    let chokudai = "chokudai".chars().collect_vec();

    // dp[i][j]
    // 文字列 s の i 文字目までで chokudai の j 文字目までを作る通り数
    let mut dp = vec![vec![Mint::new(0); chokudai.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        dp[i][0] = Mint::new(1);
    }

    for i in 1..=s.len() {
        let si = s[i - 1];
        for j in 1..=chokudai.len() {
            let ci = chokudai[j - 1];
            // i 文字目を使わない（or 使えない）
            // i - 1 文字目までで chokudai の j 文字目まで作る通り数を引き継ぐ
            dp[i][j] = dp[i - 1][j];
            if si == ci {
                // i 文字目を使う
                // i - 1 文字目までで chokudai の j - 1 文字目まで作る通り数を足しこむ
                dp[i][j] = dp[i][j] + dp[i - 1][j - 1];
            }
        }
    }

    println!("{}", dp[s.len()][chokudai.len()]);
}
