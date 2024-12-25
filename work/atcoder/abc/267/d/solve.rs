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
        n: usize, m: usize,
        a: [isize; n],
    }

    // dp[i][j]
    // a の i 番目までを要素とする数列で j 要素の部分列をとったときの最大値
    let mut dp = vec![vec![None; m + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let ai = a[i - 1];
        for j in 0..=m.min(i) {
            dp[i][j] = dp[i - 1][j];
            if 0 < j {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1].map(|e| e + ai * j as isize));
            }
        }
    }

    println!("{}", dp[n][m].unwrap());
}
