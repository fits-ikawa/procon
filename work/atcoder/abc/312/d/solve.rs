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
    input! {
        s: Chars,
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // i 文字目までで閉じてない括弧の数が j 個のときの通り数
    let mut dp = vec![vec![Mint::new(0); s.len() + 1]; s.len() + 1];
    dp[0][0] = Mint::new(1);

    for i in 1..=s.len() {
        let si = s[i - 1];

        for j in 0..=s.len() {
            if si == '(' || si == '?' {
                if j > 0 {
                    dp[i][j] = dp[i][j] + dp[i - 1][j - 1];
                }
            }

            if si == ')' || si == '?' {
                if j < s.len() {
                    dp[i][j] = dp[i][j] + dp[i - 1][j + 1];
                }
            }
        }
    }

    println!("{}", dp[s.len()][0]);
}
