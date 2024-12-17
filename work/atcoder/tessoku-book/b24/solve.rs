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
        xy: [(usize, usize); n],
    }

    let ys = xy
        .iter()
        .sorted_by_key(|&&(x, y)| (x, Reverse(y)))
        .map(|&(_, y)| y)
        .collect_vec();

    // dp[x]
    // 長さ x-1 の増加部分列の最後の要素として考えられる最小値
    let mut dp = vec![usize::MAX; n];

    for i in 0..n {
        let pos = dp.lower_bound(&ys[i]);
        dp[pos] = ys[i];
    }

    println!("{}", dp.lower_bound(&usize::MAX));
}
