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
        t: [usize; n],
    }

    let m = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let ti = t[i - 1];
        for j in 0..=m {
            dp[i][j] = dp[i - 1][j];
            if j >= ti {
                dp[i][j] = dp[i][j] || dp[i - 1][j - ti];
            }
        }
    }

    let ans = (1..=m)
        .filter_map(|i| {
            if dp[n][i] {
                Some((i.abs_diff(m - i), i.max(m - i)))
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .1;

    println!("{}", ans);
}
