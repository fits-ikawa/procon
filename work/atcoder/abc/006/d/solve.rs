#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }

    // dp[i]
    // 長さが i+1 になるような部分増加列の最後の要素の最小値
    let mut dp = vec![usize::MAX; n];

    for ci in c {
        let j = dp.lower_bound(&ci);
        dp[j] = ci;
    }

    println!("{}", n - dp.lower_bound(&usize::MAX));
}
