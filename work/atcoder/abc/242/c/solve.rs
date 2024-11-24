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
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // i 桁の数字で、下一桁が j + 1 であるときの X の個数
    let mut dp = vec![vec![Mint::new(0); 9]; n + 1];
    dp[2] = [2, 3, 3, 3, 3, 3, 3, 3, 2]
        .into_iter()
        .map(Mint::new)
        .collect_vec();

    for i in 3..=n {
        for j in 0..9 {
            if j == 0 {
                // 下一桁が 1 の場合
                dp[i][j] = dp[i - 1][0] + dp[i - 1][1];
            } else if j == 8 {
                // 下一桁が 9 の場合
                dp[i][j] = dp[i - 1][7] + dp[i - 1][8];
            } else {
                // それ以外
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j] + dp[i - 1][j + 1];
            }
        }
    }

    println!("{}", dp[n].iter().sum::<Mint>());
}
