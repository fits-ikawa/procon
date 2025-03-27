#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize,
    }

    // dp[i][j]
    // i 回目に振ったダイスが j のときのスコア期待値
    let mut dp = vec![[0.0; 6]; n + 1];
    dp[n] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    for i in (0..n).rev() {
        let e = dp[i + 1].iter().sum::<f64>() / 6.0;

        for j in 1..=6 {
            dp[i][j - 1] = if (j as f64) < e { e } else { j as f64 };
        }
    }

    println!("{}", dp[1].iter().sum::<f64>() / 6.0);
}
