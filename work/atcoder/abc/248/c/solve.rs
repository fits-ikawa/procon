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
        n: usize, m: usize, k: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // 長さ i の数列で合計を j にできるものの個数
    let mut dp = vec![vec![Mint::new(0); k + 1]; n + 1];
    for j in 1..=k {
        dp[1][j] = Mint::new(if j <= m { 1 } else { 0 });
    }

    for i in 1..=n {
        for j in 1..=k {
            for ai in 1..=m {
                if j >= ai {
                    dp[i][j] = dp[i][j] + dp[i - 1][j - ai];
                }
            }
        }
    }

    println!("{}", dp[n].iter().sum::<Mint>());
}
