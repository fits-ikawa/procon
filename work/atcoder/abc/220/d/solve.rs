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
        a: [usize; n],
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // i 番目までの数字を消費したあと、先頭が j になっている通り数
    let mut dp = vec![vec![Mint::new(0); 10]; n + 1];
    // 1 番目の数字を消費した結果が a[0] であるとみなして初期化
    for j in 0..=9 {
        dp[1][j] = if j == a[0] {
            Mint::new(1)
        } else {
            Mint::new(0)
        };
    }

    for i in 2..=n {
        let ai = a[i - 1];
        for j in 0..=9 {
            let add = (ai + j) % 10;
            let mul = (ai * j) % 10;
            dp[i][add] = dp[i][add] + dp[i - 1][j];
            dp[i][mul] = dp[i][mul] + dp[i - 1][j];
        }
    }

    println!("{}", dp[n].iter().join("\n"));
}
