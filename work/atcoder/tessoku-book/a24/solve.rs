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

    // dp[x]
    // 長さ x-1 の増加部分列の最後の要素として考えられる最小値
    let mut dp = vec![usize::MAX; n];
    dp[0] = a[0];

    for i in 1..n {
        let x = dp.lower_bound(&a[i]);
        dp[x] = a[i];
    }

    // usize::MAX ではない（＝未満の）値の個数が求める答え
    println!("{}", dp.lower_bound(&usize::MAX));
}
